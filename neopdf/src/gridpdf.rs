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
use super::interleaved::InterleavedHermite;
use super::interpolator::{DynInterpolator, InterpolationConfig, InterpolatorFactory};
use super::metadata::{InterpolatorType, MetaData};
use super::parser::SubgridData;
use super::strategy::ChebyshevAllPids;
use super::subgrid::{ParamRange, RangeParameters, SubGrid};

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

/// Direct-indexed PID lookup table. PIDs in range `[PID_MIN, PID_MAX]` are
/// resolved in O(1) via a flat array; out-of-range PIDs fall back to `None`.
const PID_MIN: i32 = -6;
const PID_MAX: i32 = 22;
const PID_RANGE: usize = (PID_MAX - PID_MIN + 1) as usize;
const PID_NONE: u8 = u8::MAX;

#[derive(Debug, Clone)]
struct PidLookup {
    table: [u8; PID_RANGE],
}

impl Default for PidLookup {
    fn default() -> Self {
        Self {
            table: [PID_NONE; PID_RANGE],
        }
    }
}

impl PidLookup {
    fn build(pids: &Array1<i32>) -> Self {
        let mut lut = Self::default();
        for (idx, &pid) in pids.iter().enumerate() {
            let normalized = if pid == 0 { 21 } else { pid };
            if (PID_MIN..=PID_MAX).contains(&normalized) {
                let slot = (normalized - PID_MIN) as usize;
                if lut.table[slot] == PID_NONE {
                    lut.table[slot] = idx as u8;
                }
            }
        }
        lut
    }

    #[inline(always)]
    fn get(&self, pid: i32) -> Option<usize> {
        let normalized = if pid == 0 { 21 } else { pid };
        if !(PID_MIN..=PID_MAX).contains(&normalized) {
            return None;
        }
        let v = self.table[(normalized - PID_MIN) as usize];
        if v == PID_NONE {
            None
        } else {
            Some(v as usize)
        }
    }
}

/// Stores the complete PDF grid data, including all subgrids and flavor information.
#[derive(Debug, Serialize)]
pub struct GridArray {
    /// An array of particle flavor IDs (PIDs).
    pub pids: Array1<i32>,
    /// A collection of `SubGrid` instances that make up the full grid.
    pub subgrids: Vec<SubGrid>,
    /// Direct-indexed PID lookup with complexity O(1).
    #[serde(skip)]
    pid_lookup: PidLookup,
}

impl<'de> Deserialize<'de> for GridArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            pids: Array1<i32>,
            subgrids: Vec<SubGrid>,
        }
        let h = Helper::deserialize(deserializer)?;
        let pid_lookup = PidLookup::build(&h.pids);
        Ok(Self {
            pids: h.pids,
            subgrids: h.subgrids,
            pid_lookup,
        })
    }
}

impl GridArray {
    /// Creates a `GridArray` from prebuilt pids and subgrids.
    pub fn from_parts(pids: Array1<i32>, subgrids: Vec<SubGrid>) -> Self {
        let pid_lookup = PidLookup::build(&pids);
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
        let pid_lookup = PidLookup::build(&pids);

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
    #[inline(always)]
    fn pid_index(&self, flavor_id: i32) -> Option<usize> {
        self.pid_lookup.get(flavor_id)
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
fn fp_identity(value: f64) -> f64 {
    value
}

fn fp_clip_negative(value: f64) -> f64 {
    value.max(0.0)
}

fn fp_clip_small(value: f64) -> f64 {
    value.max(1e-10)
}

/// Build an `InterleavedHermite` for a subgrid, if its config is supported.
fn build_interleaved(
    subgrid: &SubGrid,
    config: InterpolationConfig,
    n_pids: usize,
) -> Option<InterleavedHermite> {
    let log_xs: Vec<f64> = subgrid.xs.iter().map(|&x| x.ln()).collect();
    let log_q2s: Vec<f64> = subgrid.q2s.iter().map(|&q| q.ln()).collect();

    match config {
        InterpolationConfig::TwoD => {
            let extra_grids = vec![log_q2s];
            let grid = subgrid.grid.view();
            let is_8d = subgrid.is_8d();
            let interleaved =
                InterleavedHermite::build(log_xs, extra_grids, n_pids, |pid, x_idx, extra| {
                    let q2_idx = extra[0];
                    if is_8d {
                        grid[[0, 0, 0, 0, 0, pid, x_idx, q2_idx]]
                    } else {
                        grid[[0, 0, pid, 0, x_idx, q2_idx]]
                    }
                });
            Some(interleaved)
        }
        InterpolationConfig::ThreeDNucleons => {
            let log_nucs: Vec<f64> = subgrid.nucleons.iter().map(|&v| v.ln()).collect();
            let extra_grids = vec![log_q2s, log_nucs];
            let grid = subgrid.grid.view();
            let interleaved =
                InterleavedHermite::build(log_xs, extra_grids, n_pids, |pid, x_idx, extra| {
                    let q2_idx = extra[0];
                    let nuc_idx = extra[1];
                    grid[[nuc_idx, 0, pid, 0, x_idx, q2_idx]]
                });
            Some(interleaved)
        }
        InterpolationConfig::ThreeDAlphas => {
            let log_alp: Vec<f64> = subgrid.alphas.iter().map(|&v| v.ln()).collect();
            let extra_grids = vec![log_q2s, log_alp];
            let grid = subgrid.grid.view();
            let interleaved =
                InterleavedHermite::build(log_xs, extra_grids, n_pids, |pid, x_idx, extra| {
                    let q2_idx = extra[0];
                    let alp_idx = extra[1];
                    grid[[0, alp_idx, pid, 0, x_idx, q2_idx]]
                });
            Some(interleaved)
        }
        InterpolationConfig::ThreeDKt => {
            let log_kts: Vec<f64> = subgrid.kts.iter().map(|&v| v.ln()).collect();
            let extra_grids = vec![log_q2s, log_kts];
            let grid = subgrid.grid.view();
            let interleaved =
                InterleavedHermite::build(log_xs, extra_grids, n_pids, |pid, x_idx, extra| {
                    let q2_idx = extra[0];
                    let kt_idx = extra[1];
                    grid[[0, 0, pid, kt_idx, x_idx, q2_idx]]
                });
            Some(interleaved)
        }
        InterpolationConfig::ThreeDXi => {
            let log_xis: Vec<f64> = subgrid.xis.iter().map(|&v| v.ln()).collect();
            let extra_grids = vec![log_q2s, log_xis];
            let grid = subgrid.grid.view();
            let interleaved =
                InterleavedHermite::build(log_xs, extra_grids, n_pids, |pid, x_idx, extra| {
                    let q2_idx = extra[0];
                    let xi_idx = extra[1];
                    grid[[0, 0, xi_idx, 0, 0, pid, x_idx, q2_idx]]
                });
            Some(interleaved)
        }
        InterpolationConfig::ThreeDDelta => {
            let log_del: Vec<f64> = subgrid.deltas.iter().map(|&v| v.ln()).collect();
            let extra_grids = vec![log_q2s, log_del];
            let grid = subgrid.grid.view();
            let interleaved =
                InterleavedHermite::build(log_xs, extra_grids, n_pids, |pid, x_idx, extra| {
                    let q2_idx = extra[0];
                    let del_idx = extra[1];
                    grid[[0, 0, 0, del_idx, 0, pid, x_idx, q2_idx]]
                });
            Some(interleaved)
        }
        InterpolationConfig::FourDNucleonsAlphas => {
            let log_nucs: Vec<f64> = subgrid.nucleons.iter().map(|&v| v.ln()).collect();
            let log_alp: Vec<f64> = subgrid.alphas.iter().map(|&v| v.ln()).collect();
            let extra_grids = vec![log_q2s, log_alp, log_nucs];
            let grid = subgrid.grid.view();
            let interleaved =
                InterleavedHermite::build(log_xs, extra_grids, n_pids, |pid, x_idx, extra| {
                    let q2_idx = extra[0];
                    let alp_idx = extra[1];
                    let nuc_idx = extra[2];
                    grid[[nuc_idx, alp_idx, pid, 0, x_idx, q2_idx]]
                });
            Some(interleaved)
        }
        InterpolationConfig::FourDNucleonsKt => {
            let log_nucs: Vec<f64> = subgrid.nucleons.iter().map(|&v| v.ln()).collect();
            let log_kts: Vec<f64> = subgrid.kts.iter().map(|&v| v.ln()).collect();
            let extra_grids = vec![log_q2s, log_kts, log_nucs];
            let grid = subgrid.grid.view();
            let interleaved =
                InterleavedHermite::build(log_xs, extra_grids, n_pids, |pid, x_idx, extra| {
                    let q2_idx = extra[0];
                    let kt_idx = extra[1];
                    let nuc_idx = extra[2];
                    grid[[nuc_idx, 0, pid, kt_idx, x_idx, q2_idx]]
                });
            Some(interleaved)
        }
        InterpolationConfig::FourDAlphasKt => {
            let log_alp: Vec<f64> = subgrid.alphas.iter().map(|&v| v.ln()).collect();
            let log_kts: Vec<f64> = subgrid.kts.iter().map(|&v| v.ln()).collect();
            let extra_grids = vec![log_q2s, log_kts, log_alp];
            let grid = subgrid.grid.view();
            let interleaved =
                InterleavedHermite::build(log_xs, extra_grids, n_pids, |pid, x_idx, extra| {
                    let q2_idx = extra[0];
                    let kt_idx = extra[1];
                    let alp_idx = extra[2];
                    grid[[0, alp_idx, pid, kt_idx, x_idx, q2_idx]]
                });
            Some(interleaved)
        }
        InterpolationConfig::FourDXiDelta => {
            let log_xis: Vec<f64> = subgrid.xis.iter().map(|&v| v.ln()).collect();
            let log_del: Vec<f64> = subgrid.deltas.iter().map(|&v| v.ln()).collect();
            let extra_grids = vec![log_q2s, log_del, log_xis];
            let grid = subgrid.grid.view();
            let interleaved =
                InterleavedHermite::build(log_xs, extra_grids, n_pids, |pid, x_idx, extra| {
                    let q2_idx = extra[0];
                    let del_idx = extra[1];
                    let xi_idx = extra[2];
                    grid[[0, 0, xi_idx, del_idx, 0, pid, x_idx, q2_idx]]
                });
            Some(interleaved)
        }
        InterpolationConfig::FiveD => {
            let log_xis: Vec<f64> = subgrid.xis.iter().map(|&v| v.ln()).collect();
            let log_del: Vec<f64> = subgrid.deltas.iter().map(|&v| v.ln()).collect();
            let log_kts: Vec<f64> = subgrid.kts.iter().map(|&v| v.ln()).collect();

            // Re-order [kT, xi, delta, x, Q2] into [Q2, delta, xi, kT] (innermost first)
            let extra_grids = vec![log_q2s, log_del, log_xis, log_kts];
            let grid = subgrid.grid.view();
            let interleaved =
                InterleavedHermite::build(log_xs, extra_grids, n_pids, |pid, x_idx, extra| {
                    let q2_idx = extra[0];
                    let del_idx = extra[1];
                    let xi_idx = extra[2];
                    let kt_idx = extra[3];
                    grid[[0, 0, xi_idx, del_idx, kt_idx, pid, x_idx, q2_idx]]
                });
            Some(interleaved)
        }
        // `SixD`, `SevenD`, and non-cubic configs are not supported.
        _ => None,
    }
}

/// Build a `ChebyshevAllPids` for a subgrid, if its config is supported.
fn build_chebyshev_fast(
    subgrid: &SubGrid,
    config: InterpolationConfig,
    n_pids: usize,
) -> Option<ChebyshevAllPids> {
    let log_xs: Vec<f64> = subgrid.xs.iter().map(|&x| x.ln()).collect();
    let log_q2s: Vec<f64> = subgrid.q2s.iter().map(|&q| q.ln()).collect();

    match config {
        InterpolationConfig::TwoD => {
            let coords = vec![log_xs, log_q2s];
            let grid = subgrid.grid.view();
            let is_8d = subgrid.is_8d();
            Some(ChebyshevAllPids::build(coords, n_pids, |pid, idx| {
                if is_8d {
                    grid[[0, 0, 0, 0, 0, pid, idx[0], idx[1]]]
                } else {
                    grid[[0, 0, pid, 0, idx[0], idx[1]]]
                }
            }))
        }
        InterpolationConfig::ThreeDNucleons => {
            let log_nucs: Vec<f64> = subgrid.nucleons.iter().map(|&v| v.ln()).collect();
            let coords = vec![log_nucs, log_xs, log_q2s];
            let grid = subgrid.grid.view();
            Some(ChebyshevAllPids::build(coords, n_pids, |pid, idx| {
                grid[[idx[0], 0, pid, 0, idx[1], idx[2]]]
            }))
        }
        InterpolationConfig::ThreeDAlphas => {
            let log_alps: Vec<f64> = subgrid.alphas.iter().map(|&v| v.ln()).collect();
            let coords = vec![log_alps, log_xs, log_q2s];
            let grid = subgrid.grid.view();
            Some(ChebyshevAllPids::build(coords, n_pids, |pid, idx| {
                grid[[0, idx[0], pid, 0, idx[1], idx[2]]]
            }))
        }
        InterpolationConfig::ThreeDKt => {
            let log_kts: Vec<f64> = subgrid.kts.iter().map(|&v| v.ln()).collect();
            let coords = vec![log_kts, log_xs, log_q2s];
            let grid = subgrid.grid.view();
            Some(ChebyshevAllPids::build(coords, n_pids, |pid, idx| {
                grid[[0, 0, pid, idx[0], idx[1], idx[2]]]
            }))
        }
        InterpolationConfig::ThreeDXi => {
            let log_xis: Vec<f64> = subgrid.xis.iter().map(|&v| v.ln()).collect();
            let coords = vec![log_xis, log_xs, log_q2s];
            let grid = subgrid.grid.view();
            Some(ChebyshevAllPids::build(coords, n_pids, |pid, idx| {
                grid[[0, 0, idx[0], 0, 0, pid, idx[1], idx[2]]]
            }))
        }
        InterpolationConfig::ThreeDDelta => {
            let log_dels: Vec<f64> = subgrid.deltas.iter().map(|&v| v.ln()).collect();
            let coords = vec![log_dels, log_xs, log_q2s];
            let grid = subgrid.grid.view();
            Some(ChebyshevAllPids::build(coords, n_pids, |pid, idx| {
                grid[[0, 0, 0, idx[0], 0, pid, idx[1], idx[2]]]
            }))
        }
        InterpolationConfig::FourDNucleonsAlphas => {
            let coords = vec![
                subgrid.nucleons.to_vec(),
                subgrid.alphas.to_vec(),
                subgrid.xs.to_vec(),
                subgrid.q2s.to_vec(),
            ];
            let grid = subgrid.grid.view();
            Some(ChebyshevAllPids::build(coords, n_pids, |pid, idx| {
                grid[[idx[0], idx[1], pid, 0, idx[2], idx[3]]]
            }))
        }
        InterpolationConfig::FourDNucleonsKt => {
            let log_nucs: Vec<f64> = subgrid.nucleons.iter().map(|&v| v.ln()).collect();
            let log_kts: Vec<f64> = subgrid.kts.iter().map(|&v| v.ln()).collect();
            let coords = vec![log_nucs, log_kts, log_xs, log_q2s];
            let grid = subgrid.grid.view();
            Some(ChebyshevAllPids::build(coords, n_pids, |pid, idx| {
                grid[[idx[0], 0, pid, idx[1], idx[2], idx[3]]]
            }))
        }
        InterpolationConfig::FourDAlphasKt => {
            let log_alps: Vec<f64> = subgrid.alphas.iter().map(|&v| v.ln()).collect();
            let log_kts: Vec<f64> = subgrid.kts.iter().map(|&v| v.ln()).collect();
            let coords = vec![log_alps, log_kts, log_xs, log_q2s];
            let grid = subgrid.grid.view();
            Some(ChebyshevAllPids::build(coords, n_pids, |pid, idx| {
                grid[[0, idx[0], pid, idx[1], idx[2], idx[3]]]
            }))
        }
        InterpolationConfig::FourDXiDelta => {
            let log_xis: Vec<f64> = subgrid.xis.iter().map(|&v| v.ln()).collect();
            let log_dels: Vec<f64> = subgrid.deltas.iter().map(|&v| v.ln()).collect();
            let coords = vec![log_xis, log_dels, log_xs, log_q2s];
            let grid = subgrid.grid.view();
            Some(ChebyshevAllPids::build(coords, n_pids, |pid, idx| {
                grid[[0, 0, idx[0], idx[1], 0, pid, idx[2], idx[3]]]
            }))
        }
        InterpolationConfig::FiveD => {
            let log_kts: Vec<f64> = subgrid.kts.iter().map(|&v| v.ln()).collect();
            let log_xis: Vec<f64> = subgrid.xis.iter().map(|&v| v.ln()).collect();
            let log_dels: Vec<f64> = subgrid.deltas.iter().map(|&v| v.ln()).collect();
            let coords = vec![log_kts, log_xis, log_dels, log_xs, log_q2s];
            let grid = subgrid.grid.view();
            Some(ChebyshevAllPids::build(coords, n_pids, |pid, idx| {
                grid[[0, 0, idx[1], idx[2], idx[0], pid, idx[3], idx[4]]]
            }))
        }
        _ => None,
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
    /// Optional fast path for all-flavor evaluation (cubic Hermite, 2D–5D).
    interleaved: Option<Vec<InterleavedHermite>>,
    /// Optional fast path for all-flavor evaluation (Chebyshev, 2D–5D).
    chebyshev_fast: Option<Vec<ChebyshevAllPids>>,
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

        // Build interleaved coefficients for cubic Hermite grids (2D–5D)
        let interleaved = if matches!(
            info.interpolator_type,
            InterpolatorType::LogBicubic
                | InterpolatorType::LogTricubic
                | InterpolatorType::LogFourCubic
                | InterpolatorType::LogFiveCubic
        ) {
            let built: Vec<Option<InterleavedHermite>> = knot_array
                .subgrids
                .iter()
                .map(|sg| build_interleaved(sg, sg.interpolation_config(), knot_array.pids.len()))
                .collect();
            if built.iter().all(|o| o.is_some()) {
                Some(built.into_iter().map(|o| o.unwrap()).collect())
            } else {
                None
            }
        } else {
            None
        };

        // Build fast path for Chebyshev grids (2D–5D)
        let chebyshev_fast = if info.interpolator_type == InterpolatorType::LogChebyshev {
            let built: Vec<Option<ChebyshevAllPids>> = knot_array
                .subgrids
                .iter()
                .map(|sg| {
                    build_chebyshev_fast(sg, sg.interpolation_config(), knot_array.pids.len())
                })
                .collect();
            if built.iter().all(|o| o.is_some()) {
                Some(built.into_iter().map(|o| o.unwrap()).collect())
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
            chebyshev_fast,
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
    /// Avoids `map_err` string allocation.
    pub(crate) fn xfxq2_fast(&self, flavor_id: i32, points: &[f64]) -> f64 {
        let subgrid_idx = match self.knot_array.find_subgrid(points) {
            Some(idx) => idx,
            None => return 0.0,
        };

        let pid_idx = match self.knot_array.pid_index(flavor_id) {
            Some(idx) => idx,
            None => return 0.0,
        };

        // Fast path: interleaved Hermite coefficients. This bypasses vtable dispatch, ninterp
        // validation, and Result wrapping.
        if let Some(ref il) = self.interleaved {
            if let Some(val) = il[subgrid_idx].eval_single_fast(pid_idx, points) {
                return (self.force_positive_fn)(val);
            }
        }

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
        let pid_slots: Vec<Option<usize>> = pids
            .iter()
            .map(|&pid| self.knot_array.pid_index(pid))
            .collect();
        self.xfxq2_allpids_with_slots(&pid_slots, points, out);
    }

    /// Internal fast path using pre-resolved PID slots.
    pub(crate) fn xfxq2_allpids_with_slots(
        &self,
        pid_slots: &[Option<usize>],
        points: &[f64],
        out: &mut [f64],
    ) {
        let subgrid_idx = match self.knot_array.find_subgrid(points) {
            Some(idx) => idx,
            None => {
                out.iter_mut().for_each(|v| *v = 0.0);
                return;
            }
        };

        // Fast path: interleaved Hermite coefficients (2D–5D)
        if let Some(ref il) = self.interleaved {
            let il = &il[subgrid_idx];
            let loc = match il.locate(points) {
                Some(l) => l,
                None => {
                    out.iter_mut().for_each(|v| *v = 0.0);
                    return;
                }
            };

            il.eval_allpids(&loc, pid_slots, self.force_positive_fn, out);
            return;
        }

        // Fast path: Chebyshev — compute barycentric coefficients once, loop over PIDs
        if let Some(ref cf) = self.chebyshev_fast {
            let cf = &cf[subgrid_idx];
            let mut buf = [0.0f64; 8];
            for (i, &p) in points.iter().enumerate() {
                buf[i] = if self.use_log { p.ln() } else { p };
            }
            let log_points = &buf[..points.len()];

            let loc = cf.locate(log_points);
            cf.eval_allpids(&loc, pid_slots, self.force_positive_fn, out);
            return;
        }

        // Generic fallback
        let mut buf = [0.0f64; 8];
        for (i, &p) in points.iter().enumerate() {
            buf[i] = if self.use_log { p.ln() } else { p };
        }
        let log_points = &buf[..points.len()];

        for (o, slot) in out.iter_mut().zip(pid_slots.iter()) {
            *o = match *slot {
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
        let n_pids = flavors.len();
        let n_points = slice_points.len();

        let pid_slots: Vec<Option<usize>> = flavors
            .iter()
            .map(|&pid| self.knot_array.pid_index(pid))
            .collect();

        let mut results = Array2::<f64>::zeros((n_pids, n_points));
        let mut out_buf = vec![0.0; n_pids];

        for (j, point) in slice_points.iter().enumerate() {
            self.xfxq2_allpids_with_slots(&pid_slots, point, &mut out_buf);
            for i in 0..n_pids {
                results[[i, j]] = out_buf[i];
            }
        }
        results
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
