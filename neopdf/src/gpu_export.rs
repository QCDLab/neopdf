//! A minimal, GPU-oriented export of a loaded collinear PDF member.
//!
//! This module is the boundary a device/GPU backend crate (e.g. `neopdf_cuda`) reads
//! across. It owns plain `f64` data only — no `ninterp` types, no `neopdf`-internal
//! strategy representations — so a downstream crate never needs to depend on this
//! crate's interpolation machinery, only on the exported numbers.
//!
//! [`GridPDF::collinear_export`](super::gridpdf::GridPDF::collinear_export) is the
//! entry point. It only succeeds for the plain 2-D `(x, Q²)` "collinear" case:
//! `LogBicubic` or `LogChebyshev` interpolation, one or more subgrids that all share
//! the same x grid and are contiguous in Q² (the standard LHAPDF layout: real sets
//! routinely split into several subgrids at flavor-threshold boundaries, e.g.
//! `NNPDF40_nnlo_as_01180` splits in two at the bottom-quark threshold), and a
//! table-or-constant (not analytic) `alpha_s`. Anything else — nuclear, TMD,
//! fragmentation grids with extra axes, subgrids with different x grids, or
//! analytic running — is explicitly out of scope and reported as an error rather
//! than silently exporting something a GPU kernel would misinterpret.

use thiserror::Error;

/// Why [`GridPDF::collinear_export`](super::gridpdf::GridPDF::collinear_export) could
/// not produce a [`CollinearExport`].
#[derive(Debug, Error)]
pub enum CollinearExportError {
    /// The PDF set's interpolator type has no fast all-flavor table to export
    /// (only `LogBicubic` and `LogChebyshev` are supported).
    #[error("interpolator type {0:?} has no collinear GPU export")]
    UnsupportedInterpolatorType(crate::metadata::InterpolatorType),
    /// A fast-path table exists but isn't a plain 2-D `(x, Q²)` table (it has
    /// nucleon/kT/xi/delta axes).
    #[error("subgrid {0}'s fast-path table is not a plain 2-D (x, Q²) table")]
    NotCollinear(usize),
    /// Subgrid `at`'s x grid differs from subgrid 0's; the collinear export
    /// requires every subgrid to share one x grid (only Q² may vary between them).
    #[error("subgrid {at}'s x grid differs from subgrid 0's")]
    XGridMismatch {
        /// The offending subgrid's index.
        at: usize,
    },
    /// Subgrid `at`'s Q² range doesn't start where subgrid `at - 1`'s ends; the
    /// collinear export requires subgrids to be contiguous in Q², in order.
    #[error(
        "subgrid {at}'s Q² range does not start where subgrid {} ends \
         (expected first knot {expected}, found {found})",
        at - 1
    )]
    QGridNotContiguous {
        /// The offending subgrid's index.
        at: usize,
        /// The log-Q² value subgrid `at - 1` ended on.
        expected: f64,
        /// The log-Q² value subgrid `at` actually started on.
        found: f64,
    },
    /// `alpha_s` is computed analytically rather than from a table or constant;
    /// porting the closed-form running to a device kernel is unscoped follow-up.
    #[error("analytic alpha_s has no GPU export yet")]
    AnalyticAlphaSUnsupported,
}

/// The all-flavor value table backing a [`CollinearSubgrid`], in whichever of
/// `neopdf`'s two fast-path representations the PDF set was built with.
pub enum CollinearValues {
    /// Interleaved cubic Hermite coefficients, `neopdf`'s `LogBicubic` fast path.
    /// Flat layout: `[(x_cell * n_q2 + q2_idx) * n_flavors + flavor) * 4 + c]`.
    Hermite {
        /// Flat coefficient array, see the field's layout note above.
        coeffs: Vec<f64>,
    },
    /// Barycentric Chebyshev values, `neopdf`'s `LogChebyshev` fast path.
    Chebyshev {
        /// One flat, C-order `[n_x * n_q2]` array per flavor, in `pids` order.
        values: Vec<Vec<f64>>,
        /// Chebyshev nodes in `[-1, 1]` for the x axis, one per x knot.
        x_nodes: Vec<f64>,
        /// Chebyshev nodes in `[-1, 1]` for this subgrid's Q² axis, one per Q² knot.
        q2_nodes: Vec<f64>,
    },
}

/// `alpha_s(Q²)` data backing a [`CollinearExport`]'s fused `alpha_s` output.
pub enum AlphaSValues {
    /// A single, Q-independent value.
    Constant(f64),
    /// Log-Q² knots and the `alpha_s` value at each knot, for the same cubic
    /// interpolation the CPU path uses (`AlphaSCubicInterpolation`).
    Cubic {
        /// Log-transformed Q² knots, strictly increasing.
        log_q2s: Vec<f64>,
        /// `alpha_s` value at each knot in `log_q2s`.
        alphas: Vec<f64>,
    },
}

/// One Q²-contiguous piece of a [`CollinearExport`]. All subgrids of one export
/// share the same x grid (see [`CollinearExport::log_xs`]); this holds the part
/// that's specific to one Q² range.
pub struct CollinearSubgrid {
    /// Log-transformed Q² grid knots for this subgrid, strictly increasing.
    pub log_q2s: Vec<f64>,
    /// The all-flavor value table for this subgrid.
    pub values: CollinearValues,
}

/// A device/GPU-ready export of one collinear PDF member.
///
/// All coordinate grids are already log-transformed (`ln(x)`, `ln(Q²)`), matching
/// what the CPU fast paths themselves interpolate over — a device kernel takes the
/// same `ln` of its query point and does no further coordinate transformation.
///
/// `subgrids` is ordered by increasing Q², each contiguous with the next (the last
/// log-Q² knot of one subgrid equals the first of the next) — a device kernel locates
/// the right subgrid for a query Q² with a small search over `subgrids.len()` ranges,
/// the same way `GridArray::find_subgrid` does on the CPU path.
pub struct CollinearExport {
    /// Log-transformed x grid knots, strictly increasing, shared by every subgrid.
    pub log_xs: Vec<f64>,
    /// Flavor PIDs, in the same order as every per-flavor axis below.
    pub pids: Vec<i32>,
    /// This member's subgrids, ordered by increasing, contiguous Q² range.
    pub subgrids: Vec<CollinearSubgrid>,
    /// The fused `alpha_s(Q²)` data.
    pub alphas: AlphaSValues,
}

impl CollinearExport {
    /// Number of flavors, i.e. `self.pids.len()`.
    pub fn n_flavors(&self) -> usize {
        self.pids.len()
    }
}
