//! Per-output-element evaluation over a flattened [`DeviceGridLayout`].
//!
//! Everything here is plain, allocation-free Rust over borrowed slices, so the same
//! code is compiled into the device kernels (see [`crate::device`]) and runs on the
//! host for validation against `neopdf`'s CPU path. Each function is a direct port of
//! its CPU counterpart, keeping the same operation order:
//!
//! - [`Hermite`]: `InterleavedHermite::eval_allpids` (2-D branch).
//! - [`Chebyshev`]: `ChebyshevAllPids::{locate, eval_allpids}` (2-D branch).
//! - `GridView::alphas_q2`: `AlphaSCubicInterpolation::interpolate`.
//!
//! The batch output is `[n_events, n_members, n_flavors + 1]` in C order, the last
//! column holding `alpha_s(Q²)`. [`GridView::eval`] computes one entry of it, which
//! makes "one thread per output element" the natural device mapping.
//!
//! [`DeviceGridLayout`]: crate::grid_prep::DeviceGridLayout

// The arithmetic deliberately mirrors the CPU's own expressions (no fused
// multiply-adds, no `midpoint`), which is what makes the host port bit-exact.
#![allow(clippy::suboptimal_flops, clippy::manual_midpoint)]

/// Borrowed, flattened grid data; see [`DeviceGridLayout`] for the layout of each field.
///
/// [`DeviceGridLayout`]: crate::grid_prep::DeviceGridLayout
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub struct GridView<'a> {
    pub n_members: usize,
    pub n_flavors: usize,
    pub clip_floor: f64,
    pub log_xs: &'a [f64],
    pub x_nodes: &'a [f64],
    pub log_q2s: &'a [f64],
    pub q2_nodes: &'a [f64],
    pub q2_offsets: &'a [usize],
    pub values: &'a [f64],
    pub value_offsets: &'a [usize],
    pub alphas_log_q2s: &'a [f64],
    pub alphas: &'a [f64],
    pub alphas_offsets: &'a [usize],
}

/// One Q²-contiguous subgrid of one member, as located by [`GridView::subgrid`].
struct Subgrid<'a> {
    log_q2s: &'a [f64],
    q2_nodes: &'a [f64],
    /// This member's own value table for this subgrid.
    values: &'a [f64],
}

/// A collinear fast path a kernel can be specialized on.
pub trait Interpolator {
    /// Interpolated, unclipped `xf(x, Q²)` of `flavor` for `member`, at `ln(x)`/`ln(Q²)`,
    /// or `None` outside the region this fast path evaluates.
    fn xfxq2(grid: &GridView, member: usize, flavor: usize, lx: f64, lq2: f64) -> Option<f64>;
}

/// Interleaved cubic Hermite (`LogBicubic`): zero outside the grid.
pub enum Hermite {}

/// Barycentric Chebyshev (`LogChebyshev`): clamped to the grid.
pub enum Chebyshev {}

impl GridView<'_> {
    /// Number of output columns per `(event, member)`: every flavor, then `alpha_s`.
    #[must_use]
    pub const fn n_columns(&self) -> usize {
        self.n_flavors + 1
    }

    /// Number of output entries for `n_events` events.
    #[must_use]
    pub const fn output_len(&self, n_events: usize) -> usize {
        n_events * self.n_members * self.n_columns()
    }

    /// Entry `index` of the `[n_events, n_members, n_flavors + 1]` batch output.
    #[must_use]
    pub fn eval<I: Interpolator>(&self, index: usize, xs: &[f64], q2s: &[f64]) -> f64 {
        let column = index % self.n_columns();
        let member = (index / self.n_columns()) % self.n_members;
        let event = index / (self.n_columns() * self.n_members);
        let q2 = q2s[event];

        if column == self.n_flavors {
            self.alphas_q2(member, q2)
        } else {
            I::xfxq2(self, member, column, xs[event].ln(), q2.ln())
                .map_or(0.0, |value| value.max(self.clip_floor))
        }
    }

    /// Fills `out` with the whole batch output, on the host.
    ///
    /// # Panics
    ///
    /// Panics if `xs` and `q2s` differ in length or `out` is not
    /// `self.output_len(xs.len())` long.
    pub fn eval_batch<I: Interpolator>(&self, xs: &[f64], q2s: &[f64], out: &mut [f64]) {
        assert_eq!(xs.len(), q2s.len());
        assert_eq!(out.len(), self.output_len(xs.len()));
        for (index, o) in out.iter_mut().enumerate() {
            *o = self.eval::<I>(index, xs, q2s);
        }
    }

    /// Index of the first subgrid whose Q² range contains `lq2`, if any.
    fn subgrid_containing(&self, lq2: f64) -> Option<usize> {
        (0..self.q2_offsets.len() - 1).find(|&s| {
            let (lo, hi) = (self.q2_offsets[s], self.q2_offsets[s + 1] - 1);
            self.log_q2s[lo] <= lq2 && lq2 <= self.log_q2s[hi]
        })
    }

    /// Member `member`'s data for subgrid `s`.
    fn subgrid(&self, s: usize, member: usize) -> Subgrid<'_> {
        let q2 = self.q2_offsets[s]..self.q2_offsets[s + 1];
        let stride = (self.value_offsets[s + 1] - self.value_offsets[s]) / self.n_members;
        let start = self.value_offsets[s] + member * stride;
        Subgrid {
            log_q2s: &self.log_q2s[q2.clone()],
            q2_nodes: self.q2_nodes.get(q2).unwrap_or_default(),
            values: &self.values[start..start + stride],
        }
    }

    /// `alpha_s(Q²)` for `member`, as `AlphaSInterpol::alphas_q2` computes it.
    #[must_use]
    pub fn alphas_q2(&self, member: usize, q2: f64) -> f64 {
        let range = self.alphas_offsets[member]..self.alphas_offsets[member + 1];
        let (knots, alphas) = (&self.alphas_log_q2s[range.clone()], &self.alphas[range]);
        if alphas.len() == 1 {
            return alphas[0];
        }

        let lq2 = q2.ln();
        let last = knots.len() - 1;
        if !(knots[0] <= lq2 && lq2 <= knots[last]) {
            return 0.0;
        }

        // `ilogq2below`: snap onto a knot lying within 1e-9 above `lq2`.
        let i = match knots.partition_point(|&k| k < lq2) {
            above if above < last && (knots[above] - lq2).abs() < 1e-9 => above,
            above => above.min(last) - 1,
        };

        let slope = |j: usize| (alphas[j + 1] - alphas[j]) / (knots[j + 1] - knots[j]);
        let central = |j: usize| 0.5 * (slope(j) + slope(j - 1));
        let (di, di1) = if i == 0 {
            (slope(0), central(1))
        } else if i == last - 1 {
            (central(i), slope(i))
        } else {
            (central(i), central(i + 1))
        };

        let dlq2 = knots[i + 1] - knots[i];
        hermite(
            (lq2 - knots[i]) / dlq2,
            alphas[i],
            di * dlq2,
            alphas[i + 1],
            di1 * dlq2,
        )
    }
}

impl Interpolator for Hermite {
    fn xfxq2(grid: &GridView, member: usize, flavor: usize, lx: f64, lq2: f64) -> Option<f64> {
        let ix = find_interval(grid.log_xs, lx)?;
        let sg = grid.subgrid(grid.subgrid_containing(lq2)?, member);
        let iq2 = find_interval(sg.log_q2s, lq2)?;

        let knots = sg.log_q2s;
        let nq2 = knots.len();
        let u = (lx - grid.log_xs[ix]) / (grid.log_xs[ix + 1] - grid.log_xs[ix]);
        let v = (lq2 - knots[iq2]) / (knots[iq2 + 1] - knots[iq2]);
        // Cubic in x at Q² knot `iq2 + offset`, from the precomputed coefficients.
        let at = |offset: isize| {
            let cell = ix * nq2 + iq2.wrapping_add_signed(offset);
            let c = &sg.values[(cell * grid.n_flavors + flavor) * 4..][..4];
            let u2 = u * u;
            c[0] * (u2 * u) + c[1] * u2 + c[2] * u + c[3]
        };

        let dq_1 = knots[iq2 + 1] - knots[iq2];
        let (vl, vh) = (at(0), at(1));
        let below = || (vl - at(-1)) * dq_1 * (1.0 / (knots[iq2] - knots[iq2 - 1]));
        let above = || (at(2) - vh) * dq_1 * (1.0 / (knots[iq2 + 2] - knots[iq2 + 1]));
        let (vdl, vdh) = if nq2 == 2 {
            (vh - vl, vh - vl)
        } else if iq2 == 0 {
            (vh - vl, (vh - vl + above()) * 0.5)
        } else if iq2 == nq2 - 2 {
            ((vh - vl + below()) * 0.5, vh - vl)
        } else {
            ((vh - vl + below()) * 0.5, (vh - vl + above()) * 0.5)
        };

        Some(hermite(v, vl, vdl, vh, vdh))
    }
}

impl Interpolator for Chebyshev {
    fn xfxq2(grid: &GridView, member: usize, flavor: usize, lx: f64, lq2: f64) -> Option<f64> {
        // Out of every subgrid's range: the nearest one, as `GridArray::find_subgrid`.
        let s = grid.subgrid_containing(lq2).unwrap_or_else(|| {
            if lq2 < grid.log_q2s[0] {
                0
            } else {
                grid.q2_offsets.len() - 2
            }
        });
        let sg = grid.subgrid(s, member);
        let bx = Barycentric::new(lx, grid.log_xs, grid.x_nodes);
        let bq2 = Barycentric::new(lq2, sg.log_q2s, sg.q2_nodes);

        let (nx, nq2) = (grid.log_xs.len(), sg.log_q2s.len());
        let table = &sg.values[flavor * nx * nq2..][..nx * nq2];
        let mut result = 0.0;
        for i in 0..nx {
            let row = &table[i * nq2..][..nq2];
            let mut acc = 0.0;
            for (j, &value) in row.iter().enumerate() {
                acc += bq2.coeff(j) * value;
            }
            result += bx.coeff(i) * acc;
        }
        Some(result)
    }
}

/// Normalized barycentric coefficients at one point, computed on demand so the
/// kernel needs no per-thread scratch buffer.
struct Barycentric<'a> {
    t: f64,
    nodes: &'a [f64],
    /// The node `t` coincides with, where the coefficients collapse to a delta.
    exact: Option<usize>,
    inv_sum: f64,
}

impl<'a> Barycentric<'a> {
    /// Locates `point` (clamped to `knots`) among the Chebyshev `nodes` in `[-1, 1]`.
    fn new(point: f64, knots: &[f64], nodes: &'a [f64]) -> Self {
        let (lo, hi) = (knots[0], knots[knots.len() - 1]);
        let t = 2.0 * (point.clamp(lo, hi) - lo) / (hi - lo) - 1.0;
        let exact = nodes.iter().position(|&node| (t - node).abs() < 1e-15);
        let sum: f64 = (0..nodes.len()).map(|j| Self::term(t, nodes, j)).sum();
        Self {
            t,
            nodes,
            exact,
            inv_sum: sum.recip(),
        }
    }

    /// Unnormalized term `w_j / (t - t_j)`, with the Chebyshev barycentric weights
    /// `w_j = (-1)^j`, halved at both ends.
    fn term(t: f64, nodes: &[f64], j: usize) -> f64 {
        let sign = if j % 2 == 1 { -1.0 } else { 1.0 };
        let weight = if j == 0 || j == nodes.len() - 1 {
            sign * 0.5
        } else {
            sign
        };
        weight / (t - nodes[j])
    }

    fn coeff(&self, j: usize) -> f64 {
        self.exact.map_or_else(
            || Self::term(self.t, self.nodes, j) * self.inv_sum,
            |k| f64::from(u8::from(j == k)),
        )
    }
}

/// Index `i` with `knots[i] <= value <= knots[i + 1]`, the last interval for the
/// last knot, or `None` outside the grid (`utils::find_interval_index`).
fn find_interval(knots: &[f64], value: f64) -> Option<usize> {
    let last = knots.len() - 1;
    if !(knots[0] <= value && value <= knots[last]) {
        return None;
    }
    Some(knots.partition_point(|&k| k <= value).clamp(1, last) - 1)
}

/// Cubic Hermite interpolation on `[0, 1]` (`precision::hermite`).
fn hermite(t: f64, vl: f64, vdl: f64, vh: f64, vdh: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    let two_t3 = 2.0 * t3;
    let three_t2 = 3.0 * t2;
    let p0 = (two_t3 - three_t2 + 1.0) * vl;
    let m0 = (t3 - 2.0 * t2 + t) * vdl;
    let p1 = (0.0 - two_t3 + three_t2) * vh;
    let m1 = (t3 - t2) * vdh;
    p0 + m0 + p1 + m1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_interval_matches_cpu_conventions() {
        let knots = [0.0, 1.0, 2.0, 3.0];
        assert_eq!(find_interval(&knots, -0.1), None);
        assert_eq!(find_interval(&knots, 0.0), Some(0));
        assert_eq!(find_interval(&knots, 1.0), Some(1));
        assert_eq!(find_interval(&knots, 2.5), Some(2));
        assert_eq!(find_interval(&knots, 3.0), Some(2));
        assert_eq!(find_interval(&knots, 3.1), None);
    }
}
