//! Flattens N PDF members into one contiguous, device-upload-ready layout.
//!
//! This module is pure Rust and depends only on [`neopdf::gpu_export`] — no GPU
//! toolchain is needed to build or test it. It is the place almost all of this
//! crate's correctness risk lives: once [`DeviceGridLayout`] is right, the device
//! kernels (a later phase) are close to a transliteration of its indexing.
//!
//! LHAPDF-style sets store the same `(x, Q²)` grid (across one or more Q²-contiguous
//! subgrids — see [`neopdf::gpu_export`] for why there can be more than one) and the
//! same flavor PIDs for every member of a set — only the tabulated values (and often
//! the `alpha_s` table) differ by member. [`DeviceGridLayout::build`] checks that
//! assumption explicitly rather than relying on it silently.

use neopdf::gpu_export::{
    AlphaSValues, CollinearExport, CollinearExportError, CollinearSubgrid, CollinearValues,
};
use neopdf::pdf::PDF;
use thiserror::Error;

/// Why [`DeviceGridLayout::build`] could not flatten a set of members.
#[derive(Debug, Error)]
pub enum GridPrepError {
    /// No members were supplied.
    #[error("no members supplied")]
    NoMembers,
    /// One member's [`PDF::collinear_export`] itself failed.
    #[error("member {member}'s collinear export failed")]
    Export {
        /// The member's index in the input slice.
        member: usize,
        /// Why that member's export failed.
        #[source]
        source: CollinearExportError,
    },
    /// A member's x grid differs from member 0's.
    #[error("member {member}'s x grid differs from member 0's")]
    XGridMismatch {
        /// The member's index in the input slice.
        member: usize,
    },
    /// A member's flavor PIDs differ from member 0's.
    #[error("member {member}'s flavor PIDs differ from member 0's")]
    PidMismatch {
        /// The member's index in the input slice.
        member: usize,
    },
    /// A member has a different number of subgrids than member 0.
    #[error(
        "member {member} has {found} subgrids, but member 0 has {expected}; \
         a device grid requires every member to split into the same subgrids"
    )]
    SubgridCountMismatch {
        /// The member's index in the input slice.
        member: usize,
        /// Member 0's subgrid count.
        expected: usize,
        /// This member's subgrid count.
        found: usize,
    },
    /// Subgrid `subgrid` of member `member` has a different Q² grid than
    /// subgrid `subgrid` of member 0.
    #[error("member {member}'s subgrid {subgrid} has a different Q² grid than member 0's")]
    QGridMismatch {
        /// The member's index in the input slice.
        member: usize,
        /// The subgrid's index within that member.
        subgrid: usize,
    },
    /// Subgrid `subgrid` of member `member` uses a different fast-path
    /// representation than subgrid `subgrid` of member 0 (one is `LogBicubic`,
    /// the other `LogChebyshev`).
    #[error(
        "member {member}'s subgrid {subgrid} is {found}, but member 0's is {expected}; \
         a device grid requires every member to use the same fast path"
    )]
    ValueKindMismatch {
        /// The member's index in the input slice.
        member: usize,
        /// The subgrid's index within that member.
        subgrid: usize,
        /// Member 0's fast-path kind for this subgrid.
        expected: &'static str,
        /// This member's fast-path kind for this subgrid.
        found: &'static str,
    },
    /// A member's `alpha_s` data is a different kind than member 0's (one
    /// member has a table, another a single constant value).
    #[error(
        "member {member}'s alpha_s data is {found}, but member 0's is {expected}; \
         a device grid requires every member to use the same kind"
    )]
    AlphaSKindMismatch {
        /// The member's index in the input slice.
        member: usize,
        /// Member 0's `alpha_s` kind.
        expected: &'static str,
        /// This member's `alpha_s` kind.
        found: &'static str,
    },
}

/// The all-flavor value table across every member for one subgrid, flattened
/// member-major.
pub enum ValuesLayout {
    /// Interleaved cubic Hermite coefficients. `coeffs[m * member_stride..][..member_stride]`
    /// is member `m`'s own [`CollinearValues::Hermite`] array for this subgrid, unchanged.
    Hermite {
        /// Every member's coefficients, concatenated in member order.
        coeffs: Vec<f64>,
        /// Length of one member's slice within `coeffs`.
        member_stride: usize,
    },
    /// Barycentric Chebyshev values. `values[flavor][m * member_stride..][..member_stride]`
    /// is member `m`'s own per-flavor array for this subgrid, unchanged.
    Chebyshev {
        /// Per-flavor arrays, each the concatenation of every member's values
        /// for that flavor and this subgrid, in member order.
        values: Vec<Vec<f64>>,
        /// Length of one member's slice within each per-flavor array.
        member_stride: usize,
    },
}

/// `alpha_s(Q²)` data across every member.
pub enum AlphaSLayout {
    /// One Q-independent value per member.
    Constant {
        /// `values[m]` is member `m`'s constant `alpha_s`.
        values: Vec<f64>,
    },
    /// A per-member table, concatenated. Member `m`'s knots/values are
    /// `log_q2s[offsets[m]..offsets[m + 1]]` / `alphas[offsets[m]..offsets[m + 1]]`.
    Cubic {
        /// Every member's log-Q² knots, concatenated in member order.
        log_q2s: Vec<f64>,
        /// Every member's `alpha_s` values, concatenated in member order.
        alphas: Vec<f64>,
        /// Length `n_members + 1`; `offsets[m]..offsets[m + 1]` bounds member `m`.
        offsets: Vec<usize>,
    },
}

/// One Q²-contiguous piece of a [`DeviceGridLayout`], across every member.
pub struct SubgridLayout {
    /// Log-transformed Q² grid knots for this subgrid, shared by every member.
    pub log_q2s: Vec<f64>,
    /// The all-flavor value table for this subgrid, across every member.
    pub values: ValuesLayout,
}

/// A device-upload-ready layout for `n_members` members of one collinear PDF set.
pub struct DeviceGridLayout {
    /// Log-transformed x grid knots, shared by every member and every subgrid.
    pub log_xs: Vec<f64>,
    /// Flavor PIDs, shared by every member.
    pub pids: Vec<i32>,
    /// Number of members flattened into this layout.
    pub n_members: usize,
    /// Subgrids, ordered by increasing, contiguous Q² range (see
    /// [`neopdf::gpu_export::CollinearExport`] for why there can be more than one).
    pub subgrids: Vec<SubgridLayout>,
    /// The `alpha_s(Q²)` data.
    pub alphas: AlphaSLayout,
}

impl DeviceGridLayout {
    /// Number of flavors, i.e. `self.pids.len()`.
    #[must_use]
    pub fn n_flavors(&self) -> usize {
        self.pids.len()
    }

    /// Flattens `members` (same PDF set, every member) into one device-ready layout.
    ///
    /// Every member must share the same x grid, the same subgrid Q² ranges, the
    /// same flavor PIDs, and the same fast-path/`alpha_s` kind as member 0 — see
    /// [`GridPrepError`] for what's checked and why.
    ///
    /// # Errors
    ///
    /// Returns an error if `members` is empty, if any member's
    /// [`PDF::collinear_export`] fails, or if the members disagree on any of the
    /// grids/PIDs/kinds above. See [`GridPrepError`] for each case.
    pub fn build(members: &[PDF]) -> Result<Self, GridPrepError> {
        if members.is_empty() {
            return Err(GridPrepError::NoMembers);
        }

        let exports: Vec<CollinearExport> = members
            .iter()
            .enumerate()
            .map(|(member, pdf)| {
                pdf.collinear_export()
                    .map_err(|source| GridPrepError::Export { member, source })
            })
            .collect::<Result<_, _>>()?;

        let first = &exports[0];
        let n_subgrids = first.subgrids.len();
        for (member, export) in exports.iter().enumerate().skip(1) {
            if export.log_xs != first.log_xs {
                return Err(GridPrepError::XGridMismatch { member });
            }
            if export.pids != first.pids {
                return Err(GridPrepError::PidMismatch { member });
            }
            if export.subgrids.len() != n_subgrids {
                return Err(GridPrepError::SubgridCountMismatch {
                    member,
                    expected: n_subgrids,
                    found: export.subgrids.len(),
                });
            }
        }

        let log_xs = first.log_xs.clone();
        let pids = first.pids.clone();
        let n_members = exports.len();

        let subgrids = (0..n_subgrids)
            .map(|subgrid| Self::flatten_subgrid(&exports, subgrid))
            .collect::<Result<_, _>>()?;
        let alphas = Self::flatten_alphas(&exports)?;

        Ok(Self {
            log_xs,
            pids,
            n_members,
            subgrids,
            alphas,
        })
    }

    fn flatten_subgrid(
        exports: &[CollinearExport],
        subgrid: usize,
    ) -> Result<SubgridLayout, GridPrepError> {
        let first = &exports[0].subgrids[subgrid];
        let log_q2s = first.log_q2s.clone();

        let values = match &first.values {
            CollinearValues::Hermite { coeffs } => {
                let member_stride = coeffs.len();
                let mut all = Vec::with_capacity(member_stride * exports.len());
                for (member, export) in exports.iter().enumerate() {
                    let this: &CollinearSubgrid = &export.subgrids[subgrid];
                    if this.log_q2s != log_q2s {
                        return Err(GridPrepError::QGridMismatch { member, subgrid });
                    }
                    match &this.values {
                        CollinearValues::Hermite { coeffs } if coeffs.len() == member_stride => {
                            all.extend_from_slice(coeffs);
                        }
                        CollinearValues::Hermite { .. } => {
                            return Err(GridPrepError::QGridMismatch { member, subgrid });
                        }
                        CollinearValues::Chebyshev { .. } => {
                            return Err(GridPrepError::ValueKindMismatch {
                                member,
                                subgrid,
                                expected: "hermite",
                                found: "chebyshev",
                            });
                        }
                    }
                }
                ValuesLayout::Hermite {
                    coeffs: all,
                    member_stride,
                }
            }
            CollinearValues::Chebyshev { values } => {
                let n_flavors = values.len();
                let member_stride = values.first().map_or(0, Vec::len);
                let mut all: Vec<Vec<f64>> = (0..n_flavors)
                    .map(|_| Vec::with_capacity(member_stride * exports.len()))
                    .collect();
                for (member, export) in exports.iter().enumerate() {
                    let this: &CollinearSubgrid = &export.subgrids[subgrid];
                    if this.log_q2s != log_q2s {
                        return Err(GridPrepError::QGridMismatch { member, subgrid });
                    }
                    match &this.values {
                        CollinearValues::Chebyshev { values } if values.len() == n_flavors => {
                            for (flavor, per_member) in values.iter().enumerate() {
                                if per_member.len() != member_stride {
                                    return Err(GridPrepError::QGridMismatch { member, subgrid });
                                }
                                all[flavor].extend_from_slice(per_member);
                            }
                        }
                        CollinearValues::Chebyshev { .. } => {
                            return Err(GridPrepError::QGridMismatch { member, subgrid });
                        }
                        CollinearValues::Hermite { .. } => {
                            return Err(GridPrepError::ValueKindMismatch {
                                member,
                                subgrid,
                                expected: "chebyshev",
                                found: "hermite",
                            });
                        }
                    }
                }
                ValuesLayout::Chebyshev {
                    values: all,
                    member_stride,
                }
            }
        };

        Ok(SubgridLayout { log_q2s, values })
    }

    fn flatten_alphas(exports: &[CollinearExport]) -> Result<AlphaSLayout, GridPrepError> {
        match &exports[0].alphas {
            AlphaSValues::Constant(_) => {
                let mut values = Vec::with_capacity(exports.len());
                for (member, export) in exports.iter().enumerate() {
                    match &export.alphas {
                        AlphaSValues::Constant(v) => values.push(*v),
                        AlphaSValues::Cubic { .. } => {
                            return Err(GridPrepError::AlphaSKindMismatch {
                                member,
                                expected: "constant",
                                found: "cubic",
                            });
                        }
                    }
                }
                Ok(AlphaSLayout::Constant { values })
            }
            AlphaSValues::Cubic { .. } => {
                let mut log_q2s_all = Vec::new();
                let mut alphas_all = Vec::new();
                let mut offsets = Vec::with_capacity(exports.len() + 1);
                offsets.push(0usize);
                for (member, export) in exports.iter().enumerate() {
                    match &export.alphas {
                        AlphaSValues::Cubic { log_q2s, alphas } => {
                            log_q2s_all.extend_from_slice(log_q2s);
                            alphas_all.extend_from_slice(alphas);
                            offsets.push(log_q2s_all.len());
                        }
                        AlphaSValues::Constant(_) => {
                            return Err(GridPrepError::AlphaSKindMismatch {
                                member,
                                expected: "cubic",
                                found: "constant",
                            });
                        }
                    }
                }
                Ok(AlphaSLayout::Cubic {
                    log_q2s: log_q2s_all,
                    alphas: alphas_all,
                    offsets,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_members(name: &str) -> Vec<PDF> {
        PDF::load_pdfs(name)
    }

    #[test]
    fn flattens_nnpdf40_members_with_shared_grid_and_pids() {
        let members = load_members("NNPDF40_nnlo_as_01180");
        let n_members = members.len();
        let layout = DeviceGridLayout::build(&members).unwrap();

        assert_eq!(layout.n_members, n_members);
        assert_eq!(layout.pids, members[0].pids().to_vec());
        assert!(layout.log_xs.windows(2).all(|w| w[0] < w[1]));
        // NNPDF40_nnlo_as_01180 splits into 2 Q²-contiguous subgrids at the
        // bottom-quark threshold -- this is the normal LHAPDF layout, not an
        // edge case, so a 1-subgrid-only export would reject almost every real
        // proton PDF set.
        assert_eq!(layout.subgrids.len(), 2);

        for (i, subgrid) in layout.subgrids.iter().enumerate() {
            assert!(subgrid.log_q2s.windows(2).all(|w| w[0] < w[1]));
            if i > 0 {
                // Exact equality is the point: adjacent subgrids share their
                // boundary knot verbatim (same provenance, not independently
                // rounded), which is exactly what `QGridNotContiguous` checks.
                #[allow(clippy::float_cmp)]
                let shares_boundary_knot =
                    *layout.subgrids[i - 1].log_q2s.last().unwrap() == subgrid.log_q2s[0];
                assert!(shares_boundary_knot);
            }
            match &subgrid.values {
                ValuesLayout::Hermite {
                    coeffs,
                    member_stride,
                } => {
                    assert_eq!(coeffs.len(), member_stride * n_members);
                    assert_eq!(
                        *member_stride,
                        (layout.log_xs.len() - 1) * subgrid.log_q2s.len() * layout.n_flavors() * 4
                    );
                }
                ValuesLayout::Chebyshev { .. } => {
                    panic!("NNPDF40_nnlo_as_01180 is LogBicubic, expected a Hermite layout")
                }
            }
        }
    }

    #[test]
    fn rejects_an_empty_member_list() {
        assert!(matches!(
            DeviceGridLayout::build(&[]),
            Err(GridPrepError::NoMembers)
        ));
    }

    /// `MAP22_grids_FF_Km_N3LL` is `LogChebyshev`, but it's a 3-D `(x, kT, Q²)` TMD
    /// grid, not a plain 2-D collinear one (its own benchmark queries it with a
    /// 3-element point). It should be rejected, not silently mis-exported.
    ///
    /// This is also the only locally available `LogChebyshev` set, so it only
    /// covers the rejection path — genuine 2-D `LogChebyshev` coverage needs
    /// either a real 2-D `LogChebyshev` set or a synthetic on-disk fixture built
    /// through `neopdf`'s own writer, neither of which exists yet.
    #[test]
    fn rejects_a_non_collinear_chebyshev_set() {
        let members = load_members("MAP22_grids_FF_Km_N3LL.neopdf.lz4");
        assert!(matches!(
            DeviceGridLayout::build(&members),
            Err(GridPrepError::Export {
                source: CollinearExportError::NotCollinear(_),
                ..
            })
        ));
    }
}
