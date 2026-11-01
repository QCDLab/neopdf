//! The main collinear `PDF` path in a caller-chosen arithmetic.
//!
//! [`crate::pdf::PDF::with_arithmetic`] (or [`GridPDF::with_arithmetic`]) prepares a
//! [`PrecisionPDF`]: the same subgrid selection, flavour lookup, positivity policy and log-Hermite
//! interpolation as [`crate::pdf::PDF::xfxq2`] and [`crate::pdf::PDF::alphas_q2`], evaluated with
//! the scalar-generic kernels of [`crate::precision`]. With [`Binary64`](crate::precision::Binary64)
//! it reproduces the binary64 path; with a wider arithmetic it evaluates the same fixed binary64
//! dataset more accurately.
//!
//! Supported class: sets with the `LogBicubic` interpolator whose subgrids are all `(x, Q^2)` grids
//! (one nucleon number, one αs value, collinear), and tabulated αs. Anything else is reported as
//! [`PrecisionError::Unsupported`] when the view is prepared, or for αs when it is evaluated.
//!
//! Differences from the binary64 path, all explicit:
//! - No extrapolation: a point outside every subgrid, or αs outside its table, is
//!   [`InterpolationError::OutOfRange`], where the binary64 path clamps or extrapolates.
//! - Thresholds: where the set was read from LHAPDF `.dat` files, the original Q knots are squared
//!   in the selected arithmetic, for the subgrids and for αs alike, so a threshold given by one Q
//!   stays one point (see [`GridArray::with_q_knots`](crate::gridpdf::GridArray::with_q_knots)).
//!   Otherwise the stored binary64 Q^2 knots are used.
//!
//! As in the binary64 path, a flavour that the set does not contain evaluates to zero.

use std::cmp::Ordering;

use thiserror::Error;

use crate::gridpdf::{ForcePositive, GridPDF};
use crate::interpolator::InterpolationConfig;
use crate::metadata::InterpolatorType;
use crate::precision::{Arithmetic, InterpolationError, LogBicubicGrid, LogCubicTable};

/// Errors of the precision-generic path; it never substitutes a value for a failure.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum PrecisionError {
    /// The set lies outside the supported class (see the module documentation).
    #[error("not supported by the precision-generic path: {0}")]
    Unsupported(String),
    /// Interpolation failed (outside the grid, non-finite input, insufficient precision, ...).
    #[error(transparent)]
    Interpolation(#[from] InterpolationError),
}

/// One subgrid, prepared in the selected arithmetic, with its bounds in that arithmetic.
struct PreparedSubgrid<A: Arithmetic> {
    grid: LogBicubicGrid<A>,
    x: (A::Scalar, A::Scalar),
    q2: (A::Scalar, A::Scalar),
}

/// The collinear `PDF` path in the arithmetic `A`; see the module documentation.
pub struct PrecisionPDF<'a, A: Arithmetic> {
    arithmetic: A,
    pdf: &'a GridPDF,
    subgrids: Vec<PreparedSubgrid<A>>,
    /// `None` for sets with analytic αs.
    alphas: Option<LogCubicTable<A>>,
}

impl<'a, A: Arithmetic + Clone> PrecisionPDF<'a, A> {
    /// Prepares the view: every subgrid and the αs table are rebuilt from the fixed binary64 data
    /// in `arithmetic`.
    pub(crate) fn new(pdf: &'a GridPDF, arithmetic: A) -> Result<Self, PrecisionError> {
        let info = pdf.metadata();
        if info.interpolator_type != InterpolatorType::LogBicubic {
            return Err(PrecisionError::Unsupported(format!(
                "interpolator {:?} (only LogBicubic)",
                info.interpolator_type
            )));
        }
        let knots = &pdf.knot_array;
        let flavors = knots.pids.len();
        let subgrids = knots
            .subgrids
            .iter()
            .enumerate()
            .map(|(index, subgrid)| {
                if !matches!(subgrid.interpolation_config(), InterpolationConfig::TwoD) {
                    return Err(PrecisionError::Unsupported(
                        "subgrids beyond (x, Q^2): nucleon, αs, kT, ξ or Δ dimensions".to_string(),
                    ));
                }
                let xs = subgrid.xs.as_slice().expect("contiguous x knots");
                // The (x, Q^2) plane of each flavour slot (the index into `knots.pids`), as
                // `SubGrid::grid_slice` lays it out for the binary64 path.
                let planes: Vec<_> = (0..flavors).map(|f| subgrid.grid_slice(f)).collect();
                let value_at = |flavor: usize, ix: usize, iq: usize| planes[flavor][[ix, iq]];
                let a = &arithmetic;
                let square = |q: f64| {
                    let q = a.lift(q);
                    a.mul(&q, &q)
                };
                // Original Q knots are squared in `arithmetic` (PDF and alpha_s thresholds given by
                // the same Q stay identical); otherwise the stored Q^2 knots are used.
                let (prepared, q2) = match knots.q_knots(index) {
                    Some(qs) => (
                        LogBicubicGrid::from_q_knots(
                            arithmetic.clone(),
                            xs,
                            qs,
                            flavors,
                            value_at,
                        )?,
                        (square(qs[0]), square(qs[qs.len() - 1])),
                    ),
                    None => {
                        let q2s = subgrid.q2s.as_slice().expect("contiguous Q^2 knots");
                        (
                            LogBicubicGrid::new(arithmetic.clone(), xs, q2s, flavors, value_at)?,
                            (a.lift(q2s[0]), a.lift(q2s[q2s.len() - 1])),
                        )
                    }
                };
                Ok(PreparedSubgrid {
                    grid: prepared,
                    x: (a.lift(xs[0]), a.lift(xs[xs.len() - 1])),
                    q2,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let alphas = if info.alphas_vals.is_empty() {
            None
        } else {
            // The same knots as the binary64 path (repeated threshold Q knots reduced to the first).
            let (qs, values) = crate::alphas::alphas_knots(info);
            Some(LogCubicTable::from_q_knots(
                arithmetic.clone(),
                &qs,
                &values,
            )?)
        };
        Ok(Self {
            arithmetic,
            pdf,
            subgrids,
            alphas,
        })
    }
}

impl<A: Arithmetic> PrecisionPDF<'_, A> {
    /// `x f(x, Q^2)` of flavour `pid` (PDG code, 0 for the gluon), as [`crate::pdf::PDF::xfxq2`]
    /// with the points `[x, Q^2]`.
    pub fn xfxq2(
        &self,
        pid: i32,
        x: &A::Scalar,
        q2: &A::Scalar,
    ) -> Result<A::Scalar, PrecisionError> {
        let a = &self.arithmetic;
        if !a.is_finite(x) || !a.is_finite(q2) {
            return Err(InterpolationError::NonFinite.into());
        }
        let Some(flavor) = self.pdf.knot_array.pid_index(pid) else {
            return Ok(a.lift(0.0));
        };
        let subgrid = self.locate(x, q2)?;
        let value = subgrid.grid.evaluate(flavor, x, q2)?;
        Ok(match self.pdf.force_positive {
            Some(ForcePositive::ClipNegative) => self.at_least(value, 0.0),
            Some(ForcePositive::ClipSmall) => self.at_least(value, 1e-10),
            Some(ForcePositive::NoClipping) | None => value,
        })
    }

    /// `αs(Q^2)` from the set's table, as [`crate::pdf::PDF::alphas_q2`].
    pub fn alphas_q2(&self, q2: &A::Scalar) -> Result<A::Scalar, PrecisionError> {
        let table = self.alphas.as_ref().ok_or_else(|| {
            PrecisionError::Unsupported("analytic αs (only tabulated αs)".to_string())
        })?;
        Ok(table.evaluate(q2)?)
    }

    /// The subgrid that evaluates `(x, Q^2)`: the first subgrid, in the set's order, whose x range and
    /// Q^2 range both contain the point, bounds included. This is the subgrid the binary64 path uses
    /// for the same point; a point outside every subgrid is an error (no extrapolation).
    fn locate(
        &self,
        x: &A::Scalar,
        q2: &A::Scalar,
    ) -> Result<&PreparedSubgrid<A>, InterpolationError> {
        let a = &self.arithmetic;
        let inside = |v: &A::Scalar, (lo, hi): &(A::Scalar, A::Scalar)| {
            a.compare(v, lo) != Some(Ordering::Less) && a.compare(v, hi) != Some(Ordering::Greater)
        };
        self.subgrids
            .iter()
            .find(|s| inside(x, &s.x) && inside(q2, &s.q2))
            .ok_or(InterpolationError::OutOfRange)
    }

    fn at_least(&self, value: A::Scalar, floor: f64) -> A::Scalar {
        let a = &self.arithmetic;
        let floor = a.lift(floor);
        if a.compare(&value, &floor) == Some(Ordering::Less) {
            floor
        } else {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gridpdf::GridArray;
    use crate::metadata::{MetaData, SetType};
    use crate::precision::Binary64;
    use crate::subgrid::SubGrid;

    const PIDS: [i32; 3] = [-1, 21, 2];

    /// Two subgrids sharing the threshold Q = 4.5 GeV, a tabulated αs with the threshold knot
    /// repeated, and smooth but nontrivial values.
    fn grid_pdf(interpolator: InterpolatorType, q_knots: bool) -> GridPDF {
        let xs: Vec<f64> = vec![1e-5, 1e-4, 1e-3, 0.01, 0.1, 0.3, 0.6, 1.0];
        let qs = [vec![1.65, 2.0, 3.0, 4.5], vec![4.5, 10.0, 100.0, 1e3, 1e4]];
        let subgrid = |qs: &Vec<f64>| {
            let q2s: Vec<f64> = qs.iter().map(|q| q * q).collect();
            let mut data = Vec::new();
            for x in &xs {
                for q in qs {
                    for f in 0..PIDS.len() {
                        let base = (f + 1) as f64 * (1.0 - *x).powi(3) * x.powf(-0.2);
                        data.push(base * (1.0 + 0.1 * q.ln()) - 0.05 * f as f64);
                    }
                }
            }
            SubGrid::new(
                vec![1.0],
                vec![0.118],
                vec![0.0],
                xs.clone(),
                q2s,
                PIDS.len(),
                data,
            )
        };
        let mut knots = GridArray::from_parts(
            ndarray::Array1::from_vec(PIDS.to_vec()),
            vec![subgrid(&qs[0]), subgrid(&qs[1])],
        );
        if q_knots {
            knots = knots.with_q_knots(qs.to_vec()).unwrap();
        }
        let info = MetaData {
            set_desc: "test".into(),
            set_index: 0,
            num_members: 1,
            x_min: 1e-5,
            x_max: 1.0,
            q_min: 1.65,
            q_max: 1e4,
            flavors: PIDS.to_vec(),
            format: "lhagrid1".into(),
            alphas_q_values: vec![1.65, 2.0, 3.0, 4.5, 4.5, 10.0, 100.0, 1e3, 1e4],
            alphas_vals: vec![0.33, 0.30, 0.25, 0.22, 0.2205, 0.18, 0.118, 0.095, 0.08],
            polarised: false,
            set_type: SetType::SpaceLike,
            interpolator_type: interpolator,
            error_type: "replicas".into(),
            hadron_pid: 2212,
            git_version: String::new(),
            code_version: String::new(),
            flavor_scheme: "variable".into(),
            order_qcd: 2,
            alphas_order_qcd: 2,
            m_w: 80.377,
            m_z: 91.1876,
            m_up: 0.0,
            m_down: 0.0,
            m_strange: 0.0,
            m_charm: 1.51,
            m_bottom: 4.5,
            m_top: 172.5,
            alphas_type: "ipol".into(),
            number_flavors: 5,
            xi_min: 0.0,
            xi_max: 0.0,
            delta_min: 0.0,
            delta_max: 0.0,
            error_conf_level: None,
        };
        GridPDF::new(info, knots)
    }

    #[test]
    fn binary64_view_reproduces_the_main_path() {
        for q_knots in [false, true] {
            let pdf = grid_pdf(InterpolatorType::LogBicubic, q_knots);
            let view = pdf.with_arithmetic(Binary64).unwrap();
            for &pid in &[-1, 0, 21, 2] {
                for x in [1e-5, 3e-5, 1e-3, 0.02, 0.3, 0.77, 1.0] {
                    for q in [1.65, 1.9, 4.5, 4.6, 33.0, 1e4] {
                        let q2 = q * q;
                        let expected = pdf.xfxq2(pid, &[x, q2]).unwrap();
                        let got = view.xfxq2(pid, &x, &q2).unwrap();
                        assert!(
                            (got - expected).abs() <= 1e-13 * expected.abs().max(1.0),
                            "{pid} {x} {q}: {got} vs {expected}"
                        );
                    }
                }
            }
            for q in [1.65, 2.5, 4.5, 7.0, 1e4] {
                let (expected, got) = (pdf.alphas_q2(q * q), view.alphas_q2(&(q * q)).unwrap());
                assert!(
                    (got - expected).abs() < 1e-15,
                    "αs({q}): {got} vs {expected}"
                );
            }
        }
    }

    #[test]
    fn missing_flavours_positivity_and_failures_are_explicit() {
        let mut pdf = grid_pdf(InterpolatorType::LogBicubic, true);
        let view = pdf.with_arithmetic(Binary64).unwrap();
        // A flavour absent from the set is zero, as in the binary64 path.
        assert_eq!(view.xfxq2(5, &0.1, &100.0), Ok(0.0));
        // No implicit extrapolation.
        let out = Err(PrecisionError::Interpolation(
            InterpolationError::OutOfRange,
        ));
        assert_eq!(view.xfxq2(21, &1e-6, &100.0), out);
        assert_eq!(view.xfxq2(21, &0.1, &(2e4 * 2e4)), out);
        assert_eq!(view.alphas_q2(&1.0), out);
        assert_eq!(
            view.xfxq2(21, &f64::NAN, &100.0),
            Err(PrecisionError::Interpolation(InterpolationError::NonFinite))
        );
        // Positivity clipping follows the PDF's policy (the gluon slot is negative near x = 1).
        let (x, q2) = (0.99, 100.0);
        assert!(view.xfxq2(21, &x, &q2).unwrap() < 0.0);
        drop(view);
        pdf.set_force_positive(ForcePositive::ClipNegative);
        assert_eq!(
            pdf.with_arithmetic(Binary64).unwrap().xfxq2(21, &x, &q2),
            Ok(0.0)
        );
        pdf.set_force_positive(ForcePositive::ClipSmall);
        assert_eq!(
            pdf.with_arithmetic(Binary64).unwrap().xfxq2(21, &x, &q2),
            Ok(1e-10)
        );
        // Other interpolators are reported, not approximated.
        assert!(matches!(
            grid_pdf(InterpolatorType::LogBilinear, true).with_arithmetic(Binary64),
            Err(PrecisionError::Unsupported(_))
        ));
    }

    #[test]
    fn inconsistent_q_knots_are_rejected() {
        let pdf = grid_pdf(InterpolatorType::LogBicubic, false);
        let knots =
            GridArray::from_parts(pdf.knot_array.pids.clone(), pdf.knot_array.subgrids.clone());
        assert!(knots
            .with_q_knots(vec![
                vec![1.65, 2.0, 3.0, 4.5],
                vec![4.5, 10.0, 100.0, 1e3, 1.1e4]
            ])
            .is_err());
    }
}
