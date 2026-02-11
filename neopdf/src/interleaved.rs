//! Generalized interleaved Hermite interpolation for 2D–5D PDF grids.
//!
//! This module is self-contained: it depends only on `super::utils` for binary search
//! and the Hermite cubic basis function. All grid layout and config awareness lives in
//! `gridpdf.rs`, which supplies a value-extraction closure at build time.

use super::utils;

/// Maximum number of "extra" dimensions beyond x (Q2 + up to 3 outer dims covers 5D).
const MAX_EXTRA_DIMS: usize = 4;

/// Result of binary-searching all dimensions.
pub(crate) struct Location {
    pub ix: usize,
    pub u_x: f64,
    pub extra_indices: [usize; MAX_EXTRA_DIMS],
    pub extra_ts: [f64; MAX_EXTRA_DIMS],
}

/// Generalized interleaved Hermite interpolation structure.
///
/// The x-polynomial coefficients are precomputed and stored in interleaved
/// `[cell][flavor][4]` layout, where `cell` is the linearized index over
/// all extra dimensions and x-intervals. Evaluation in dimensions beyond x
/// is done recursively via `eval_level`.
pub(crate) struct InterleavedHermite {
    /// Flat coefficient array: `[(cell * n_flavors + flavor) * 4 + c]`
    /// where `cell = x_cell * cells_per_x + extra_linear_index`.
    coeffs: Vec<f64>,
    /// Log-transformed x grid.
    log_xs: Vec<f64>,
    /// Log-transformed grids for extra dimensions, ordered [Q2, outer1, outer2, ...].
    extra_grids: Vec<Vec<f64>>,
    /// Stride per extra dimension in the cell index.
    extra_strides: Vec<usize>,
    /// Product of all extra dimension sizes (number of cells per x-interval).
    cells_per_x: usize,
    /// Number of extra dimensions.
    n_extra: usize,
    /// Number of flavors stored.
    n_flavors: usize,
}

/// Compute the x-derivative at knot `ix` using central differences.
///
/// Same algorithm as `LogBicubicInterpolation::calculate_ddx`:
/// - Interior knots: average of left and right finite differences
/// - Boundary knots: one-sided finite difference
fn compute_x_derivative(log_xs: &[f64], values: &[f64], ix: usize) -> f64 {
    let n = log_xs.len();
    let del1 = if ix > 0 {
        log_xs[ix] - log_xs[ix - 1]
    } else {
        0.0
    };
    let del2 = if ix < n - 1 {
        log_xs[ix + 1] - log_xs[ix]
    } else {
        0.0
    };

    if ix > 0 && ix < n - 1 {
        let lddx = (values[ix] - values[ix - 1]) / del1;
        let rddx = (values[ix + 1] - values[ix]) / del2;
        (lddx + rddx) / 2.0
    } else if ix == 0 {
        (values[ix + 1] - values[ix]) / del2
    } else {
        // ix == n - 1
        (values[ix] - values[ix - 1]) / del1
    }
}

impl InterleavedHermite {
    /// Build interleaved x-polynomial coefficients.
    ///
    /// # Arguments
    ///
    /// * `log_xs` — log-transformed x grid knots
    /// * `extra_grids` — log-transformed grids for extra dims, ordered `[Q2, outer1, ...]`
    /// * `n_flavors` — number of flavors (PIDs)
    /// * `value_at` — closure `(flavor, x_idx, extra_indices) -> f64` that extracts
    ///   the grid value in a config-agnostic way. `extra_indices` is ordered to match
    ///   `extra_grids`: `[q2_idx, outer1_idx, ...]`.
    pub fn build<F>(
        log_xs: Vec<f64>,
        extra_grids: Vec<Vec<f64>>,
        n_flavors: usize,
        value_at: F,
    ) -> Self
    where
        F: Fn(usize, usize, &[usize]) -> f64,
    {
        let n_extra = extra_grids.len();
        let nx = log_xs.len();

        // Compute strides and total cells per x-interval
        let extra_sizes: Vec<usize> = extra_grids.iter().map(|g| g.len()).collect();
        let mut extra_strides = vec![0usize; n_extra];
        let mut cells_per_x = 1usize;
        for i in 0..n_extra {
            extra_strides[i] = cells_per_x;
            cells_per_x *= extra_sizes[i];
        }

        let n_x_cells = nx - 1;
        let total_cells = n_x_cells * cells_per_x;
        let mut coeffs = vec![0.0f64; total_cells * n_flavors * 4];

        // Temporary buffer for x-values at a fixed (flavor, extra_indices) combination
        let mut x_vals = vec![0.0f64; nx];
        let mut extra_idx_buf = vec![0usize; n_extra];

        for flavor in 0..n_flavors {
            // Iterate over all combinations of extra dimension indices
            for extra_linear in 0..cells_per_x {
                // Decode linear index to per-dimension indices
                let mut remaining = extra_linear;
                for dim in 0..n_extra {
                    extra_idx_buf[dim] = remaining % extra_sizes[dim];
                    remaining /= extra_sizes[dim];
                }

                // Gather x-values for this (flavor, extra) combination
                for (ix, val) in x_vals.iter_mut().enumerate() {
                    *val = value_at(flavor, ix, &extra_idx_buf);
                }

                // Compute polynomial coefficients for each x-interval
                for ix in 0..n_x_cells {
                    let dx = log_xs[ix + 1] - log_xs[ix];
                    let vl = x_vals[ix];
                    let vh = x_vals[ix + 1];
                    let vdl = compute_x_derivative(&log_xs, &x_vals, ix) * dx;
                    let vdh = compute_x_derivative(&log_xs, &x_vals, ix + 1) * dx;

                    let a = vdh + vdl - 2.0 * vh + 2.0 * vl;
                    let b = 3.0 * vh - 3.0 * vl - 2.0 * vdl - vdh;
                    let c = vdl;
                    let d = vl;

                    let cell = ix * cells_per_x + extra_linear;
                    let base = (cell * n_flavors + flavor) * 4;
                    coeffs[base] = a;
                    coeffs[base + 1] = b;
                    coeffs[base + 2] = c;
                    coeffs[base + 3] = d;
                }
            }
        }

        Self {
            coeffs,
            log_xs,
            extra_grids,
            extra_strides,
            cells_per_x,
            n_extra,
            n_flavors,
        }
    }

    /// Binary-search all dimensions and return a `Location`.
    ///
    /// `points` is ordered `[outer_K, ..., outer_1, x, Q2]` (same as the public API).
    /// Maps: x = points[n-2], Q2 = points[n-1], outer dims = points[0..n-2] reversed
    /// into extra_grids order `[Q2, outer1, outer2, ...]`.
    pub fn locate(&self, points: &[f64]) -> Option<Location> {
        let n = points.len();
        // Validate dimensionality: expect 1 (x) + n_extra dimensions
        if n != self.n_extra + 1 {
            return None;
        }
        let lx = points[n - 2].ln();
        let lq2 = points[n - 1].ln();

        let ix = utils::find_interval_index(&self.log_xs, lx).ok()?;
        let dx = self.log_xs[ix + 1] - self.log_xs[ix];
        let u_x = (lx - self.log_xs[ix]) / dx;

        let mut extra_indices = [0usize; MAX_EXTRA_DIMS];
        let mut extra_ts = [0.0f64; MAX_EXTRA_DIMS];

        // extra_grids[0] = Q2
        {
            let grid = &self.extra_grids[0];
            let idx = utils::find_interval_index(grid, lq2).ok()?;
            let d = grid[idx + 1] - grid[idx];
            extra_indices[0] = idx;
            extra_ts[0] = (lq2 - grid[idx]) / d;
        }

        // extra_grids[1..] = outer dims, mapped from points[0..n-2] in reverse
        for k in 1..self.n_extra {
            let point_idx = n - 3 - (k - 1); // reversed: outermost point first
            let log_val = points[point_idx].ln();
            let grid = &self.extra_grids[k];
            let idx = utils::find_interval_index(grid, log_val).ok()?;
            let d = grid[idx + 1] - grid[idx];
            extra_indices[k] = idx;
            extra_ts[k] = (log_val - grid[idx]) / d;
        }

        Some(Location {
            ix,
            u_x,
            extra_indices,
            extra_ts,
        })
    }

    /// Evaluate a single flavor at the given points.
    #[inline]
    #[cfg(test)]
    pub fn eval_single(&self, flavor: usize, points: &[f64]) -> f64 {
        match self.locate(points) {
            Some(loc) => self.eval_at(&loc, flavor),
            None => 0.0,
        }
    }

    /// Fast single-flavor evaluation for 2D grids.
    /// Returns `None` if out of bounds or wrong dimensionality.
    #[inline]
    pub fn eval_single_fast(&self, flavor: usize, points: &[f64]) -> Option<f64> {
        if points.len() != self.n_extra + 1 {
            return None;
        }
        let lx = points[points.len() - 2].ln();
        let lq2 = points[points.len() - 1].ln();
        let ix = utils::find_interval_index(&self.log_xs, lx).ok()?;
        let dx = self.log_xs[ix + 1] - self.log_xs[ix];
        let u_x = (lx - self.log_xs[ix]) / dx;
        let cell_base = ix * self.cells_per_x;

        if self.n_extra == 1 {
            let grid = &self.extra_grids[0];
            let iq2 = utils::find_interval_index(grid, lq2).ok()?;
            let d = grid[iq2 + 1] - grid[iq2];
            let v = (lq2 - grid[iq2]) / d;
            Some(self.eval_q2_inline(cell_base, flavor, u_x, iq2, v))
        } else {
            let loc = self.locate(points)?;
            Some(self.eval_level(self.n_extra, cell_base, flavor, u_x, &loc))
        }
    }

    /// Evaluate a single flavor at a pre-computed `Location`.
    #[inline]
    #[cfg(test)]
    pub fn eval_at(&self, loc: &Location, flavor: usize) -> f64 {
        let cell_base = loc.ix * self.cells_per_x;
        if self.n_extra == 1 {
            self.eval_q2_inline(
                cell_base,
                flavor,
                loc.u_x,
                loc.extra_indices[0],
                loc.extra_ts[0],
            )
        } else {
            self.eval_level(self.n_extra, cell_base, flavor, loc.u_x, loc)
        }
    }

    /// Evaluate all requested flavors at a pre-computed `Location`.
    pub fn eval_allpids(
        &self,
        loc: &Location,
        pid_slots: &[Option<usize>],
        force_positive_fn: fn(f64) -> f64,
        out: &mut [f64],
    ) {
        let cell_base = loc.ix * self.cells_per_x;
        if self.n_extra == 1 {
            // Specialized 2D path: avoid recursion overhead
            let iq2 = loc.extra_indices[0];
            let v = loc.extra_ts[0];
            let u = loc.u_x;
            let nq2 = self.cells_per_x; // cells_per_x == nq2 for 2D
            let log_q2s = &self.extra_grids[0];
            let dq_1 = log_q2s[iq2 + 1] - log_q2s[iq2];

            for (o, slot) in out.iter_mut().zip(pid_slots.iter()) {
                let fi = match *slot {
                    Some(idx) => idx,
                    None => {
                        *o = 0.0;
                        continue;
                    }
                };

                let cell_lo = cell_base + iq2;
                let vl = self.hermite_x(cell_lo, fi, u);
                let vh = self.hermite_x(cell_lo + 1, fi, u);

                let (vdl, vdh) = if iq2 == 0 {
                    let vdl_val = vh - vl;
                    if nq2 > 2 {
                        let vhh = self.hermite_x(cell_lo + 2, fi, u);
                        let dq_2_inv = 1.0 / (log_q2s[iq2 + 2] - log_q2s[iq2 + 1]);
                        let vdh_val = (vdl_val + (vhh - vh) * dq_1 * dq_2_inv) * 0.5;
                        (vdl_val, vdh_val)
                    } else {
                        (vdl_val, vh - vl)
                    }
                } else if iq2 == nq2 - 2 {
                    let vdh_val = vh - vl;
                    if nq2 > 2 {
                        let vll = self.hermite_x(cell_lo - 1, fi, u);
                        let dq_0_inv = 1.0 / (log_q2s[iq2] - log_q2s[iq2 - 1]);
                        let vdl_val = (vdh_val + (vl - vll) * dq_1 * dq_0_inv) * 0.5;
                        (vdl_val, vdh_val)
                    } else {
                        (vh - vl, vdh_val)
                    }
                } else {
                    let vll = self.hermite_x(cell_lo - 1, fi, u);
                    let dq_0_inv = 1.0 / (log_q2s[iq2] - log_q2s[iq2 - 1]);
                    let vhh = self.hermite_x(cell_lo + 2, fi, u);
                    let dq_2_inv = 1.0 / (log_q2s[iq2 + 2] - log_q2s[iq2 + 1]);
                    let vdl_val = ((vh - vl) + (vl - vll) * dq_1 * dq_0_inv) * 0.5;
                    let vdh_val = ((vh - vl) + (vhh - vh) * dq_1 * dq_2_inv) * 0.5;
                    (vdl_val, vdh_val)
                };

                *o = force_positive_fn(utils::hermite_cubic_interpolate(v, vl, vdl, vh, vdh));
            }
        } else {
            for (o, slot) in out.iter_mut().zip(pid_slots.iter()) {
                match *slot {
                    Some(fi) => {
                        *o = force_positive_fn(self.eval_level(
                            self.n_extra,
                            cell_base,
                            fi,
                            loc.u_x,
                            loc,
                        ));
                    }
                    None => *o = 0.0,
                }
            }
        }
    }

    /// Specialized 2D evaluation: Q2 Hermite interpolation without recursion.
    #[inline(always)]
    fn eval_q2_inline(&self, cell_base: usize, flavor: usize, u: f64, iq2: usize, v: f64) -> f64 {
        let nq2 = self.cells_per_x;
        let log_q2s = &self.extra_grids[0];
        let dq_1 = log_q2s[iq2 + 1] - log_q2s[iq2];

        let cell_lo = cell_base + iq2;
        let vl = self.hermite_x(cell_lo, flavor, u);
        let vh = self.hermite_x(cell_lo + 1, flavor, u);

        let (vdl, vdh) = if iq2 == 0 {
            let vdl_val = vh - vl;
            if nq2 > 2 {
                let vhh = self.hermite_x(cell_lo + 2, flavor, u);
                let dq_2_inv = 1.0 / (log_q2s[iq2 + 2] - log_q2s[iq2 + 1]);
                let vdh_val = (vdl_val + (vhh - vh) * dq_1 * dq_2_inv) * 0.5;
                (vdl_val, vdh_val)
            } else {
                (vdl_val, vh - vl)
            }
        } else if iq2 == nq2 - 2 {
            let vdh_val = vh - vl;
            if nq2 > 2 {
                let vll = self.hermite_x(cell_lo - 1, flavor, u);
                let dq_0_inv = 1.0 / (log_q2s[iq2] - log_q2s[iq2 - 1]);
                let vdl_val = (vdh_val + (vl - vll) * dq_1 * dq_0_inv) * 0.5;
                (vdl_val, vdh_val)
            } else {
                (vh - vl, vdh_val)
            }
        } else {
            let vll = self.hermite_x(cell_lo - 1, flavor, u);
            let dq_0_inv = 1.0 / (log_q2s[iq2] - log_q2s[iq2 - 1]);
            let vhh = self.hermite_x(cell_lo + 2, flavor, u);
            let dq_2_inv = 1.0 / (log_q2s[iq2 + 2] - log_q2s[iq2 + 1]);
            let vdl_val = ((vh - vl) + (vl - vll) * dq_1 * dq_0_inv) * 0.5;
            let vdh_val = ((vh - vl) + (vhh - vh) * dq_1 * dq_2_inv) * 0.5;
            (vdl_val, vdh_val)
        };

        utils::hermite_cubic_interpolate(v, vl, vdl, vh, vdh)
    }

    /// Evaluate the precomputed x-polynomial at a given cell and flavor.
    #[inline(always)]
    fn hermite_x(&self, cell: usize, flavor: usize, u: f64) -> f64 {
        let base = (cell * self.n_flavors + flavor) * 4;
        let c = &self.coeffs[base..base + 4];
        let u2 = u * u;
        let u3 = u2 * u;
        c[0] * u3 + c[1] * u2 + c[2] * u + c[3]
    }

    /// Recursive evaluation through extra dimensions.
    ///
    /// `level = 0` evaluates the x-polynomial (base case).
    /// `level = k` performs Hermite interpolation in `extra_grids[k-1]` using
    /// ~4 evaluations of `eval_level(k-1, ...)`.
    #[inline(always)]
    fn eval_level(
        &self,
        level: usize,
        cell_base: usize,
        flavor: usize,
        u_x: f64,
        loc: &Location,
    ) -> f64 {
        if level == 0 {
            return self.hermite_x(cell_base, flavor, u_x);
        }

        let dim = level - 1; // index into extra_grids
        let grid = &self.extra_grids[dim];
        let n_knots = grid.len();
        let idx = loc.extra_indices[dim];
        let t = loc.extra_ts[dim];
        let stride = self.extra_strides[dim];

        // Evaluate the inner dimension at idx and idx+1
        let cell_lo = cell_base + idx * stride;
        let cell_hi = cell_base + (idx + 1) * stride;
        let vl = self.eval_level(level - 1, cell_lo, flavor, u_x, loc);
        let vh = self.eval_level(level - 1, cell_hi, flavor, u_x, loc);

        // Derivative estimation using the same central-difference scheme
        let dq = grid[idx + 1] - grid[idx];

        let (vdl, vdh) = if idx == 0 {
            // Forward difference for low edge
            let vdl_val = vh - vl;
            if n_knots > 2 {
                let cell_hh = cell_base + (idx + 2) * stride;
                let vhh = self.eval_level(level - 1, cell_hh, flavor, u_x, loc);
                let dq2_inv = 1.0 / (grid[idx + 2] - grid[idx + 1]);
                let vdh_val = (vdl_val + (vhh - vh) * dq * dq2_inv) * 0.5;
                (vdl_val, vdh_val)
            } else {
                (vdl_val, vh - vl)
            }
        } else if idx == n_knots - 2 {
            // Backward difference for high edge
            let vdh_val = vh - vl;
            if n_knots > 2 {
                let cell_ll = cell_base + (idx - 1) * stride;
                let vll = self.eval_level(level - 1, cell_ll, flavor, u_x, loc);
                let dq0_inv = 1.0 / (grid[idx] - grid[idx - 1]);
                let vdl_val = (vdh_val + (vl - vll) * dq * dq0_inv) * 0.5;
                (vdl_val, vdh_val)
            } else {
                (vh - vl, vdh_val)
            }
        } else {
            // Central difference
            let cell_ll = cell_base + (idx - 1) * stride;
            let cell_hh = cell_base + (idx + 2) * stride;
            let vll = self.eval_level(level - 1, cell_ll, flavor, u_x, loc);
            let vhh = self.eval_level(level - 1, cell_hh, flavor, u_x, loc);
            let dq0_inv = 1.0 / (grid[idx] - grid[idx - 1]);
            let dq2_inv = 1.0 / (grid[idx + 2] - grid[idx + 1]);
            let vdl_val = ((vh - vl) + (vl - vll) * dq * dq0_inv) * 0.5;
            let vdh_val = ((vh - vl) + (vhh - vh) * dq * dq2_inv) * 0.5;
            (vdl_val, vdh_val)
        };

        utils::hermite_cubic_interpolate(t, vl, vdl, vh, vdh)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_x_derivative_interior() {
        let log_xs = vec![0.0, 1.0, 2.0, 3.0];
        let values = vec![0.0, 1.0, 4.0, 9.0];
        // Interior: average of (1-0)/1 and (4-1)/1 = (1+3)/2 = 2
        let d = compute_x_derivative(&log_xs, &values, 1);
        assert!((d - 2.0).abs() < 1e-12);
    }

    #[test]
    fn test_compute_x_derivative_boundary() {
        let log_xs = vec![0.0, 1.0, 2.0];
        let values = vec![0.0, 2.0, 6.0];
        // Left boundary: (2-0)/1 = 2
        assert!((compute_x_derivative(&log_xs, &values, 0) - 2.0).abs() < 1e-12);
        // Right boundary: (6-2)/1 = 4
        assert!((compute_x_derivative(&log_xs, &values, 2) - 4.0).abs() < 1e-12);
    }

    #[test]
    fn test_2d_matches_knot_values() {
        // Build a simple 2x2 grid and verify knot values are reproduced exactly
        let log_xs = vec![0.0, 1.0];
        let log_q2s = vec![0.0, 1.0];
        let vals = [[1.0, 2.0], [3.0, 4.0]]; // vals[x][q2]

        let ih = InterleavedHermite::build(log_xs, vec![log_q2s], 1, |_flav, x_idx, extra| {
            vals[x_idx][extra[0]]
        });

        // Evaluate at exact lower-left knot: x=exp(0)=1.0, q2=exp(0)=1.0
        let v00 = ih.eval_single(0, &[1.0, 1.0]);
        assert!((v00 - 1.0).abs() < 1e-12, "Got {v00} expected 1.0");

        // Evaluate at exact upper-right knot: x=exp(1)=e, q2=exp(1)=e
        let v11 = ih.eval_single(0, &[1.0_f64.exp(), 1.0_f64.exp()]);
        assert!((v11 - 4.0).abs() < 1e-12, "Got {v11} expected 4.0");
    }
}
