//! This module defines the main PDF grid interface and data structures for handling PDF grid data.
//!
//! # Contents
//!
//! - [`GridPDF`]: High-level interface for PDF grid interpolation and metadata access.
//! - [`GridArray`]: Stores the full set of subgrids and flavor IDs.

use core::panic;
use ndarray::{Array1, Array2};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

use super::alphas::AlphaS;
use super::interpolator::{DynInterpolator, InterpolatorFactory};
use super::metadata::{InterpolatorType, MetaData};
use super::parser::SubgridData;
use super::strategy::LogBicubicInterpolation;
use super::subgrid::{ParamRange, RangeParameters, SubGrid};
use super::utils;

/// Errors that can occur during PDF grid operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Error indicating that no suitable subgrid was found for the given `x` and `q2` values.
    #[error("No subgrid found for x={x}, q2={q2}")]
    SubgridNotFound {
        /// The momentum fraction `x` value.
        x: f64,
        /// The energy scale squared `q2` value.
        q2: f64,
    },
    /// Error indicating invalid interpolation parameters, with a descriptive message.
    #[error("Invalid interpolation parameters: {0}")]
    InterpolationError(String),
}

/// Stores the complete PDF grid data, including all subgrids and flavor information.
#[derive(Debug, Serialize, Deserialize)]
pub struct GridArray {
    /// An array of particle flavor IDs (PIDs).
    pub pids: Array1<i32>,
    /// A collection of `SubGrid` instances that make up the full grid.
    pub subgrids: Vec<SubGrid>,
    /// Precomputed lookup from normalized PID to index (avoids per-call linear scan).
    #[serde(skip)]
    pid_lookup: HashMap<i32, usize>,
}

impl GridArray {
    /// Builds the PID lookup table from a pids array.
    fn build_pid_lookup(pids: &Array1<i32>) -> HashMap<i32, usize> {
        let mut pid_lookup = HashMap::with_capacity(pids.len());
        for (idx, &pid) in pids.iter().enumerate() {
            let normalized = if pid == 0 { 21 } else { pid };
            pid_lookup.entry(normalized).or_insert(idx);
        }
        pid_lookup
    }

    /// Creates a `GridArray` from prebuilt pids and subgrids.
    pub fn from_parts(pids: Array1<i32>, subgrids: Vec<SubGrid>) -> Self {
        let pid_lookup = Self::build_pid_lookup(&pids);
        Self {
            pids,
            subgrids,
            pid_lookup,
        }
    }

    /// Creates a new `GridArray` from a vector of `SubgridData`.
    ///
    /// # Arguments
    ///
    /// * `subgrid_data` - A vector of `SubgridData` parsed from the PDF data file.
    /// * `pids` - A vector of particle flavor IDs.
    pub fn new(subgrid_data: Vec<SubgridData>, pids: Vec<i32>) -> Self {
        let nflav = pids.len();
        let subgrids = subgrid_data
            .into_iter()
            .map(|data| {
                if data.xis.len() > 1 || data.deltas.len() > 1 {
                    SubGrid::new_8d(
                        data.nucleons,
                        data.alphas,
                        data.xis,
                        data.deltas,
                        data.kts,
                        data.xs,
                        data.q2s,
                        nflav,
                        data.grid_data,
                    )
                } else {
                    SubGrid::new(
                        data.nucleons,
                        data.alphas,
                        data.kts,
                        data.xs,
                        data.q2s,
                        nflav,
                        data.grid_data,
                    )
                }
            })
            .collect();

        let pids = Array1::from_vec(pids);
        let pid_lookup = Self::build_pid_lookup(&pids);

        Self {
            pids,
            subgrids,
            pid_lookup,
        }
    }

    /// Gets the PDF value at a specific knot point in the grid.
    ///
    /// # Arguments
    ///
    /// * `nucleon_idx` - The index of the nucleon.
    /// * `alpha_idx` - The index of the alpha_s value.
    /// * `kt_idx` - The index of the `kT` value.
    /// * `x_idx` - The index of the `x` value.
    /// * `q2_idx` - The index of the `q2` value.
    /// * `flavor_id` - The particle flavor ID.
    /// * `subgrid_idx` - The index of the subgrid.
    ///
    /// # Returns
    ///
    /// The PDF value `f64` at the specified grid point.
    ///
    /// # Panics
    ///
    /// Panics if the `flavor_id` is invalid.
    #[allow(clippy::too_many_arguments)]
    pub fn xf_from_index(
        &self,
        nucleon_idx: usize,
        alpha_idx: usize,
        kt_idx: usize,
        x_idx: usize,
        q2_idx: usize,
        flavor_id: i32,
        subgrid_idx: usize,
    ) -> f64 {
        let pid_idx = self.pid_index(flavor_id).expect("Invalid flavor ID");
        let grid_view = self.subgrids[subgrid_idx].grid.view();
        grid_view[[nucleon_idx, alpha_idx, pid_idx, kt_idx, x_idx, q2_idx]]
    }

    /// Finds the index of the subgrid that contains the given point.
    ///
    /// # Arguments
    ///
    /// * `points` - A slice of coordinates for the point.
    ///
    /// # Returns
    ///
    /// An `Option<usize>` containing the index of the subgrid if found, otherwise `None`.
    pub fn find_subgrid(&self, points: &[f64]) -> Option<usize> {
        // Fast path: single subgrid (common case), clamping handles boundaries
        if self.subgrids.len() == 1 {
            return Some(0);
        }
        self.subgrids
            .iter()
            .position(|sg| sg.contains_point(points))
            .or_else(|| {
                self.subgrids
                    .iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| {
                        a.distance_to_point(points)
                            .partial_cmp(&b.distance_to_point(points))
                            .unwrap()
                    })
                    .map(|(idx, _)| idx)
            })
    }

    /// Gets the index corresponding to a given flavor ID.
    fn pid_index(&self, flavor_id: i32) -> Option<usize> {
        let normalized = if flavor_id == 0 { 21 } else { flavor_id };
        // Fast path: use precomputed lookup (populated by GridArray::new)
        if !self.pid_lookup.is_empty() {
            return self.pid_lookup.get(&normalized).copied();
        }
        // Fallback: linear scan (for deserialized GridArrays where pid_lookup is empty)
        self.pids
            .iter()
            .position(|&pid| (if pid == 0 { 21 } else { pid }) == normalized)
    }

    /// Gets the overall parameter ranges across all subgrids.
    ///
    /// This method calculates the minimum and maximum values for the nucleon numbers `A`,
    /// the AlphaS values `as`, the momentum fraction `x` and the energy scale `q2` across
    /// all subgrids to determine the global parameter space.
    ///
    /// # Returns
    ///
    /// A `RangeParameters` struct containing the global parameter ranges.
    pub fn global_ranges(&self) -> RangeParameters {
        fn global_range<F>(subgrids: &[SubGrid], extractor: F) -> ParamRange
        where
            F: Fn(&SubGrid) -> &ParamRange,
        {
            let min = subgrids
                .iter()
                .map(|sg| extractor(sg).min)
                .fold(f64::INFINITY, f64::min);
            let max = subgrids
                .iter()
                .map(|sg| extractor(sg).max)
                .fold(f64::NEG_INFINITY, f64::max);
            ParamRange::new(min, max)
        }

        RangeParameters::new(
            global_range(&self.subgrids, |sg| &sg.nucleons_range),
            global_range(&self.subgrids, |sg| &sg.alphas_range),
            global_range(&self.subgrids, |sg| &sg.xi_range),
            global_range(&self.subgrids, |sg| &sg.delta_range),
            global_range(&self.subgrids, |sg| &sg.kt_range),
            global_range(&self.subgrids, |sg| &sg.x_range),
            global_range(&self.subgrids, |sg| &sg.q2_range),
        )
    }
}

/// Defines the methods for handling negative or small PDF values.
#[repr(C)]
#[derive(Debug, Clone)]
pub enum ForcePositive {
    /// If the calculated PDF value is negative, it is forced to 0.
    ClipNegative,
    /// If the calculated PDF value is less than 1e-10, it is set to 1e-10.
    ClipSmall,
    /// No clipping is done, value is returned as it is.
    NoClipping,
}

/// Helper functions for force-positive clipping via function pointer.
fn fp_identity(v: f64) -> f64 {
    v
}
fn fp_clip_negative(v: f64) -> f64 {
    v.max(0.0)
}
fn fp_clip_small(v: f64) -> f64 {
    v.max(1e-10)
}

/// Interleaved bicubic coefficients for fast all-flavor evaluation.
///
/// Layout: coefficients are stored as `[cell][flavor][4]` where
/// `cell = ix * nq2knots + iq2`, giving optimal cache locality when
/// evaluating all flavors at the same `(x, Q2)` point.
struct InterleavedBicubic {
    /// Flat coefficient array: `[(ix * nq2 + iq2) * n_flavors * 4 + flavor * 4 + c]`
    coeffs: Vec<f64>,
    /// Log-transformed x grid (shared across all flavors).
    log_xs: Vec<f64>,
    /// Log-transformed Q2 grid (shared across all flavors).
    log_q2s: Vec<f64>,
    /// Number of Q2 knots.
    nq2: usize,
    /// Number of flavors stored.
    n_flavors: usize,
}

impl InterleavedBicubic {
    /// Build interleaved coefficients from a subgrid's per-flavor data.
    fn build(subgrid: &SubGrid, n_pids: usize) -> Self {
        let log_xs: Vec<f64> = subgrid.xs.iter().map(|&x| x.ln()).collect();
        let log_q2s: Vec<f64> = subgrid.q2s.iter().map(|&q2| q2.ln()).collect();
        let nxknots = log_xs.len();
        let nq2knots = log_q2s.len();

        // Compute per-flavor coefficients using the existing algorithm,
        // then interleave into [cell][flavor][4].
        let n_cells = (nxknots - 1) * nq2knots;
        let mut interleaved = vec![0.0f64; n_cells * n_pids * 4];

        for pid_idx in 0..n_pids {
            let grid_slice = subgrid.grid_slice(pid_idx).to_owned();
            let data = ninterp::data::InterpData2D {
                grid: [
                    ndarray::Array1::from_vec(log_xs.clone()),
                    ndarray::Array1::from_vec(log_q2s.clone()),
                ],
                values: grid_slice,
            };
            let flavor_coeffs = LogBicubicInterpolation::compute_polynomial_coefficients(&data);

            // Copy into interleaved layout
            for cell in 0..n_cells {
                let src = cell * 4;
                let dst = (cell * n_pids + pid_idx) * 4;
                interleaved[dst..dst + 4].copy_from_slice(&flavor_coeffs[src..src + 4]);
            }
        }

        Self {
            coeffs: interleaved,
            log_xs,
            log_q2s,
            nq2: nq2knots,
            n_flavors: n_pids,
        }
    }

    /// Evaluate the Hermite x-polynomial for a given cell and flavor.
    #[inline(always)]
    fn hermite_x(&self, cell: usize, flavor: usize, u: f64) -> f64 {
        let base = (cell * self.n_flavors + flavor) * 4;
        let c = &self.coeffs[base..base + 4];
        let u2 = u * u;
        let u3 = u2 * u;
        c[0] * u3 + c[1] * u2 + c[2] * u + c[3]
    }

    /// Evaluate all flavors at `(ix, iq2, u, v)`.
    ///
    /// `pid_slots` maps each output position to `Some(flavor_index)` or `None`.
    fn eval_allpids(
        &self,
        ix: usize,
        iq2: usize,
        u: f64,
        v: f64,
        pid_slots: &[Option<usize>],
        force_positive_fn: fn(f64) -> f64,
        out: &mut [f64],
    ) {
        let nq2 = self.nq2;
        let dq_1 = self.log_q2s[iq2 + 1] - self.log_q2s[iq2];

        for (o, slot) in out.iter_mut().zip(pid_slots.iter()) {
            let fi = match *slot {
                Some(idx) => idx,
                None => {
                    *o = 0.0;
                    continue;
                }
            };

            let cell_lo = ix * nq2 + iq2;
            let cell_hi = ix * nq2 + iq2 + 1;

            let vl = self.hermite_x(cell_lo, fi, u);
            let vh = self.hermite_x(cell_hi, fi, u);

            let (vdl, vdh) = if iq2 == 0 {
                let vdl_val = vh - vl;
                let vhh = self.hermite_x(ix * nq2 + iq2 + 2, fi, u);
                let dq_2_inv = 1.0 / (self.log_q2s[iq2 + 2] - self.log_q2s[iq2 + 1]);
                let vdh_val = (vdl_val + (vhh - vh) * dq_1 * dq_2_inv) * 0.5;
                (vdl_val, vdh_val)
            } else if iq2 == nq2 - 2 {
                let vdh_val = vh - vl;
                let vll = self.hermite_x(ix * nq2 + iq2 - 1, fi, u);
                let dq_0_inv = 1.0 / (self.log_q2s[iq2] - self.log_q2s[iq2 - 1]);
                let vdl_val = (vdh_val + (vl - vll) * dq_1 * dq_0_inv) * 0.5;
                (vdl_val, vdh_val)
            } else {
                let vll = self.hermite_x(ix * nq2 + iq2 - 1, fi, u);
                let dq_0_inv = 1.0 / (self.log_q2s[iq2] - self.log_q2s[iq2 - 1]);
                let vhh = self.hermite_x(ix * nq2 + iq2 + 2, fi, u);
                let dq_2_inv = 1.0 / (self.log_q2s[iq2 + 2] - self.log_q2s[iq2 + 1]);
                let vdl_val = ((vh - vl) + (vl - vll) * dq_1 * dq_0_inv) * 0.5;
                let vdh_val = ((vh - vl) + (vhh - vh) * dq_1 * dq_2_inv) * 0.5;
                (vdl_val, vdh_val)
            };

            *o = force_positive_fn(utils::hermite_cubic_interpolate(v, vl, vdl, vh, vdh));
        }
    }
}

/// The main PDF grid interface, providing high-level methods for interpolation.
pub struct GridPDF {
    /// The metadata associated with the PDF set.
    info: MetaData,
    /// The underlying grid data stored in a `GridArray`.
    pub knot_array: GridArray,
    /// A nested vector of interpolators for each subgrid and flavor.
    interpolators: Vec<Vec<Box<dyn DynInterpolator>>>,
    /// Calculator for the running of alpha_s.
    alphas: AlphaS,
    /// Clip the values to positive definite numbers if negatives.
    pub force_positive: Option<ForcePositive>,
    /// Cached: whether the interpolator uses log-space coordinates.
    use_log: bool,
    /// Cached: function pointer for force-positive clipping (avoids per-call match).
    force_positive_fn: fn(f64) -> f64,
    /// Optional fast path for all-flavor evaluation (2D LogBicubic only).
    /// One entry per subgrid.
    interleaved: Option<Vec<InterleavedBicubic>>,
}

impl GridPDF {
    /// Creates a new `GridPDF` instance.
    ///
    /// # Arguments
    ///
    /// * `info` - The `MetaData` for the PDF set.
    /// * `knot_array` - The `GridArray` containing the grid data.
    pub fn new(info: MetaData, knot_array: GridArray) -> Self {
        let interpolators = Self::build_interpolators(&info, &knot_array);
        let alphas = AlphaS::from_metadata(&info).expect("Failed to create AlphaS calculator");
        let use_log = matches!(
            info.interpolator_type,
            InterpolatorType::LogBilinear
                | InterpolatorType::LogBicubic
                | InterpolatorType::LogTricubic
                | InterpolatorType::LogFourCubic
                | InterpolatorType::LogFiveCubic
                | InterpolatorType::LogChebyshev
        );

        // Build interleaved coefficients for 2D LogBicubic grids
        let interleaved = if info.interpolator_type == InterpolatorType::LogBicubic {
            let all_2d = knot_array.subgrids.iter().all(|sg| {
                matches!(
                    sg.interpolation_config(),
                    super::interpolator::InterpolationConfig::TwoD
                )
            });
            if all_2d {
                Some(
                    knot_array
                        .subgrids
                        .iter()
                        .map(|sg| InterleavedBicubic::build(sg, knot_array.pids.len()))
                        .collect(),
                )
            } else {
                None
            }
        } else {
            None
        };

        Self {
            info,
            knot_array,
            interpolators,
            alphas,
            force_positive: None,
            use_log,
            force_positive_fn: fp_identity,
            interleaved,
        }
    }

    /// Sets the method for handling negative or small PDF values.
    ///
    /// # Arguments
    ///
    /// * `flag` - The `ForcePositive` enum variant specifying the clipping method.
    pub fn set_force_positive(&mut self, flag: ForcePositive) {
        self.force_positive_fn = match &flag {
            ForcePositive::ClipNegative => fp_clip_negative,
            ForcePositive::ClipSmall => fp_clip_small,
            ForcePositive::NoClipping => fp_identity,
        };
        self.force_positive = Some(flag);
    }

    /// Applies the configured clipping method to a given PDF value.
    ///
    /// # Arguments
    ///
    /// * `value` - The PDF value to which the clipping policy is applied.
    ///
    /// # Returns
    ///
    /// The clipped PDF value, according to the policy set by `set_force_positive`.
    pub fn apply_force_positive(&self, value: f64) -> f64 {
        match &self.force_positive {
            Some(ForcePositive::ClipNegative) => value.max(0.0),
            Some(ForcePositive::ClipSmall) => value.max(1e-10),
            Some(ForcePositive::NoClipping) => value,
            _ => value,
        }
    }

    /// Builds the interpolators for all subgrids and flavors.
    fn build_interpolators(
        info: &MetaData,
        knot_array: &GridArray,
    ) -> Vec<Vec<Box<dyn DynInterpolator>>> {
        knot_array
            .subgrids
            .iter()
            .map(|subgrid| {
                (0..knot_array.pids.len())
                    .map(|pid_idx| {
                        InterpolatorFactory::create(
                            info.interpolator_type.clone(),
                            subgrid,
                            pid_idx,
                        )
                    })
                    .collect()
            })
            .collect()
    }

    /// Interpolates the PDF value for `(nucleons, alphas, x, q2)` and a given flavor.
    ///
    /// # Arguments
    ///
    /// * `flavor_id` - The particle flavor ID.
    /// * `points` - A slice containing the collection of points to interpolate on.
    ///
    /// # Returns
    ///
    /// A `Result` containing the interpolated PDF value or an `Error`.
    pub fn xfxq2(&self, flavor_id: i32, points: &[f64]) -> Result<f64, Error> {
        let subgrid_idx = self.knot_array.find_subgrid(points).ok_or_else(|| {
            let (x, q2) = self.get_x_q2(points);
            Error::SubgridNotFound { x, q2 }
        })?;

        let pid_idx = match self.knot_array.pid_index(flavor_id) {
            Some(idx) => idx,
            None => return Ok(0.0),
        };

        let mut buf = [0.0f64; 8];
        for (i, &p) in points.iter().enumerate() {
            buf[i] = if self.use_log { p.ln() } else { p };
        }

        self.interpolators[subgrid_idx][pid_idx]
            .interpolate_point(&buf[..points.len()])
            .map_err(|e| Error::InterpolationError(e.to_string()))
            .map(|result| (self.force_positive_fn)(result))
    }

    /// Internal fast path for interpolation — returns `f64` directly, no `Result` wrapping.
    /// Avoids `map_err` string allocation. Used by `PDF::xfxq2`.
    pub(crate) fn xfxq2_fast(&self, flavor_id: i32, points: &[f64]) -> f64 {
        let subgrid_idx = match self.knot_array.find_subgrid(points) {
            Some(idx) => idx,
            None => return 0.0,
        };

        let pid_idx = match self.knot_array.pid_index(flavor_id) {
            Some(idx) => idx,
            None => return 0.0,
        };

        let mut buf = [0.0f64; 8];
        for (i, &p) in points.iter().enumerate() {
            buf[i] = if self.use_log { p.ln() } else { p };
        }

        match self.interpolators[subgrid_idx][pid_idx].interpolate_point(&buf[..points.len()]) {
            Ok(result) => (self.force_positive_fn)(result),
            Err(e) => panic!("InterpolationError: {e}"),
        }
    }

    /// Fast path for evaluating all requested flavors at a single kinematic point.
    ///
    /// For 2D LogBicubic grids with interleaved coefficients, the binary search
    /// is performed once and all flavors are evaluated with optimal cache locality.
    /// Falls back to per-flavor interpolation for other grid types.
    pub(crate) fn xfxq2_allpids(&self, pids: &[i32], points: &[f64], out: &mut [f64]) {
        let subgrid_idx = match self.knot_array.find_subgrid(points) {
            Some(idx) => idx,
            None => {
                out.iter_mut().for_each(|v| *v = 0.0);
                return;
            }
        };

        // Fast path: interleaved 2D LogBicubic coefficients
        if let Some(ref il) = self.interleaved {
            let il = &il[subgrid_idx];
            let lx = points[0].ln();
            let lq2 = points[1].ln();

            let ix = match utils::find_interval_index(&il.log_xs, lx) {
                Ok(i) => i,
                Err(_) => {
                    out.iter_mut().for_each(|v| *v = 0.0);
                    return;
                }
            };
            let iq2 = match utils::find_interval_index(&il.log_q2s, lq2) {
                Ok(j) => j,
                Err(_) => {
                    out.iter_mut().for_each(|v| *v = 0.0);
                    return;
                }
            };

            let dx = il.log_xs[ix + 1] - il.log_xs[ix];
            let dy = il.log_q2s[iq2 + 1] - il.log_q2s[iq2];
            let u = (lx - il.log_xs[ix]) / dx;
            let v = (lq2 - il.log_q2s[iq2]) / dy;

            // Precompute PID → flavor slot mapping
            let mut pid_slots: [Option<usize>; 32] = [None; 32];
            for (i, &pid) in pids.iter().enumerate().take(32) {
                pid_slots[i] = self.knot_array.pid_index(pid);
            }

            il.eval_allpids(ix, iq2, u, v, &pid_slots[..pids.len()], self.force_positive_fn, out);
            return;
        }

        // Generic fallback
        let mut buf = [0.0f64; 8];
        for (i, &p) in points.iter().enumerate() {
            buf[i] = if self.use_log { p.ln() } else { p };
        }
        let log_points = &buf[..points.len()];

        for (o, &pid) in out.iter_mut().zip(pids.iter()) {
            *o = match self.knot_array.pid_index(pid) {
                Some(pid_idx) => {
                    match self.interpolators[subgrid_idx][pid_idx].interpolate_point(log_points) {
                        Ok(result) => (self.force_positive_fn)(result),
                        Err(e) => panic!("InterpolationError: {e}"),
                    }
                }
                None => 0.0,
            };
        }
    }

    /// Interpolates PDF values for multiple points in parallel.
    ///
    /// # Arguments
    ///
    /// * `flavors` - A vector of flavor IDs.
    /// * `slice_points` - A slice containing the collection of knots to interpolate on.
    ///   A knot is a collection of points containing `(nucleon, alphas, x, Q2)`.
    ///
    /// # Returns
    ///
    /// A 2D array of interpolated PDF values with shape `[flavors, N_knots]`.
    pub fn xfxq2s(&self, flavors: Vec<i32>, slice_points: &[&[f64]]) -> Array2<f64> {
        let grid_shape = [flavors.len(), slice_points.len()];
        let flatten_len = grid_shape.iter().product();

        let data: Vec<f64> = (0..flatten_len)
            .map(|idx| {
                let num_cols = slice_points.len();
                let (fl_idx, s_idx) = (idx / num_cols, idx % num_cols);
                self.xfxq2_fast(flavors[fl_idx], slice_points[s_idx])
            })
            .collect();

        Array2::from_shape_vec(grid_shape, data).unwrap()
    }

    /// Interpolates PDF values for multiple points in parallel using Chebyshev batch interpolation.
    ///
    /// # Arguments
    ///
    /// * `flavor_id` - The flavor ID.
    /// * `points` - A slice containing the collection of knots to interpolate on.
    ///   A knot is a collection of points containing `(nucleon, alphas, x, Q2)`.
    ///
    /// # Returns
    ///
    /// A `Vec<f64>` of interpolated PDF values.
    pub fn xfxq2_cheby_batch(&self, flavor_id: i32, points: &[&[f64]]) -> Result<Vec<f64>, Error> {
        if points.is_empty() {
            return Ok(Vec::new());
        }

        let pid_idx = match self.knot_array.pid_index(flavor_id) {
            Some(idx) => idx,
            None => return Ok(vec![0.0; points.len()]),
        };

        if !matches!(self.info.interpolator_type, InterpolatorType::LogChebyshev) {
            return Err(Error::InterpolationError(
                "xfxq2_cheby_batch only supports LogChebyshev interpolator".to_string(),
            ));
        }

        let mut subgrid_groups: HashMap<usize, Vec<(usize, &[f64])>> = HashMap::new();
        for (i, point) in points.iter().enumerate() {
            let subgrid_idx = self.knot_array.find_subgrid(point).ok_or_else(|| {
                let (x, q2) = self.get_x_q2(point);
                Error::SubgridNotFound { x, q2 }
            })?;

            subgrid_groups
                .entry(subgrid_idx)
                .or_default()
                .push((i, *point));
        }

        let mut all_results: Vec<(usize, f64)> = Vec::new();

        for (subgrid_idx, group) in subgrid_groups {
            let subgrid = &self.knot_array.subgrids[subgrid_idx];

            let (indices, group_points): (Vec<_>, Vec<_>) = group.into_iter().unzip();

            let log_points: Vec<Vec<f64>> = group_points
                .iter()
                .map(|p| p.iter().map(|&v| v.ln()).collect::<Vec<f64>>())
                .collect();

            let batch_interpolator =
                InterpolatorFactory::create_batch_interpolator(subgrid, pid_idx)
                    .map_err(Error::InterpolationError)?;

            let results = batch_interpolator
                .interpolate(log_points)
                .map_err(|e| Error::InterpolationError(e.to_string()))?;

            for (original_index, result) in indices.into_iter().zip(results) {
                all_results.push((original_index, result));
            }
        }

        // sort the results according to the original index
        all_results.sort_by_key(|&(i, _)| i);
        let final_results = all_results
            .into_iter()
            .map(|(_, r)| self.apply_force_positive(r))
            .collect();

        Ok(final_results)
    }

    /// Get the values of the momentum fraction `x` and momentum scale `Q2`.
    ///
    /// # Arguments
    ///
    /// * `points` - A slice where the last two elements are `x` and `q2`.
    ///
    /// # Returns
    ///
    /// A tuple containing the `x` and `q2` values.
    pub fn get_x_q2(&self, points: &[f64]) -> (f64, f64) {
        match points {
            [.., x, q2] => (*x, *q2),
            _ => panic!("The inputs must at least be x and Q2."),
        }
    }

    /// Gets the alpha_s value at a given `Q²`.
    ///
    /// # Arguments
    ///
    /// * `q2` - The energy scale squared `q2`.
    ///
    /// # Returns
    ///
    /// The interpolated alpha_s value.
    pub fn alphas_q2(&self, q2: f64) -> f64 {
        self.alphas.alphas_q2(q2)
    }

    /// Returns a reference to the PDF metadata.
    pub fn metadata(&self) -> &MetaData {
        &self.info
    }

    /// Gets the global parameter ranges for the entire PDF set.
    pub fn param_ranges(&self) -> RangeParameters {
        self.knot_array.global_ranges()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_array_creation() {
        let subgrid_data = vec![SubgridData {
            nucleons: vec![1.0],
            alphas: vec![0.118],
            kts: vec![0.0],
            xis: vec![0.0],
            deltas: vec![0.0],
            xs: vec![1.0, 2.0, 3.0],
            q2s: vec![4.0, 5.0],
            grid_data: vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
            ],
        }];
        let flavors = vec![21, 22];
        let grid_array = GridArray::new(subgrid_data, flavors);

        // Grid shape is 6D: [nucleons, alphas, pids, kT, x, Q²]
        match &grid_array.subgrids[0].grid {
            crate::subgrid::GridData::Grid6D(grid) => {
                assert_eq!(grid.shape(), &[1, 1, 2, 1, 3, 2]);
            }
            _ => std::panic!("Expected 6D grid"),
        }
        assert!(grid_array.find_subgrid(&[1.5, 4.5]).is_some());
    }
}
