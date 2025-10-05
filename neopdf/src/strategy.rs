//! This module defines various interpolation strategies used within the `neopdf` library.
//!
//! It provides implementations for 1D, 2D, and 3D interpolation, including:
//! - `BilinearInterpolation`: Standard bilinear interpolation for 2D data.
//! - `LogBilinearInterpolation`: Bilinear interpolation performed in logarithmic space for both
//!   coordinates, suitable for data that exhibits linear behavior in log-log plots.
//! - `LogBicubicInterpolation`: Bicubic interpolation with logarithmic coordinate scaling,
//!   providing C1 continuity and higher accuracy for 2D data.
//! - `LogTricubicInterpolation`: Tricubic interpolation with logarithmic coordinate scaling,
//!   extending bicubic interpolation to 3D data with C1 continuity.
//! - `AlphaSCubicInterpolation`: A specialized 1D cubic interpolation strategy for alpha_s values,
//!   incorporating specific extrapolation rules as defined in LHAPDF.
//!
//! All interpolation strategies are designed to work with `ninterp`'s data structures and traits,
//! ensuring compatibility and extensibility.

use ndarray::{Array2, Axis, Data, RawDataClone};
use ninterp::data::{InterpData1D, InterpData2D, InterpData3D};
use ninterp::error::{InterpolateError, ValidateError};
use ninterp::strategy::traits::{Strategy1D, Strategy2D, Strategy3D};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

use super::utils;

/// Implements bilinear interpolation for 2D data.
///
/// This strategy performs linear interpolation sequentially along two dimensions.
/// It is suitable for smooth, continuous 2D datasets where a simple linear
/// approximation between grid points is sufficient.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BilinearInterpolation;

impl BilinearInterpolation {
    /// Performs linear interpolation between two points.
    ///
    /// Given two points `(x1, y1)` and `(x2, y2)`, this function calculates the
    /// y-value corresponding to a given `x` using linear interpolation.
    ///
    /// # Arguments
    ///
    /// * `x1` - The x-coordinate of the first point.
    /// * `x2` - The x-coordinate of the second point.
    /// * `y1` - The y-coordinate of the first point.
    /// * `y2` - The y-coordinate of the second point.
    /// * `x` - The x-coordinate at which to interpolate.
    ///
    /// # Returns
    ///
    /// The interpolated y-value.
    fn linear_interpolate(x1: f64, x2: f64, y1: f64, y2: f64, x: f64) -> f64 {
        if x1 == x2 {
            return y1;
        }
        y1 + (y2 - y1) * (x - x1) / (x2 - x1)
    }
}

impl<D> Strategy2D<D> for BilinearInterpolation
where
    D: Data<Elem = f64> + RawDataClone + Clone,
{
    /// Performs bilinear interpolation at a given point.
    ///
    /// # Arguments
    ///
    /// * `data` - The interpolation data containing grid coordinates and values.
    /// * `point` - A 2-element array `[x, y]` representing the coordinates to interpolate at.
    ///
    /// # Returns
    ///
    /// The interpolated value as a `Result`.
    fn interpolate(
        &self,
        data: &InterpData2D<D>,
        point: &[f64; 2],
    ) -> Result<f64, InterpolateError> {
        let [x, y] = *point;

        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();
        let values = &data.values;

        let x_idx = utils::find_interval_index(x_coords, x)?;
        let y_idx = utils::find_interval_index(y_coords, y)?;

        let x1 = x_coords[x_idx];
        let x2 = x_coords[x_idx + 1];
        let y1 = y_coords[y_idx];
        let y2 = y_coords[y_idx + 1];

        let q11 = values[[x_idx, y_idx]]; // f(x1, y1)
        let q12 = values[[x_idx, y_idx + 1]]; // f(x1, y2)
        let q21 = values[[x_idx + 1, y_idx]]; // f(x2, y1)
        let q22 = values[[x_idx + 1, y_idx + 1]]; // f(x2, y2)

        let r1 = Self::linear_interpolate(x1, x2, q11, q21, x);
        let r2 = Self::linear_interpolate(x1, x2, q12, q22, x);

        let result = Self::linear_interpolate(y1, y2, r1, r2, y);

        Ok(result)
    }

    /// Indicates that this strategy does not allow extrapolation.
    fn allow_extrapolate(&self) -> bool {
        true
    }
}

/// Performs bilinear interpolation in log space.
///
/// This strategy transforms the input coordinates to their natural logarithms
/// before performing bilinear interpolation, which is suitable for data
/// that is linear in log-log space. It is particularly useful for physical
/// quantities that span several orders of magnitude, such as momentum transfer
/// squared (Q²) or Bjorken x.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogBilinearInterpolation;

impl<D> Strategy2D<D> for LogBilinearInterpolation
where
    D: Data<Elem = f64> + RawDataClone + Clone,
{
    /// Initializes the strategy, performing validation checks.
    ///
    /// # Arguments
    ///
    /// * `data` - The interpolation data to validate.
    fn init(&mut self, _data: &InterpData2D<D>) -> Result<(), ValidateError> {
        Ok(())
    }

    /// Performs log-bilinear interpolation at a given point.
    ///
    /// The input `point` coordinates are first transformed to log space,
    /// then bilinear interpolation is applied.
    ///
    /// # Arguments
    ///
    /// * `data` - The interpolation data containing grid coordinates and values.
    /// * `point` - A 2-element array `[x, y]` representing the coordinates to interpolate at.
    ///
    /// # Returns
    ///
    /// The interpolated value as a `Result`.
    fn interpolate(
        &self,
        data: &InterpData2D<D>,
        point: &[f64; 2],
    ) -> Result<f64, InterpolateError> {
        let [x, y] = *point;

        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();
        let values = &data.values;

        let x_idx = utils::find_interval_index(x_coords, x)?;
        let y_idx = utils::find_interval_index(y_coords, y)?;

        let x1 = x_coords[x_idx];
        let x2 = x_coords[x_idx + 1];
        let y1 = y_coords[y_idx];
        let y2 = y_coords[y_idx + 1];

        let q11 = values[[x_idx, y_idx]]; // f(x1, y1)
        let q12 = values[[x_idx, y_idx + 1]]; // f(x1, y2)
        let q21 = values[[x_idx + 1, y_idx]]; // f(x2, y1)
        let q22 = values[[x_idx + 1, y_idx + 1]]; // f(x2, y2)

        let r1 = BilinearInterpolation::linear_interpolate(x1, x2, q11, q21, x);
        let r2 = BilinearInterpolation::linear_interpolate(x1, x2, q12, q22, x);

        let result = BilinearInterpolation::linear_interpolate(y1, y2, r1, r2, y);

        Ok(result)
    }

    /// Indicates that this strategy does not allow extrapolation.
    fn allow_extrapolate(&self) -> bool {
        true
    }
}

/// LogBicubic interpolation strategy for PDF-like data.
///
/// This strategy implements bicubic interpolation with logarithmic coordinate scaling.
/// It is designed for interpolating Parton Distribution Functions (PDFs) where:
/// - x-coordinates (e.g., Bjorken x) are logarithmically spaced.
/// - y-coordinates (e.g., Q² values) are logarithmically spaced.
/// - z-values (PDF values) are interpolated using bicubic splines.
///
/// Bicubic interpolation uses a 4x4 grid of points around the interpolation point
/// and provides C1 continuity (continuous first derivatives), resulting in a
/// smoother and more accurate interpolation compared to bilinear methods.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogBicubicInterpolation {
    coeffs: Vec<f64>,
}

impl LogBicubicInterpolation {
    /// Find the interval for bicubic interpolation.
    ///
    /// This function determines the appropriate interval index `i` within a set of
    /// coordinates `coords` such that `coords[i] <= x < coords[i+1]`. For bicubic
    /// interpolation, this index `i` is used to select the 4x4 grid of points
    /// `[i-1, i, i+1, i+2]` that are relevant for the interpolation.
    ///
    /// # Arguments
    ///
    /// * `coords` - A slice of `f64` representing the sorted coordinate values.
    /// * `x` - The `f64` value for which to find the interval.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `usize` index of the lower bound of the interval
    /// if successful, or an `InterpolateError` if `x` is out of bounds.
    fn find_bicubic_interval(coords: &[f64], x: f64) -> Result<usize, InterpolateError> {
        // Find the interval [i, i+1] such that coords[i] <= x < coords[i+1]
        let i = utils::find_interval_index(coords, x)?;
        Ok(i)
    }

    /// Cubic interpolation using a passed array of coefficients (a*x^3 + b*x^2 + c*x + d)
    pub fn hermite_cubic_interpolate_from_coeffs(t: f64, coeffs: &[f64; 4]) -> f64 {
        let x = t;
        let x2 = x * x;
        let x3 = x2 * x;
        coeffs[0] * x3 + coeffs[1] * x2 + coeffs[2] * x + coeffs[3]
    }

    /// Calculates the derivative with respect to x at a given knot.
    /// This mirrors the _ddx function in LHAPDF's C++ implementation.
    pub fn calculate_ddx<D>(data: &InterpData2D<D>, ix: usize, iq2: usize) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let nxknots = data.grid[0].len();
        let x_coords = data.grid[0].as_slice().unwrap();
        let values = &data.values;

        let del1 = match ix {
            0 => 0.0,
            i => x_coords[i] - x_coords[i - 1],
        };

        let del2 = match x_coords.get(ix + 1) {
            Some(&next) => next - x_coords[ix],
            None => 0.0,
        };

        if ix != 0 && ix != nxknots - 1 {
            let lddx = (values[[ix, iq2]] - values[[ix - 1, iq2]]) / del1;
            let rddx = (values[[ix + 1, iq2]] - values[[ix, iq2]]) / del2;
            (lddx + rddx) / 2.0
        } else if ix == 0 {
            (values[[ix + 1, iq2]] - values[[ix, iq2]]) / del2
        } else if ix == nxknots - 1 {
            (values[[ix, iq2]] - values[[ix - 1, iq2]]) / del1
        } else {
            panic!("Should not reach here: Invalid index for derivative calculation.");
        }
    }

    /// Computes the polynomial coefficients for bicubic interpolation, mirroring LHAPDF's C++ implementation.
    fn compute_polynomial_coefficients<D>(data: &InterpData2D<D>) -> Vec<f64>
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let nxknots = data.grid[0].len();
        let nq2knots = data.grid[1].len();
        let values = &data.values;

        // The shape of the coefficients array: (nxknots-1) * nq2knots * 4 (for a,b,c,d)
        let mut coeffs: Vec<f64> = vec![0.0; (nxknots - 1) * nq2knots * 4];

        for ix in 0..nxknots - 1 {
            for iq2 in 0..nq2knots {
                let dx =
                    data.grid[0].as_slice().unwrap()[ix + 1] - data.grid[0].as_slice().unwrap()[ix];

                let vl = values[[ix, iq2]];
                let vh = values[[ix + 1, iq2]];
                let vdl = Self::calculate_ddx(data, ix, iq2) * dx;
                let vdh = Self::calculate_ddx(data, ix + 1, iq2) * dx;

                // polynomial coefficients
                let a = vdh + vdl - 2.0 * vh + 2.0 * vl;
                let b = 3.0 * vh - 3.0 * vl - 2.0 * vdl - vdh;
                let c = vdl;
                let d = vl;

                let base_idx = (ix * nq2knots + iq2) * 4;
                coeffs[base_idx] = a;
                coeffs[base_idx + 1] = b;
                coeffs[base_idx + 2] = c;
                coeffs[base_idx + 3] = d;
            }
        }
        coeffs
    }

    /// Performs bicubic interpolation using pre-computed coefficients.
    fn interpolate_with_coeffs<D>(
        &self,
        data: &InterpData2D<D>,
        ix: usize,
        iq2: usize,
        u: f64,
        v: f64,
    ) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let nq2knots = data.grid[1].len();

        let base_idx_vl = (ix * nq2knots + iq2) * 4;
        let coeffs_vl: [f64; 4] = self.coeffs[base_idx_vl..base_idx_vl + 4]
            .try_into()
            .unwrap();
        let vl = Self::hermite_cubic_interpolate_from_coeffs(u, &coeffs_vl);

        let base_idx_vh = (ix * nq2knots + iq2 + 1) * 4;
        let coeffs_vh: [f64; 4] = self.coeffs[base_idx_vh..base_idx_vh + 4]
            .try_into()
            .unwrap();
        let vh = Self::hermite_cubic_interpolate_from_coeffs(u, &coeffs_vh);

        let q2_grid: &[f64] = data.grid[1].as_slice().unwrap();

        let dq_1 = q2_grid[iq2 + 1] - q2_grid[iq2];

        let vdl: f64;
        let vdh: f64;

        if iq2 == 0 {
            vdl = vh - vl;
            let vhh_base_idx = (ix * nq2knots + iq2 + 2) * 4;
            let coeffs_vhh: [f64; 4] = self.coeffs[vhh_base_idx..vhh_base_idx + 4]
                .try_into()
                .unwrap();
            let vhh = Self::hermite_cubic_interpolate_from_coeffs(u, &coeffs_vhh);
            let dq_2 = 1.0 / (q2_grid[iq2 + 2] - q2_grid[iq2 + 1]);
            vdh = (vdl + (vhh - vh) * dq_1 * dq_2) * 0.5;
        } else if iq2 == nq2knots - 2 {
            vdh = vh - vl;
            let vll_base_idx = (ix * nq2knots + iq2 - 1) * 4;
            let coeffs_vll: [f64; 4] = self.coeffs[vll_base_idx..vll_base_idx + 4]
                .try_into()
                .unwrap();
            let vll = Self::hermite_cubic_interpolate_from_coeffs(u, &coeffs_vll);
            let dq_0 = 1.0 / (q2_grid[iq2] - q2_grid[iq2 - 1]);
            vdl = (vdh + (vl - vll) * dq_1 * dq_0) * 0.5;
        } else {
            let vll_base_idx = (ix * nq2knots + iq2 - 1) * 4;
            let coeffs_vll: [f64; 4] = self.coeffs[vll_base_idx..vll_base_idx + 4]
                .try_into()
                .unwrap();
            let vll = Self::hermite_cubic_interpolate_from_coeffs(u, &coeffs_vll);
            let dq_0 = 1.0 / (q2_grid[iq2] - q2_grid[iq2 - 1]);

            let vhh_base_idx = (ix * nq2knots + iq2 + 2) * 4;
            let coeffs_vhh: [f64; 4] = self.coeffs[vhh_base_idx..vhh_base_idx + 4]
                .try_into()
                .unwrap();
            let vhh = Self::hermite_cubic_interpolate_from_coeffs(u, &coeffs_vhh);
            let dq_2 = 1.0 / (q2_grid[iq2 + 2] - q2_grid[iq2 + 1]);

            vdl = ((vh - vl) + (vl - vll) * dq_1 * dq_0) * 0.5;
            vdh = ((vh - vl) + (vhh - vh) * dq_1 * dq_2) * 0.5;
        }

        utils::hermite_cubic_interpolate(v, vl, vdl, vh, vdh)
    }
}

impl<D> Strategy2D<D> for LogBicubicInterpolation
where
    D: Data<Elem = f64> + RawDataClone + Clone,
{
    fn init(&mut self, data: &InterpData2D<D>) -> Result<(), ValidateError> {
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();

        if x_coords.len() < 4 || y_coords.len() < 4 {
            return Err(ValidateError::Other(
                "Need at least 4x4 grid for bicubic interpolation".to_string(),
            ));
        }

        self.coeffs = Self::compute_polynomial_coefficients(data);
        Ok(())
    }

    fn interpolate(
        &self,
        data: &InterpData2D<D>,
        point: &[f64; 2],
    ) -> Result<f64, InterpolateError> {
        let [x, y] = *point;

        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();

        let i = Self::find_bicubic_interval(x_coords, x)?;
        let j = Self::find_bicubic_interval(y_coords, y)?;

        let dx = x_coords[i + 1] - x_coords[i];
        let dy = y_coords[j + 1] - y_coords[j];

        if dx == 0.0 || dy == 0.0 {
            return Err(InterpolateError::Other("Grid spacing is zero".to_string()));
        }

        let u = (x - x_coords[i]) / dx;
        let v = (y - y_coords[j]) / dy;

        let result = self.interpolate_with_coeffs(data, i, j, u, v);

        Ok(result)
    }

    fn allow_extrapolate(&self) -> bool {
        true
    }
}

/// LogTricubic interpolation strategy for PDF-like data
///
/// This strategy implements tricubic interpolation with logarithmic coordinate scaling:
/// - x-coordinates are logarithmically spaced (e.g., 1e-9 to 1)
/// - y-coordinates are logarithmically spaced (e.g., Q² values)
/// - z-coordinates are logarithmically spaced (e.g., Mass Atomic A, AlphaS)
/// - w-values (PDF values) are interpolated using tricubic splines
///
/// Tricubic interpolation uses a 4x4x4 grid of points around the interpolation point
/// and provides C1 continuity (continuous first derivatives).
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogTricubicInterpolation;

impl LogTricubicInterpolation {
    /// Returns the index i such that we can use points [i-1, i, i+1, i+2] for interpolation.
    fn find_tricubic_interval(coords: &[f64], x: f64) -> Result<usize, InterpolateError> {
        // Find the interval [i, i+1] such that coords[i] <= x < coords[i+1]
        let i = utils::find_interval_index(coords, x)?;
        Ok(i)
    }

    /// Cubic interpolation using a passed array of coefficients (a*x^3 + b*x^2 + c*x + d)
    pub fn hermite_cubic_interpolate_from_coeffs(t: f64, coeffs: &[f64; 4]) -> f64 {
        let x = t;
        let x2 = x * x;
        let x3 = x2 * x;
        coeffs[0] * x3 + coeffs[1] * x2 + coeffs[2] * x + coeffs[3]
    }

    /// Calculates the derivative with respect to x at a given knot.
    pub fn calculate_ddx<D>(data: &InterpData3D<D>, ix: usize, iq2: usize, iz: usize) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let nxknots = data.grid[0].len();
        let x_coords = data.grid[0].as_slice().unwrap();
        let values = &data.values;

        let del1 = match ix {
            0 => 0.0,
            i => x_coords[i] - x_coords[i - 1],
        };

        let del2 = match x_coords.get(ix + 1) {
            Some(&next) => next - x_coords[ix],
            None => 0.0,
        };

        if ix != 0 && ix != nxknots - 1 {
            let lddx = (values[[ix, iq2, iz]] - values[[ix - 1, iq2, iz]]) / del1;
            let rddx = (values[[ix + 1, iq2, iz]] - values[[ix, iq2, iz]]) / del2;
            (lddx + rddx) / 2.0
        } else if ix == 0 {
            (values[[ix + 1, iq2, iz]] - values[[ix, iq2, iz]]) / del2
        } else if ix == nxknots - 1 {
            (values[[ix, iq2, iz]] - values[[ix - 1, iq2, iz]]) / del1
        } else {
            panic!("Should not reach here: Invalid index for derivative calculation.");
        }
    }

    /// Calculates the derivative with respect to y at a given knot.
    pub fn calculate_ddy<D>(data: &InterpData3D<D>, ix: usize, iq2: usize, iz: usize) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let nq2knots = data.grid[1].len();
        let q2_coords = data.grid[1].as_slice().unwrap();
        let values = &data.values;

        let del1 = match iq2 {
            0 => 0.0,
            i => q2_coords[i] - q2_coords[i - 1],
        };

        let del2 = match q2_coords.get(iq2 + 1) {
            Some(&next) => next - q2_coords[iq2],
            None => 0.0,
        };

        if iq2 != 0 && iq2 != nq2knots - 1 {
            let lddq = (values[[ix, iq2, iz]] - values[[ix, iq2 - 1, iz]]) / del1;
            let rddq = (values[[ix, iq2 + 1, iz]] - values[[ix, iq2, iz]]) / del2;
            (lddq + rddq) / 2.0
        } else if iq2 == 0 {
            (values[[ix, iq2 + 1, iz]] - values[[ix, iq2, iz]]) / del2
        } else if iq2 == nq2knots - 1 {
            (values[[ix, iq2, iz]] - values[[ix, iq2 - 1, iz]]) / del1
        } else {
            panic!("Should not reach here: Invalid index for derivative calculation.");
        }
    }

    /// Calculates the derivative with respect to z at a given knot.
    pub fn calculate_ddz<D>(data: &InterpData3D<D>, ix: usize, iq2: usize, iz: usize) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let nmu2knots = data.grid[2].len();
        let mu2_coords = data.grid[2].as_slice().unwrap();
        let values = &data.values;

        let del1 = match iz {
            0 => 0.0,
            i => mu2_coords[i] - mu2_coords[i - 1],
        };

        let del2 = match mu2_coords.get(iz + 1) {
            Some(&next) => next - mu2_coords[iz],
            None => 0.0,
        };

        if iz != 0 && iz != nmu2knots - 1 {
            let lddmu = (values[[ix, iq2, iz]] - values[[ix, iq2, iz - 1]]) / del1;
            let rddmu = (values[[ix, iq2, iz + 1]] - values[[ix, iq2, iz]]) / del2;
            (lddmu + rddmu) / 2.0
        } else if iz == 0 {
            (values[[ix, iq2, iz + 1]] - values[[ix, iq2, iz]]) / del2
        } else if iz == nmu2knots - 1 {
            (values[[ix, iq2, iz]] - values[[ix, iq2, iz - 1]]) / del1
        } else {
            panic!("Should not reach here: Invalid index for derivative calculation.");
        }
    }

    fn hermite_tricubic_interpolate<D>(
        &self,
        data: &InterpData3D<D>,
        indices: (usize, usize, usize),
        coords: (f64, f64, f64),
        derivatives: (f64, f64, f64),
    ) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let (ix, iq2, iz) = indices;
        let (u, v, w) = coords;
        let (dx, dy, dz) = derivatives;

        let get = |dx, dy, dz| data.values[[ix + dx, iq2 + dy, iz + dz]];
        let ddx = |dx, dy, dz| Self::calculate_ddx(data, ix + dx, iq2 + dy, iz + dz);
        let ddy = |dx, dy, dz| Self::calculate_ddy(data, ix + dx, iq2 + dy, iz + dz);
        let ddz = |dx, dy, dz| Self::calculate_ddz(data, ix + dx, iq2 + dy, iz + dz);

        let interp_y: [[f64; 2]; 4] = [0, 1]
            .iter()
            .flat_map(|&y_offset| {
                [0, 1].iter().map(move |&z_offset| {
                    let (f0, f1) = (get(0, y_offset, z_offset), get(1, y_offset, z_offset));
                    let (d0, d1) = (
                        ddx(0, y_offset, z_offset) * dx,
                        ddx(1, y_offset, z_offset) * dx,
                    );
                    let interp_val = Self::cubic_interpolate(u, f0, d0, f1, d1);

                    let (df0, df1) = (
                        ddy(0, y_offset, z_offset) * dy,
                        ddy(1, y_offset, z_offset) * dy,
                    );
                    let interp_deriv = (1.0 - u) * df0 + u * df1;

                    [interp_val, interp_deriv]
                })
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let interp_z: [[f64; 2]; 2] = [0, 1]
            .iter()
            .enumerate()
            .map(|(iz_, &z_offset)| {
                let (f0, f1) = (interp_y[iz_][0], interp_y[2 + iz_][0]);
                let (d0, d1) = (interp_y[iz_][1], interp_y[2 + iz_][1]);
                let interp_val = Self::cubic_interpolate(v, f0, d0, f1, d1);

                let calc_z_deriv = |y_offset| {
                    let (df0, df1) = (
                        ddz(0, y_offset, z_offset) * dz,
                        ddz(1, y_offset, z_offset) * dz,
                    );
                    (1.0 - u) * df0 + u * df1
                };

                let interp_deriv = (1.0 - v) * calc_z_deriv(0) + v * calc_z_deriv(1);
                [interp_val, interp_deriv]
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let (f0, f1) = (interp_z[0][0], interp_z[1][0]);
        let (d0, d1) = (interp_z[0][1], interp_z[1][1]);
        Self::cubic_interpolate(w, f0, d0, f1, d1)
    }

    /// Hermite cubic interpolation with derivatives
    fn cubic_interpolate(t: f64, f0: f64, f0_prime: f64, f1: f64, f1_prime: f64) -> f64 {
        let t2 = t * t;
        let t3 = t2 * t;

        // Hermite basis functions
        let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
        let h10 = t3 - 2.0 * t2 + t;
        let h01 = -2.0 * t3 + 3.0 * t2;
        let h11 = t3 - t2;

        h00 * f0 + h10 * f0_prime + h01 * f1 + h11 * f1_prime
    }
}

impl<D> Strategy3D<D> for LogTricubicInterpolation
where
    D: Data<Elem = f64> + RawDataClone + Clone,
{
    fn init(&mut self, data: &InterpData3D<D>) -> Result<(), ValidateError> {
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();
        let z_coords = data.grid[2].as_slice().unwrap();

        if x_coords.len() < 4 || y_coords.len() < 4 || z_coords.len() < 4 {
            return Err(ValidateError::Other(
                "Need at least 4x4x4 grid for tricubic interpolation".to_string(),
            ));
        }

        // Uses the Hermite approach instead of coefficient precomputation.
        // This is more straightforward and avoids the complex 64x64 matrix.
        Ok(())
    }

    fn interpolate(
        &self,
        data: &InterpData3D<D>,
        point: &[f64; 3],
    ) -> Result<f64, InterpolateError> {
        let [x, y, z] = *point;

        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();
        let z_coords = data.grid[2].as_slice().unwrap();

        let i = Self::find_tricubic_interval(x_coords, x)?;
        let j = Self::find_tricubic_interval(y_coords, y)?;
        let k = Self::find_tricubic_interval(z_coords, z)?;

        let dx = x_coords[i + 1] - x_coords[i];
        let dy = y_coords[j + 1] - y_coords[j];
        let dz = z_coords[k + 1] - z_coords[k];

        if dx == 0.0 || dy == 0.0 || dz == 0.0 {
            return Err(InterpolateError::Other("Grid spacing is zero".to_string()));
        }

        let u = (x - x_coords[i]) / dx;
        let v = (y - y_coords[j]) / dy;
        let w = (z - z_coords[k]) / dz;

        let result = self.hermite_tricubic_interpolate(data, (i, j, k), (u, v, w), (dx, dy, dz));

        Ok(result)
    }

    fn allow_extrapolate(&self) -> bool {
        true
    }
}

/// Implements cubic interpolation for alpha_s values in log-Q2 space.
///
/// This strategy handles the specific extrapolation and interpolation rules
/// for alpha_s as defined in LHAPDF.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlphaSCubicInterpolation;

impl AlphaSCubicInterpolation {
    /// Get the index of the closest Q2 knot row <= q2
    ///
    /// If the value is >= q2_max, return (i_max-1).
    fn ilogq2below<D>(data: &InterpData1D<D>, logq2: f64) -> usize
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let logq2s = data.grid[0].as_slice().unwrap();
        if logq2 < *logq2s.first().unwrap() {
            panic!(
                "Q2 value {} is lower than lowest-Q2 grid point at {}",
                logq2.exp(),
                logq2s.first().unwrap().exp()
            );
        }
        if logq2 > *logq2s.last().unwrap() {
            panic!(
                "Q2 value {} is higher than highest-Q2 grid point at {}",
                logq2.exp(),
                logq2s.last().unwrap().exp()
            );
        }

        let idx = logq2s.partition_point(|&x| x < logq2);

        if idx == logq2s.len() {
            idx - 1
        } else if (logq2s[idx] - logq2).abs() < 1e-9 {
            if idx == logq2s.len() - 1 && logq2s.len() >= 2 {
                idx - 1
            } else {
                idx
            }
        } else {
            idx - 1
        }
    }

    /// Forward derivative w.r.t. logQ2
    fn ddlogq_forward<D>(data: &InterpData1D<D>, i: usize) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let logq2s = data.grid[0].as_slice().unwrap();
        let alphas = data.values.as_slice().unwrap();
        (alphas[i + 1] - alphas[i]) / (logq2s[i + 1] - logq2s[i])
    }

    /// Backward derivative w.r.t. logQ2
    fn ddlogq_backward<D>(data: &InterpData1D<D>, i: usize) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let logq2s = data.grid[0].as_slice().unwrap();
        let alphas = data.values.as_slice().unwrap();
        (alphas[i] - alphas[i - 1]) / (logq2s[i] - logq2s[i - 1])
    }

    /// Central (avg of forward and backward) derivative w.r.t. logQ2
    fn ddlogq_central<D>(data: &InterpData1D<D>, i: usize) -> f64
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        0.5 * (Self::ddlogq_forward(data, i) + Self::ddlogq_backward(data, i))
    }
}

impl<D> Strategy1D<D> for AlphaSCubicInterpolation
where
    D: Data<Elem = f64> + RawDataClone + Clone,
{
    fn interpolate(
        &self,
        data: &InterpData1D<D>,
        point: &[f64; 1],
    ) -> Result<f64, InterpolateError> {
        let logq2 = point[0];
        let logq2s = data.grid[0].as_slice().unwrap();
        let alphas = data.values.as_slice().unwrap();

        if logq2 < *logq2s.first().unwrap() {
            let mut next_point = 1;
            while logq2s[0] == logq2s[next_point] {
                next_point += 1;
            }
            let dlogq2 = logq2s[next_point] - logq2s[0];
            let dlogas = (alphas[next_point] / alphas[0]).ln();
            let loggrad = dlogas / dlogq2;
            return Ok(alphas[0] * (loggrad * (logq2 - logq2s[0])).exp());
        }

        if logq2 > *logq2s.last().unwrap() {
            return Ok(*alphas.last().unwrap());
        }

        let i = Self::ilogq2below(data, logq2);

        // Calculate derivatives
        let didlogq2: f64;
        let di1dlogq2: f64;
        if i == 0 {
            didlogq2 = Self::ddlogq_forward(data, i);
            di1dlogq2 = Self::ddlogq_central(data, i + 1);
        } else if i == logq2s.len() - 2 {
            didlogq2 = Self::ddlogq_central(data, i);
            di1dlogq2 = Self::ddlogq_backward(data, i + 1);
        } else {
            didlogq2 = Self::ddlogq_central(data, i);
            di1dlogq2 = Self::ddlogq_central(data, i + 1);
        }

        // Calculate alpha_s
        let dlogq2 = logq2s[i + 1] - logq2s[i];
        let tlogq2 = (logq2 - logq2s[i]) / dlogq2;
        Ok(utils::hermite_cubic_interpolate(
            tlogq2,
            alphas[i],
            didlogq2 * dlogq2,
            alphas[i + 1],
            di1dlogq2 * dlogq2,
        ))
    }

    fn allow_extrapolate(&self) -> bool {
        true
    }
}

/// Implements a global N-dimensional interpolation using Chebyshev polynomials with logarithmic
/// coordinate scaling.
///
/// This strategy, inspired by the method described in arXiv:2112.09703, first transforms the input
/// coordinates to their natural logarithms, and then fits a single, high-degree Chebyshev polynomial
/// to the entire dataset in the log-transformed space.
///
/// Key features:
/// - **Logarithmic Scaling**: Coordinates are transformed via `x -> ln(x)` before interpolation.
/// - **Global Nature**: The interpolation at any point depends on all data points in the grid.
/// - **High Degree**: The degree of the interpolating polynomial is `N-1`, where `N` is the
///   number of grid points in each dimension.
/// - **Grid Requirement**: For optimal stability and to avoid Runge's phenomenon, the grid
///   points should correspond to the roots or extrema of Chebyshev polynomials.
#[derive(Debug, Clone)]
pub struct LogChebyshevInterpolation<const DIM: usize> {
    // Pre-computed weights for the barycentric formula for each dimension.
    weights: [Vec<f64>; DIM],
    // Grid points in the t-domain [-1, 1] for each dimension.
    t_coords: [Vec<f64>; DIM],
}

impl<const DIM: usize> Default for LogChebyshevInterpolation<DIM> {
    fn default() -> Self {
        Self {
            weights: std::array::from_fn(|_| Vec::new()),
            t_coords: std::array::from_fn(|_| Vec::new()),
        }
    }
}

impl<const DIM: usize> Serialize for LogChebyshevInterpolation<DIM> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("LogChebyshevInterpolation", 2)?;
        state.serialize_field("weights", &self.weights.as_slice())?;
        state.serialize_field("t_coords", &self.t_coords.as_slice())?;
        state.end()
    }
}

impl<'de, const DIM: usize> Deserialize<'de> for LogChebyshevInterpolation<DIM> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            weights: Vec<Vec<f64>>,
            t_coords: Vec<Vec<f64>>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let weights = helper.weights.try_into().map_err(|v: Vec<Vec<f64>>| {
            serde::de::Error::invalid_length(v.len(), &"an array of the correct length")
        })?;
        let t_coords = helper.t_coords.try_into().map_err(|v: Vec<Vec<f64>>| {
            serde::de::Error::invalid_length(v.len(), &"an array of the correct length")
        })?;

        Ok(Self { weights, t_coords })
    }
}

impl<const DIM: usize> LogChebyshevInterpolation<DIM> {
    /// Computes the barycentric weights for a given set of Chebyshev points.
    /// The formula for the weights is `w_j = (-1)^j * delta_j`, where `delta_j`
    /// is 1/2 for the first and last points, and 1 otherwise.
    fn compute_barycentric_weights(n: usize) -> Vec<f64> {
        let mut weights = vec![1.0; n];
        (0..n).for_each(|j| {
            if j % 2 == 1 {
                weights[j] = -1.0;
            }
        });
        weights[0] *= 0.5;
        if n > 1 {
            weights[n - 1] *= 0.5;
        }
        weights
    }

    /// Computes normalized barycentric coefficients for interpolation
    /// Returns a vector of coefficients that sum to 1
    fn barycentric_coefficients(t: f64, t_coords: &[f64], weights: &[f64]) -> Vec<f64> {
        let mut coeffs = vec![0.0; t_coords.len()];

        for (j, &t_j) in t_coords.iter().enumerate() {
            if (t - t_j).abs() < 1e-15 {
                coeffs[j] = 1.0;
                return coeffs;
            }
        }

        let mut terms = Vec::with_capacity(t_coords.len());
        for (j, &t_j) in t_coords.iter().enumerate() {
            terms.push(weights[j] / (t - t_j));
        }

        let sum: f64 = terms.iter().sum();
        for (j, &term) in terms.iter().enumerate() {
            coeffs[j] = term / sum;
        }

        coeffs
    }

    /// Legacy barycentric interpolation method (kept for compatibility)
    fn barycentric_interpolate(t: f64, t_coords: &[f64], f_values: &[f64], weights: &[f64]) -> f64 {
        let mut numer = 0.0;
        let mut denom = 0.0;

        for (j, &t_j) in t_coords.iter().enumerate() {
            if (t - t_j).abs() < 1e-15 {
                return f_values[j];
            }

            let term = weights[j] / (t - t_j);
            numer += term * f_values[j];
            denom += term;
        }

        numer / denom
    }
}

impl<D> Strategy1D<D> for LogChebyshevInterpolation<1>
where
    D: Data<Elem = f64> + RawDataClone + Clone,
{
    fn init(&mut self, data: &InterpData1D<D>) -> Result<(), ValidateError> {
        let x_coords = data.grid[0].as_slice().unwrap();
        let n = x_coords.len();
        if n < 2 {
            return Err(ValidateError::Other(
                "LogChebyshevInterpolation requires at least 2 grid points.".to_string(),
            ));
        }

        self.t_coords[0] = (0..n)
            .map(|j| (PI * (n - 1 - j) as f64 / (n - 1) as f64).cos())
            .collect();

        self.weights[0] = Self::compute_barycentric_weights(n);

        Ok(())
    }

    fn interpolate(
        &self,
        data: &InterpData1D<D>,
        point: &[f64; 1],
    ) -> Result<f64, InterpolateError> {
        let x = point[0];
        let x_coords = data.grid[0].as_slice().unwrap();
        let f_values = data.values.as_slice().unwrap();

        let x_min = *x_coords.first().unwrap();
        let x_max = *x_coords.last().unwrap();

        if (x_max - x_min).abs() < 1e-15 {
            return Ok(f_values[0]);
        }
        let t = 2.0 * (x - x_min) / (x_max - x_min) - 1.0;

        Ok(Self::barycentric_interpolate(
            t,
            &self.t_coords[0],
            f_values,
            &self.weights[0],
        ))
    }

    fn allow_extrapolate(&self) -> bool {
        true
    }
}

impl<D> Strategy2D<D> for LogChebyshevInterpolation<2>
where
    D: Data<Elem = f64> + RawDataClone + Clone,
{
    fn init(&mut self, data: &InterpData2D<D>) -> Result<(), ValidateError> {
        for dim in 0..2 {
            let x_coords = data.grid[dim].as_slice().unwrap();
            let n = x_coords.len();
            if n < 2 {
                return Err(ValidateError::Other(
                    "LogChebyshevInterpolation requires at least 2 grid points per dimension."
                        .to_string(),
                ));
            }
            self.t_coords[dim] = (0..n)
                .map(|j| (PI * (n - 1 - j) as f64 / (n - 1) as f64).cos())
                .collect();
            self.weights[dim] = Self::compute_barycentric_weights(n);
        }
        Ok(())
    }

    fn interpolate(
        &self,
        data: &InterpData2D<D>,
        point: &[f64; 2],
    ) -> Result<f64, InterpolateError> {
        let [x, y] = *point;
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();

        let x_min = *x_coords.first().unwrap();
        let x_max = *x_coords.last().unwrap();
        let y_min = *y_coords.first().unwrap();
        let y_max = *y_coords.last().unwrap();

        let t_x = 2.0 * (x - x_min) / (x_max - x_min) - 1.0;
        let t_y = 2.0 * (y - y_min) / (y_max - y_min) - 1.0;

        let x_coeffs = Self::barycentric_coefficients(t_x, &self.t_coords[0], &self.weights[0]);
        let y_coeffs = Self::barycentric_coefficients(t_y, &self.t_coords[1], &self.weights[1]);

        let mut result = 0.0;
        for (i, &x_coeff) in x_coeffs.iter().enumerate() {
            for (j, &y_coeff) in y_coeffs.iter().enumerate() {
                result += x_coeff * y_coeff * data.values[[i, j]];
            }
        }

        Ok(result)
    }

    fn allow_extrapolate(&self) -> bool {
        true
    }
}

impl<D> Strategy3D<D> for LogChebyshevInterpolation<3>
where
    D: Data<Elem = f64> + RawDataClone + Clone,
{
    fn init(&mut self, data: &InterpData3D<D>) -> Result<(), ValidateError> {
        for dim in 0..3 {
            let x_coords = data.grid[dim].as_slice().unwrap();
            let n = x_coords.len();
            if n < 2 {
                return Err(ValidateError::Other(
                    "LogChebyshevInterpolation requires at least 2 grid points per dimension."
                        .to_string(),
                ));
            }
            self.t_coords[dim] = (0..n)
                .map(|j| (PI * (n - 1 - j) as f64 / (n - 1) as f64).cos())
                .collect();
            self.weights[dim] = Self::compute_barycentric_weights(n);
        }
        Ok(())
    }

    fn interpolate(
        &self,
        data: &InterpData3D<D>,
        point: &[f64; 3],
    ) -> Result<f64, InterpolateError> {
        let [x, y, z] = *point;
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();
        let z_coords = data.grid[2].as_slice().unwrap();

        let x_min = *x_coords.first().unwrap();
        let x_max = *x_coords.last().unwrap();
        let y_min = *y_coords.first().unwrap();
        let y_max = *y_coords.last().unwrap();
        let z_min = *z_coords.first().unwrap();
        let z_max = *z_coords.last().unwrap();

        let t_x = 2.0 * (x - x_min) / (x_max - x_min) - 1.0;
        let t_y = 2.0 * (y - y_min) / (y_max - y_min) - 1.0;
        let t_z = 2.0 * (z - z_min) / (z_max - z_min) - 1.0;

        let x_coeffs = Self::barycentric_coefficients(t_x, &self.t_coords[0], &self.weights[0]);
        let y_coeffs = Self::barycentric_coefficients(t_y, &self.t_coords[1], &self.weights[1]);
        let z_coeffs = Self::barycentric_coefficients(t_z, &self.t_coords[2], &self.weights[2]);

        let mut result = 0.0;
        for (i, &x_coeff) in x_coeffs.iter().enumerate() {
            for (j, &y_coeff) in y_coeffs.iter().enumerate() {
                for (k, &z_coeff) in z_coeffs.iter().enumerate() {
                    result += x_coeff * y_coeff * z_coeff * data.values[[i, j, k]];
                }
            }
        }

        Ok(result)
    }

    fn allow_extrapolate(&self) -> bool {
        true
    }
}

/// Implements a global N-dimensional batch interpolation using Chebyshev polynomials
/// with logarithmic coordinate scaling.
///
/// This strategy is optimized for interpolating multiple points at once by leveraging
/// matrix operations with `ndarray`.
///
/// TODO: Potentially merge this with `LogChebyshevInterpolation`.
#[derive(Debug, Clone)]
pub struct LogChebyshevBatchInterpolation<const DIM: usize> {
    // Pre-computed weights for the barycentric formula for each dimension.
    weights: [Vec<f64>; DIM],
    // Grid points in the t-domain [-1, 1] for each dimension.
    t_coords: [Vec<f64>; DIM],
}

impl<const DIM: usize> Default for LogChebyshevBatchInterpolation<DIM> {
    fn default() -> Self {
        Self {
            weights: std::array::from_fn(|_| Vec::new()),
            t_coords: std::array::from_fn(|_| Vec::new()),
        }
    }
}

impl<const DIM: usize> Serialize for LogChebyshevBatchInterpolation<DIM> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("LogChebyshevBatchInterpolation", 2)?;
        state.serialize_field("weights", &self.weights.as_slice())?;
        state.serialize_field("t_coords", &self.t_coords.as_slice())?;
        state.end()
    }
}

impl<'de, const DIM: usize> Deserialize<'de> for LogChebyshevBatchInterpolation<DIM> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            weights: Vec<Vec<f64>>,
            t_coords: Vec<Vec<f64>>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let weights = helper.weights.try_into().map_err(|v: Vec<Vec<f64>>| {
            serde::de::Error::invalid_length(v.len(), &"an array of the correct length")
        })?;
        let t_coords = helper.t_coords.try_into().map_err(|v: Vec<Vec<f64>>| {
            serde::de::Error::invalid_length(v.len(), &"an array of the correct length")
        })?;

        Ok(Self { weights, t_coords })
    }
}

impl<const DIM: usize> LogChebyshevBatchInterpolation<DIM> {
    /// Computes the barycentric weights for a given set of Chebyshev points.
    /// The formula for the weights is `w_j = (-1)^j * delta_j`, where `delta_j`
    /// is 1/2 for the first and last points, and 1 otherwise.
    fn compute_barycentric_weights(n: usize) -> Vec<f64> {
        let mut weights = vec![1.0; n];
        (0..n).for_each(|j| {
            if j % 2 == 1 {
                weights[j] = -1.0;
            }
        });
        weights[0] *= 0.5;

        if n > 1 {
            weights[n - 1] *= 0.5;
        }

        weights
    }

    /// Compute barycentric coefficients for multiple points in batch
    fn barycentric_coefficients(
        t_values: &[f64],
        t_coords: &[f64],
        weights: &[f64],
    ) -> Array2<f64> {
        let num_points = t_values.len();
        let num_coords = t_coords.len();
        let mut coeffs = Array2::<f64>::zeros((num_points, num_coords));

        for (p, &t) in t_values.iter().enumerate() {
            let mut found_exact = false;
            for (j, &t_j) in t_coords.iter().enumerate() {
                if (t - t_j).abs() < 1e-15 {
                    coeffs[[p, j]] = 1.0;
                    found_exact = true;
                    break;
                }
            }

            if !found_exact {
                let mut terms = Vec::with_capacity(num_coords);
                for (j, &t_j) in t_coords.iter().enumerate() {
                    terms.push(weights[j] / (t - t_j));
                }

                let sum: f64 = terms.iter().sum();

                for (j, &term) in terms.iter().enumerate() {
                    coeffs[[p, j]] = term / sum;
                }
            }
        }

        coeffs
    }
}

impl LogChebyshevBatchInterpolation<1> {
    pub fn init<D>(&mut self, data: &InterpData1D<D>) -> Result<(), ValidateError>
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let x_coords = data.grid[0].as_slice().unwrap();
        let n = x_coords.len();
        if n < 2 {
            return Err(ValidateError::Other(
                "LogChebyshevBatchInterpolation requires at least 2 grid points.".to_string(),
            ));
        }

        self.t_coords[0] = (0..n)
            .map(|j| (PI * (n - 1 - j) as f64 / (n - 1) as f64).cos())
            .collect();

        self.weights[0] = Self::compute_barycentric_weights(n);

        Ok(())
    }

    pub fn interpolate<D>(
        &self,
        data: &InterpData1D<D>,
        points: &[[f64; 1]],
    ) -> Result<Vec<f64>, InterpolateError>
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let x_coords = data.grid[0].as_slice().unwrap();
        let f_values = data.values.to_owned();

        let x_min = *x_coords.first().unwrap();
        let x_max = *x_coords.last().unwrap();

        let mut t_x_vals = Vec::with_capacity(points.len());
        for &[x] in points {
            t_x_vals.push(2.0 * (x - x_min) / (x_max - x_min) - 1.0);
        }

        let c_x = Self::barycentric_coefficients(&t_x_vals, &self.t_coords[0], &self.weights[0]);
        let results = c_x.dot(&f_values);

        Ok(results.to_vec())
    }
}

impl LogChebyshevBatchInterpolation<2> {
    pub fn init<D>(&mut self, data: &InterpData2D<D>) -> Result<(), ValidateError>
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        for dim in 0..2 {
            let x_coords = data.grid[dim].as_slice().unwrap();
            let n = x_coords.len();
            if n < 2 {
                return Err(ValidateError::Other(
                    "LogChebyshevBatchInterpolation requires at least 2 grid points per dimension."
                        .to_string(),
                ));
            }
            self.t_coords[dim] = (0..n)
                .map(|j| (PI * (n - 1 - j) as f64 / (n - 1) as f64).cos())
                .collect();
            self.weights[dim] = Self::compute_barycentric_weights(n);
        }

        Ok(())
    }

    pub fn interpolate<D>(
        &self,
        data: &InterpData2D<D>,
        points: &[[f64; 2]],
    ) -> Result<Vec<f64>, InterpolateError>
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();

        let x_min = *x_coords.first().unwrap();
        let x_max = *x_coords.last().unwrap();
        let y_min = *y_coords.first().unwrap();
        let y_max = *y_coords.last().unwrap();

        let mut t_x_vals = Vec::with_capacity(points.len());
        let mut t_y_vals = Vec::with_capacity(points.len());
        for &[x, y] in points {
            t_x_vals.push(2.0 * (x - x_min) / (x_max - x_min) - 1.0);
            t_y_vals.push(2.0 * (y - y_min) / (y_max - y_min) - 1.0);
        }

        let c_x = Self::barycentric_coefficients(&t_x_vals, &self.t_coords[0], &self.weights[0]);
        let c_y = Self::barycentric_coefficients(&t_y_vals, &self.t_coords[1], &self.weights[1]);
        let v = data.values.to_owned();
        let results = (&c_x.dot(&v) * &c_y).sum_axis(Axis(1));

        Ok(results.to_vec())
    }
}

impl LogChebyshevBatchInterpolation<3> {
    pub fn init<D>(&mut self, data: &InterpData3D<D>) -> Result<(), ValidateError>
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        for dim in 0..3 {
            let x_coords = data.grid[dim].as_slice().unwrap();
            let n = x_coords.len();
            if n < 2 {
                return Err(ValidateError::Other(
                    "LogChebyshevBatchInterpolation requires at least 2 grid points per dimension."
                        .to_string(),
                ));
            }
            self.t_coords[dim] = (0..n)
                .map(|j| (PI * (n - 1 - j) as f64 / (n - 1) as f64).cos())
                .collect();
            self.weights[dim] = Self::compute_barycentric_weights(n);
        }
        Ok(())
    }

    pub fn interpolate<D>(
        &self,
        data: &InterpData3D<D>,
        points: &[[f64; 3]],
    ) -> Result<Vec<f64>, InterpolateError>
    where
        D: Data<Elem = f64> + RawDataClone + Clone,
    {
        let x_coords = data.grid[0].as_slice().unwrap();
        let y_coords = data.grid[1].as_slice().unwrap();
        let z_coords = data.grid[2].as_slice().unwrap();

        let x_min = *x_coords.first().unwrap();
        let x_max = *x_coords.last().unwrap();
        let y_min = *y_coords.first().unwrap();
        let y_max = *y_coords.last().unwrap();
        let z_min = *z_coords.first().unwrap();
        let z_max = *z_coords.last().unwrap();

        let mut t_x_vals = Vec::with_capacity(points.len());
        let mut t_y_vals = Vec::with_capacity(points.len());
        let mut t_z_vals = Vec::with_capacity(points.len());

        for &[x, y, z] in points {
            t_x_vals.push(2.0 * (x - x_min) / (x_max - x_min) - 1.0);
            t_y_vals.push(2.0 * (y - y_min) / (y_max - y_min) - 1.0);
            t_z_vals.push(2.0 * (z - z_min) / (z_max - z_min) - 1.0);
        }

        let c_x = Self::barycentric_coefficients(&t_x_vals, &self.t_coords[0], &self.weights[0]);
        let c_y = Self::barycentric_coefficients(&t_y_vals, &self.t_coords[1], &self.weights[1]);
        let c_z = Self::barycentric_coefficients(&t_z_vals, &self.t_coords[2], &self.weights[2]);

        let v = &data.values;

        let num_points = points.len();
        let (nx, ny, nz) = (x_coords.len(), y_coords.len(), z_coords.len());

        let v_flat = v.to_owned().into_shape_with_order((nx, ny * nz)).unwrap();
        let temp1 = c_x.dot(&v_flat);
        let temp1_3d = temp1.into_shape_with_order((num_points, ny, nz)).unwrap();

        let mut results = Vec::with_capacity(num_points);
        for p in 0..num_points {
            let temp_slice = temp1_3d.index_axis(Axis(0), p);
            let cy_slice = c_y.row(p);
            let cz_slice = c_z.row(p);

            let temp2 = cy_slice.dot(&temp_slice);
            let result = cz_slice.dot(&temp2);
            results.push(result);
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use ndarray::{Array1, Array2, Array3, OwnedRepr};
    use ninterp::data::{InterpData1D, InterpData2D};
    use ninterp::interpolator::{Extrapolate, InterpND};
    use ninterp::prelude::Interpolator;
    use ninterp::strategy::Linear;
    use std::f64::consts::PI;

    const EPSILON: f64 = 1e-9;

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() < tolerance,
            "Expected {}, got {} (diff: {})",
            expected,
            actual,
            (actual - expected).abs()
        );
    }

    fn create_target_data_2d(max_num: i32) -> Vec<f64> {
        (1..=max_num)
            .flat_map(|i| (1..=max_num).map(move |j| (i * j) as f64))
            .collect()
    }

    fn create_logspaced(start: f64, stop: f64, n: usize) -> Vec<f64> {
        (0..n)
            .map(|value| {
                let t = value as f64 / (n - 1) as f64;
                start * (stop / start).powf(t)
            })
            .collect()
    }

    fn create_test_data_1d(
        q2_values: Vec<f64>,
        alphas_vals: Vec<f64>,
    ) -> InterpData1D<OwnedRepr<f64>> {
        InterpData1D::new(Array1::from(q2_values), Array1::from(alphas_vals)).unwrap()
    }

    fn create_test_data_2d(
        x_coords: Vec<f64>,
        y_coords: Vec<f64>,
        values: Vec<f64>,
    ) -> InterpData2D<OwnedRepr<f64>> {
        let shape = (x_coords.len(), y_coords.len());
        let values_array = Array2::from_shape_vec(shape, values).unwrap();
        InterpData2D::new(x_coords.into(), y_coords.into(), values_array).unwrap()
    }

    fn create_test_data_3d(
        x_coords: Vec<f64>,
        y_coords: Vec<f64>,
        z_coords: Vec<f64>,
        values: Vec<f64>,
    ) -> InterpData3D<OwnedRepr<f64>> {
        let shape = (x_coords.len(), y_coords.len(), z_coords.len());
        let values_array = Array3::from_shape_vec(shape, values).unwrap();
        InterpData3D::new(
            x_coords.into(),
            y_coords.into(),
            z_coords.into(),
            values_array,
        )
        .unwrap()
    }

    fn create_cheby_grid(n_points: i32, x_min: f64, x_max: f64) -> Vec<f64> {
        let u_min = x_min.ln();
        let u_max = x_max.ln();
        (0..n_points)
            .map(|j| {
                let t_j = (PI * (n_points - 1 - j) as f64 / (n_points - 1) as f64).cos();
                let u_j = u_min + (u_max - u_min) * (t_j + 1.0) / 2.0;
                u_j.exp()
            })
            .collect::<Vec<f64>>()
    }

    #[test]
    fn test_linear_interpolate() {
        let test_cases = [
            // (x1, x2, y1, y2, x, expected)
            (0.0, 1.0, 0.0, 10.0, 0.5, 5.0),
            (0.0, 10.0, 0.0, 100.0, 2.5, 25.0),
            (0.0, 1.0, 0.0, 10.0, 0.0, 0.0),   // At start endpoint
            (0.0, 1.0, 0.0, 10.0, 1.0, 10.0),  // At end endpoint
            (5.0, 5.0, 10.0, 20.0, 5.0, 10.0), // x1 == x2 case
        ];

        for (x1, x2, y1, y2, x, expected) in test_cases {
            let result = BilinearInterpolation::linear_interpolate(x1, x2, y1, y2, x);
            assert_close(result, expected, EPSILON);
        }
    }

    #[test]
    fn test_bilinear_interpolation() {
        let data = create_test_data_2d(
            vec![0.0, 1.0, 2.0],
            vec![0.0, 1.0, 2.0],
            vec![0.0, 1.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0],
        );

        let test_cases = [
            ([0.5, 0.5], 1.0),
            ([1.0, 1.0], 2.0), // Grid point
            ([0.25, 0.75], 1.0),
        ];

        for (point, expected) in test_cases {
            let result = BilinearInterpolation.interpolate(&data, &point).unwrap();
            assert_close(result, expected, EPSILON);
        }
    }

    #[test]
    fn test_log_bilinear_interpolation() {
        let data = create_test_data_2d(
            vec![1.0f64.ln(), 10.0f64.ln(), 100.0f64.ln()],
            vec![1.0f64.ln(), 10.0f64.ln(), 100.0f64.ln()],
            vec![0.0, 1.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0],
        );
        LogBilinearInterpolation.init(&data).unwrap();

        let test_cases = [
            ([3.16227766f64.ln(), 3.16227766f64.ln()], 1.0), // sqrt(10)
            ([10.0f64.ln(), 10.0f64.ln()], 2.0),             // Grid point
            ([1.77827941f64.ln(), 5.62341325f64.ln()], 1.0), // 10^0.25, 10^0.75
        ];

        for (point, expected) in test_cases {
            let result = LogBilinearInterpolation.interpolate(&data, &point).unwrap();
            assert_close(result, expected, EPSILON);
        }
    }

    #[test]
    fn test_log_tricubic_interpolation() {
        let x_coords = create_logspaced(1e-5, 1e-3, 6);
        let y_coords = create_logspaced(1e2, 1e4, 6);
        let z_coords = vec![1.0, 5.0, 25.0, 100.0, 150.0, 200.0];
        let values: Vec<f64> = x_coords
            .iter()
            .cartesian_product(y_coords.iter())
            .cartesian_product(z_coords.iter())
            .map(|((&a, &b), &c)| a * b * c)
            .collect();

        let values_ln: Vec<f64> = values.iter().map(|val| val.ln()).collect();
        let interp_data_ln = create_test_data_3d(
            x_coords.iter().map(|v| v.ln()).collect(),
            y_coords.iter().map(|v| v.ln()).collect(),
            z_coords.iter().map(|v| v.ln()).collect(),
            values_ln.clone(),
        );

        let mut strategy = LogTricubicInterpolation;
        strategy.init(&interp_data_ln).unwrap();

        let point: [f64; 3] = [1e-4, 2e3, 25.0];
        let log_point = [point[0].ln(), point[1].ln(), point[2].ln()];
        let expected: f64 = point.iter().product();
        let result = strategy
            .interpolate(&interp_data_ln, &log_point)
            .unwrap()
            .exp();
        assert_close(result, expected, EPSILON);

        let interp_data_arr =
            Array3::from_shape_vec((x_coords.len(), y_coords.len(), z_coords.len()), values)
                .unwrap();
        let nd_interp = InterpND::new(
            vec![x_coords.into(), y_coords.into(), z_coords.into()],
            interp_data_arr.into_dyn(),
            Linear,
            Extrapolate::Error,
        )
        .unwrap();
        let nd_interp_res = nd_interp.interpolate(&point).unwrap();
        assert_close(nd_interp_res, expected, EPSILON);
    }

    #[test]
    fn test_alphas_cubic_interpolation() {
        let q_values = [1.0f64, 2.0, 3.0, 4.0, 5.0];
        let alphas_vals = vec![0.1, 0.11, 0.12, 0.13, 0.14];
        let logq2_values: Vec<f64> = q_values.iter().map(|&q| (q * q).ln()).collect();
        let data = create_test_data_1d(logq2_values, alphas_vals);
        let alphas_cubic = AlphaSCubicInterpolation;

        // Test within interpolation range
        let result = alphas_cubic.interpolate(&data, &[2.25f64.ln()]).unwrap();
        assert!(result > 0.1 && result < 0.14);

        // Test at grid point
        let result = alphas_cubic.interpolate(&data, &[4.0f64.ln()]).unwrap();
        assert_close(result, 0.11, EPSILON);

        // Test extrapolation below range
        let result = alphas_cubic.interpolate(&data, &[0.5f64.ln()]).unwrap();
        assert!(result < 0.1);

        // Test extrapolation above range
        let result = alphas_cubic.interpolate(&data, &[30.0f64.ln()]).unwrap();
        assert_close(result, 0.14, EPSILON);
    }

    #[test]
    fn test_find_bicubic_interval() {
        let coords = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let test_cases = [
            (1.5, Ok(0)),
            (3.5, Ok(2)),
            (2.0, Ok(1)),   // At knot point
            (1.0, Ok(0)),   // At boundary
            (4.99, Ok(3)),  // Near boundary
            (0.5, Err(())), // Out of bounds
            (5.5, Err(())), // Out of bounds
        ];

        for (value, expected) in test_cases {
            let result = LogBicubicInterpolation::find_bicubic_interval(&coords, value);
            match expected {
                Ok(expected_idx) => assert_eq!(result.unwrap(), expected_idx),
                Err(_) => assert!(result.is_err()),
            }
        }
    }

    #[test]
    fn test_hermite_cubic_interpolate_from_coeffs() {
        let test_cases = [
            // Linear function x: coeffs = [0, 0, 1, 0]
            ([0.0, 0.0, 1.0, 0.0], 0.5, 0.5),
            ([0.0, 0.0, 1.0, 0.0], 1.0, 1.0),
            // Constant function 5: coeffs = [0, 0, 0, 5]
            ([0.0, 0.0, 0.0, 5.0], 0.5, 5.0),
            // Cubic function x^3: coeffs = [1, 0, 0, 0]
            ([1.0, 0.0, 0.0, 0.0], 2.0, 8.0),
            ([1.0, 0.0, 0.0, 0.0], 0.5, 0.125),
            // Complex polynomial 2x^3 - 3x^2 + x + 4
            ([2.0, -3.0, 1.0, 4.0], 1.0, 4.0),
            ([2.0, -3.0, 1.0, 4.0], 0.0, 4.0),
            ([2.0, -3.0, 1.0, 4.0], 2.0, 10.0),
        ];

        for (coeffs, x, expected) in test_cases {
            let result = LogBicubicInterpolation::hermite_cubic_interpolate_from_coeffs(x, &coeffs);
            assert_close(result, expected, EPSILON);
        }
    }

    #[test]
    fn test_log_bicubic_interpolation() {
        let target_data = create_target_data_2d(4);
        let data = create_test_data_2d(
            vec![1.0f64.ln(), 10.0f64.ln(), 100.0f64.ln(), 1000.0f64.ln()],
            vec![1.0f64.ln(), 10.0f64.ln(), 100.0f64.ln(), 1000.0f64.ln()],
            target_data,
        );

        let mut log_bicubic = LogBicubicInterpolation::default();
        log_bicubic.init(&data).unwrap();

        let test_cases = [
            ([10.0f64.ln(), 10.0f64.ln()], 4.0),              // Grid point
            ([3.16227766f64.ln(), 3.16227766f64.ln()], 2.25), // sqrt(10)
            ([31.6227766f64.ln(), 31.6227766f64.ln()], 6.25), // 10^1.5
        ];

        for (point, expected) in test_cases {
            let result = log_bicubic.interpolate(&data, &point).unwrap();
            assert_close(result, expected, EPSILON);
        }
    }

    #[test]
    fn test_ddlogq_derivatives() {
        let data = create_test_data_1d(
            vec![1.0f64.ln(), 2.0f64.ln(), 3.0f64.ln(), 4.0f64.ln()],
            vec![0.1, 0.2, 0.3, 0.4],
        );

        let expected_forward = 0.1 / (2.0f64.ln() - 1.0f64.ln());
        assert_close(
            AlphaSCubicInterpolation::ddlogq_forward(&data, 0),
            expected_forward,
            EPSILON,
        );

        let expected_backward = 0.1 / (2.0f64.ln() - 1.0f64.ln());
        assert_close(
            AlphaSCubicInterpolation::ddlogq_backward(&data, 1),
            expected_backward,
            EPSILON,
        );

        let expected_central =
            0.5 * (0.1 / (3.0f64.ln() - 2.0f64.ln()) + 0.1 / (2.0f64.ln() - 1.0f64.ln()));
        assert_close(
            AlphaSCubicInterpolation::ddlogq_central(&data, 1),
            expected_central,
            EPSILON,
        );
    }

    #[test]
    fn test_ilogq2below() {
        let data = create_test_data_1d(
            vec![
                1.0f64.ln(),
                2.0f64.ln(),
                3.0f64.ln(),
                4.0f64.ln(),
                5.0f64.ln(),
            ],
            vec![0.1, 0.2, 0.3, 0.4, 0.5],
        );

        let test_cases = [
            (1.5f64.ln(), 0),
            (2.0f64.ln(), 1),
            (3.9f64.ln(), 2), // Within range
            (1.0f64.ln(), 0),
            (5.0f64.ln(), 3), // At boundaries
        ];

        for (q2_val, expected_idx) in test_cases {
            assert_eq!(
                AlphaSCubicInterpolation::ilogq2below(&data, q2_val),
                expected_idx
            );
        }

        let data_small = create_test_data_1d(vec![1.0f64.ln(), 2.0f64.ln()], vec![0.1, 0.2]);
        assert_eq!(
            AlphaSCubicInterpolation::ilogq2below(&data_small, 2.0f64.ln()),
            0
        );

        let data_with_mid = create_test_data_1d(
            vec![1.0f64.ln(), 2.0f64.ln(), 3.0f64.ln()],
            vec![0.1, 0.2, 0.3],
        );
        assert_eq!(
            AlphaSCubicInterpolation::ilogq2below(&data_with_mid, 2.0f64.ln()),
            1
        );

        let data_single = create_test_data_1d(vec![1.0f64.ln()], vec![0.1]);

        let result = std::panic::catch_unwind(|| {
            AlphaSCubicInterpolation::ilogq2below(&data_single, 0.5f64.ln());
        });
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| {
            AlphaSCubicInterpolation::ilogq2below(&data_single, 1.5f64.ln());
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_log_chebyshev_interpolation_1d() {
        let n = 21;
        let x_min: f64 = 0.1;
        let x_max: f64 = 10.0;
        let x_coords = create_cheby_grid(n, x_min, x_max);

        let f_values: Vec<f64> = x_coords.iter().map(|&x| x.ln()).collect();
        let data = create_test_data_1d(x_coords.iter().map(|v| v.ln()).collect(), f_values);
        let mut cheby = LogChebyshevInterpolation::<1>::default();
        cheby.init(&data).unwrap();

        let x_test: f64 = 2.5;
        let expected = x_test.ln();
        let result = cheby.interpolate(&data, &[x_test.ln()]).unwrap();
        assert_close(result, expected, EPSILON);

        let x_test_grid = data.grid[0].as_slice().unwrap()[n as usize / 2];
        let expected_grid = x_test_grid;
        let result_grid = cheby.interpolate(&data, &[x_test_grid]).unwrap();
        assert_close(result_grid, expected_grid, EPSILON);
    }

    #[test]
    fn test_log_chebyshev_interpolation_2d() {
        let n = 11;
        let x_coords = create_cheby_grid(n, 0.1, 10.0);
        let y_coords = create_cheby_grid(n, 0.1, 10.0);

        let f_values: Vec<f64> = x_coords
            .iter()
            .flat_map(|&x| y_coords.iter().map(move |&y| x.ln() + y.ln()))
            .collect();

        let data = create_test_data_2d(
            x_coords.iter().map(|v| v.ln()).collect(),
            y_coords.iter().map(|v| v.ln()).collect(),
            f_values,
        );
        let mut cheby = LogChebyshevInterpolation::<2>::default();
        cheby.init(&data).unwrap();

        let x_test: f64 = 2.5;
        let y_test: f64 = 3.5;
        let expected = x_test.ln() + y_test.ln();
        let result = cheby
            .interpolate(&data, &[x_test.ln(), y_test.ln()])
            .unwrap();

        assert_close(result, expected, EPSILON);
    }

    #[test]
    fn test_log_chebyshev_interpolation_3d() {
        let n = 7;
        let x_coords = create_cheby_grid(n, 0.1, 10.0);
        let y_coords = create_cheby_grid(n, 0.1, 10.0);
        let z_coords = create_cheby_grid(n, 0.1, 10.0);

        let f_values: Vec<f64> = x_coords
            .iter()
            .cartesian_product(y_coords.iter())
            .cartesian_product(z_coords.iter())
            .map(|((&x, &y), &z)| x.ln() + y.ln() + z.ln())
            .collect();

        let data = create_test_data_3d(
            x_coords.iter().map(|v| v.ln()).collect(),
            y_coords.iter().map(|v| v.ln()).collect(),
            z_coords.iter().map(|v| v.ln()).collect(),
            f_values,
        );
        let mut cheby = LogChebyshevInterpolation::<3>::default();
        cheby.init(&data).unwrap();

        let x_test: f64 = 2.5;
        let y_test: f64 = 3.5;
        let z_test: f64 = 4.5;
        let expected = x_test.ln() + y_test.ln() + z_test.ln();
        let result = cheby
            .interpolate(&data, &[x_test.ln(), y_test.ln(), z_test.ln()])
            .unwrap();

        assert_close(result, expected, EPSILON);
    }

    #[test]
    fn test_log_chebyshev_batch_interpolation_1d() {
        let n = 21;
        let x_min: f64 = 0.1;
        let x_max: f64 = 10.0;
        let x_coords = create_cheby_grid(n, x_min, x_max);

        let f_values: Vec<f64> = x_coords.iter().map(|&x| x.ln()).collect();
        let data = create_test_data_1d(x_coords.iter().map(|v| v.ln()).collect(), f_values);
        let mut cheby = LogChebyshevBatchInterpolation::<1>::default();
        cheby.init(&data).unwrap();

        let test_points = [[2.5f64.ln()], [5.0f64.ln()], [7.5f64.ln()]];
        let expected: Vec<f64> = test_points.iter().map(|p| p[0]).collect();
        let results = cheby.interpolate(&data, &test_points).unwrap();

        for (res, exp) in results.iter().zip(expected.iter()) {
            assert_close(*res, *exp, EPSILON);
        }
    }

    #[test]
    fn test_log_chebyshev_batch_interpolation_2d() {
        let n = 11;
        let x_coords = create_cheby_grid(n, 0.1, 10.0);
        let y_coords = create_cheby_grid(n, 0.1, 10.0);

        let f_values: Vec<f64> = x_coords
            .iter()
            .flat_map(|&x| y_coords.iter().map(move |&y| x.ln() + y.ln()))
            .collect();

        let data = create_test_data_2d(
            x_coords.iter().map(|v| v.ln()).collect(),
            y_coords.iter().map(|v| v.ln()).collect(),
            f_values,
        );
        let mut cheby = LogChebyshevBatchInterpolation::<2>::default();
        cheby.init(&data).unwrap();

        let test_points = [
            [2.5f64.ln(), 3.5f64.ln()],
            [5.0f64.ln(), 6.0f64.ln()],
            [7.5f64.ln(), 8.5f64.ln()],
        ];
        let expected: Vec<f64> = test_points.iter().map(|p| p[0] + p[1]).collect();
        let results = cheby.interpolate(&data, &test_points).unwrap();

        for (res, exp) in results.iter().zip(expected.iter()) {
            assert_close(*res, *exp, EPSILON);
        }
    }

    #[test]
    fn test_log_chebyshev_batch_interpolation_3d() {
        let n = 7;
        let x_coords = create_cheby_grid(n, 0.1, 10.0);
        let y_coords = create_cheby_grid(n, 0.1, 10.0);
        let z_coords = create_cheby_grid(n, 0.1, 10.0);

        let f_values: Vec<f64> = x_coords
            .iter()
            .cartesian_product(y_coords.iter())
            .cartesian_product(z_coords.iter())
            .map(|((&x, &y), &z)| x.ln() + y.ln() + z.ln())
            .collect();

        let data = create_test_data_3d(
            x_coords.iter().map(|v| v.ln()).collect(),
            y_coords.iter().map(|v| v.ln()).collect(),
            z_coords.iter().map(|v| v.ln()).collect(),
            f_values,
        );
        let mut cheby = LogChebyshevBatchInterpolation::<3>::default();
        cheby.init(&data).unwrap();

        let test_points = [
            [2.5f64.ln(), 3.5f64.ln(), 4.5f64.ln()],
            [5.0f64.ln(), 6.0f64.ln(), 7.0f64.ln()],
            [7.5f64.ln(), 8.5f64.ln(), 9.5f64.ln()],
        ];
        let expected: Vec<f64> = test_points.iter().map(|p| p[0] + p[1] + p[2]).collect();
        let results = cheby.interpolate(&data, &test_points).unwrap();

        for (res, exp) in results.iter().zip(expected.iter()) {
            assert_close(*res, *exp, EPSILON);
        }
    }
}
