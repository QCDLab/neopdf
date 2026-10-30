//! Flattens N PDF members into one contiguous, device-upload-ready layout.
//!
//! This module is pure Rust and depends only on [`neopdf::gpu_export`] — no GPU
//! toolchain is needed to build or test it. It is the place almost all of this
//! crate's correctness risk lives: once [`DeviceGridLayout`] is right, the device
//! kernels are a transliteration of its indexing (see [`crate::eval`]).
//!
//! LHAPDF-style sets store the same `(x, Q²)` grid (across one or more Q²-contiguous
//! subgrids — see [`neopdf::gpu_export`] for why there can be more than one) and the
//! same flavor PIDs for every member of a set — only the tabulated values (and often
//! the `alpha_s` table) differ by member. [`DeviceGridLayout::build`] checks that
//! assumption explicitly rather than relying on it silently.

use std::mem::discriminant;

use neopdf::gpu_export::{AlphaSValues, CollinearExport, CollinearExportError, CollinearValues};
use neopdf::gridpdf::ForcePositive;
use neopdf::pdf::PDF;
use thiserror::Error;

use crate::eval::{Chebyshev, GridView, Hermite};

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
    /// A member's grids, flavors or fast-path kind differ from member 0's.
    #[error("member {member}'s {what} differs from member 0's")]
    Mismatch {
        /// The member's index in the input slice.
        member: usize,
        /// What differs (e.g. `"x grid"`, `"subgrid 1"`).
        what: String,
    },
}

/// Which of `neopdf`'s two collinear fast paths a [`DeviceGridLayout`] holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interpolation {
    /// Interleaved cubic Hermite coefficients (`LogBicubic`).
    Hermite,
    /// Barycentric Chebyshev values (`LogChebyshev`).
    Chebyshev,
}

/// A device-upload-ready layout for `n_members` members of one collinear PDF set.
///
/// Every per-subgrid quantity is concatenated over subgrids and addressed through
/// an offsets array of length `n_subgrids + 1`, so the whole layout is a handful of
/// flat buffers a kernel can take as plain slices:
///
/// - Q² knots of subgrid `s`: `log_q2s[q2_offsets[s]..q2_offsets[s + 1]]`.
/// - Values of subgrid `s`: `values[value_offsets[s]..value_offsets[s + 1]]`, split
///   into `n_members` equal member-major blocks. Within member `m`'s block, the
///   local index is `((ix * n_q2 + iq2) * n_flavors + flavor) * 4 + c` for
///   [`Interpolation::Hermite`] (the CPU's own coefficient layout), and
///   `(flavor * n_x + ix) * n_q2 + iq2` for [`Interpolation::Chebyshev`].
/// - `alpha_s` of member `m`: `alphas_log_q2s/alphas[alphas_offsets[m]..alphas_offsets[m + 1]]`;
///   a single entry means a Q-independent value.
pub struct DeviceGridLayout {
    /// Which fast path `values` holds.
    pub interpolation: Interpolation,
    /// Flavor PIDs, shared by every member.
    pub pids: Vec<i32>,
    /// Number of members flattened into this layout.
    pub n_members: usize,
    /// Lower bound applied to every PDF value, encoding the members' [`ForcePositive`].
    pub clip_floor: f64,
    /// Log-transformed x grid knots, shared by every member and every subgrid.
    pub log_xs: Vec<f64>,
    /// Chebyshev nodes matching `log_xs`; empty for [`Interpolation::Hermite`].
    pub x_nodes: Vec<f64>,
    /// Every subgrid's log-transformed Q² knots, concatenated in Q² order.
    pub log_q2s: Vec<f64>,
    /// Chebyshev nodes matching `log_q2s`; empty for [`Interpolation::Hermite`].
    pub q2_nodes: Vec<f64>,
    /// Subgrid boundaries within `log_q2s`/`q2_nodes`.
    pub q2_offsets: Vec<usize>,
    /// Every subgrid's value table across every member, concatenated.
    pub values: Vec<f64>,
    /// Subgrid boundaries within `values`.
    pub value_offsets: Vec<usize>,
    /// Every member's `alpha_s` log-Q² knots, concatenated in member order.
    pub alphas_log_q2s: Vec<f64>,
    /// Every member's `alpha_s` values, matching `alphas_log_q2s`.
    pub alphas: Vec<f64>,
    /// Member boundaries within `alphas_log_q2s`/`alphas`.
    pub alphas_offsets: Vec<usize>,
}

impl DeviceGridLayout {
    /// Number of flavors, i.e. `self.pids.len()`.
    #[must_use]
    pub const fn n_flavors(&self) -> usize {
        self.pids.len()
    }

    /// Borrows this layout as the slices the evaluation kernels read.
    #[must_use]
    pub fn view(&self) -> GridView<'_> {
        GridView {
            n_members: self.n_members,
            n_flavors: self.n_flavors(),
            clip_floor: self.clip_floor,
            log_xs: &self.log_xs,
            x_nodes: &self.x_nodes,
            log_q2s: &self.log_q2s,
            q2_nodes: &self.q2_nodes,
            q2_offsets: &self.q2_offsets,
            values: &self.values,
            value_offsets: &self.value_offsets,
            alphas_log_q2s: &self.alphas_log_q2s,
            alphas: &self.alphas,
            alphas_offsets: &self.alphas_offsets,
        }
    }

    /// Evaluates the whole `[n_events, n_members, n_flavors + 1]` batch output on the
    /// host, with the same code the device kernel runs.
    ///
    /// # Panics
    ///
    /// Panics if `xs` and `q2s` differ in length or `out` has the wrong length.
    pub fn eval_batch(&self, xs: &[f64], q2s: &[f64], out: &mut [f64]) {
        let grid = self.view();
        match self.interpolation {
            Interpolation::Hermite => grid.eval_batch::<Hermite>(xs, q2s, out),
            Interpolation::Chebyshev => grid.eval_batch::<Chebyshev>(xs, q2s, out),
        }
    }

    /// Flattens `members` (same PDF set, every member) into one device-ready layout.
    ///
    /// Every member must share the same x grid, the same subgrid Q² ranges, the
    /// same flavor PIDs, the same fast-path kind and the same [`ForcePositive`]
    /// setting as member 0.
    ///
    /// # Errors
    ///
    /// Returns an error if `members` is empty, if any member's
    /// [`PDF::collinear_export`] fails, or if the members disagree on any of the
    /// properties above. See [`GridPrepError`] for each case.
    pub fn build(members: &[PDF]) -> Result<Self, GridPrepError> {
        let first = members.first().ok_or(GridPrepError::NoMembers)?;
        let exports: Vec<CollinearExport> = members
            .iter()
            .enumerate()
            .map(|(member, pdf)| {
                pdf.collinear_export()
                    .map_err(|source| GridPrepError::Export { member, source })
            })
            .collect::<Result<_, _>>()?;

        let reference = &exports[0];
        let mismatch = |member, what: &str| GridPrepError::Mismatch {
            member,
            what: what.to_owned(),
        };
        for (member, (export, pdf)) in exports.iter().zip(members).enumerate().skip(1) {
            if export.log_xs != reference.log_xs {
                return Err(mismatch(member, "x grid"));
            }
            if export.pids != reference.pids {
                return Err(mismatch(member, "flavor PIDs"));
            }
            if export.subgrids.len() != reference.subgrids.len() {
                return Err(mismatch(member, "subgrid count"));
            }
            if discriminant(pdf.is_force_positive()) != discriminant(first.is_force_positive()) {
                return Err(mismatch(member, "force-positive setting"));
            }
            for (s, (this, base)) in export.subgrids.iter().zip(&reference.subgrids).enumerate() {
                if this.log_q2s != base.log_q2s || !same_shape(&this.values, &base.values) {
                    return Err(mismatch(member, &format!("subgrid {s}")));
                }
            }
        }

        let interpolation = match reference.subgrids.first().map(|s| &s.values) {
            Some(CollinearValues::Chebyshev { .. }) => Interpolation::Chebyshev,
            _ => Interpolation::Hermite,
        };

        let mut layout = Self {
            interpolation,
            pids: reference.pids.clone(),
            n_members: exports.len(),
            clip_floor: clip_floor(first.is_force_positive()),
            log_xs: reference.log_xs.clone(),
            x_nodes: Vec::new(),
            log_q2s: Vec::new(),
            q2_nodes: Vec::new(),
            q2_offsets: vec![0],
            values: Vec::new(),
            value_offsets: vec![0],
            alphas_log_q2s: Vec::new(),
            alphas: Vec::new(),
            alphas_offsets: vec![0],
        };

        for (s, subgrid) in reference.subgrids.iter().enumerate() {
            layout.log_q2s.extend_from_slice(&subgrid.log_q2s);
            if let CollinearValues::Chebyshev {
                x_nodes, q2_nodes, ..
            } = &subgrid.values
            {
                layout.x_nodes.clone_from(x_nodes);
                layout.q2_nodes.extend_from_slice(q2_nodes);
            }
            layout.q2_offsets.push(layout.log_q2s.len());

            for export in &exports {
                match &export.subgrids[s].values {
                    CollinearValues::Hermite { coeffs } => layout.values.extend_from_slice(coeffs),
                    CollinearValues::Chebyshev { values, .. } => {
                        layout.values.extend(values.iter().flatten());
                    }
                }
            }
            layout.value_offsets.push(layout.values.len());
        }

        for export in &exports {
            match &export.alphas {
                AlphaSValues::Constant(value) => {
                    layout.alphas_log_q2s.push(0.0);
                    layout.alphas.push(*value);
                }
                AlphaSValues::Cubic { log_q2s, alphas } => {
                    layout.alphas_log_q2s.extend_from_slice(log_q2s);
                    layout.alphas.extend_from_slice(alphas);
                }
            }
            layout.alphas_offsets.push(layout.alphas.len());
        }

        Ok(layout)
    }
}

/// Whether two value tables use the same fast path with the same number of entries.
fn same_shape(a: &CollinearValues, b: &CollinearValues) -> bool {
    match (a, b) {
        (CollinearValues::Hermite { coeffs: a }, CollinearValues::Hermite { coeffs: b }) => {
            a.len() == b.len()
        }
        (
            CollinearValues::Chebyshev { values: a, .. },
            CollinearValues::Chebyshev { values: b, .. },
        ) => a.len() == b.len() && a.iter().zip(b).all(|(a, b)| a.len() == b.len()),
        _ => false,
    }
}

/// The lower bound `value.max(floor)` that reproduces a [`ForcePositive`] policy.
const fn clip_floor(policy: &ForcePositive) -> f64 {
    match policy {
        ForcePositive::ClipNegative => 0.0,
        ForcePositive::ClipSmall => 1e-10,
        ForcePositive::NoClipping => f64::NEG_INFINITY,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flattens_nnpdf40_members_with_shared_grid_and_pids() {
        let members = PDF::load_pdfs("NNPDF40_nnlo_as_01180");
        let n_members = members.len();
        let layout = DeviceGridLayout::build(&members).unwrap();

        assert_eq!(layout.interpolation, Interpolation::Hermite);
        assert_eq!(layout.n_members, n_members);
        assert_eq!(layout.pids, members[0].pids().to_vec());
        assert!(layout.log_xs.windows(2).all(|w| w[0] < w[1]));
        assert!(layout.x_nodes.is_empty() && layout.q2_nodes.is_empty());
        // NNPDF40_nnlo_as_01180 splits into 2 Q²-contiguous subgrids at the
        // bottom-quark threshold -- this is the normal LHAPDF layout, not an
        // edge case, so a 1-subgrid-only export would reject almost every real
        // proton PDF set.
        assert_eq!(layout.q2_offsets.len(), 3);
        assert_eq!(layout.value_offsets.len(), 3);
        assert_eq!(layout.alphas_offsets.len(), n_members + 1);

        for s in 0..2 {
            let log_q2s = &layout.log_q2s[layout.q2_offsets[s]..layout.q2_offsets[s + 1]];
            assert!(log_q2s.windows(2).all(|w| w[0] < w[1]));
            assert_eq!(
                layout.value_offsets[s + 1] - layout.value_offsets[s],
                n_members * (layout.log_xs.len() - 1) * log_q2s.len() * layout.n_flavors() * 4
            );
        }
        // Adjacent subgrids share their boundary knot verbatim.
        #[allow(clippy::float_cmp)]
        let shares_boundary_knot =
            layout.log_q2s[layout.q2_offsets[1] - 1] == layout.log_q2s[layout.q2_offsets[1]];
        assert!(shares_boundary_knot);
    }

    #[test]
    fn rejects_an_empty_member_list() {
        assert!(matches!(
            DeviceGridLayout::build(&[]),
            Err(GridPrepError::NoMembers)
        ));
    }

    /// `MAP22_grids_FF_Km_N3LL` is `LogChebyshev`, but it's a 3-D `(x, kT, Q²)` TMD
    /// grid, not a plain 2-D collinear one. It should be rejected, not silently
    /// mis-exported.
    #[test]
    fn rejects_a_non_collinear_chebyshev_set() {
        let members = PDF::load_pdfs("MAP22_grids_FF_Km_N3LL.neopdf.lz4");
        assert!(matches!(
            DeviceGridLayout::build(&members),
            Err(GridPrepError::Export {
                source: CollinearExportError::NotCollinear(_),
                ..
            })
        ));
    }
}
