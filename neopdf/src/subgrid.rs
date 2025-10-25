//! This module defines the [`SubGrid`] struct and its implementation for PDF grid handling.
//!
//! # Contents
//!
//! - [`ParamRange`], [`RangeParameters`]: Parameter range types for grid axes.
//! - [`SubGrid`]: Represents a region of phase space with a consistent grid and provides
//!   methods for subgrid logic.

use ndarray::{s, Array1, Array6, ArrayD, ArrayView2, IxDyn};
use serde::{Deserialize, Serialize};

use super::interpolator::InterpolationConfig;

/// Represents the valid range of a parameter, with a minimum and maximum value.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct ParamRange {
    /// The minimum value of the parameter.
    pub min: f64,
    /// The maximum value of the parameter.
    pub max: f64,
}

impl ParamRange {
    /// Creates a new `ParamRange`.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value.
    /// * `max` - The maximum value.
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Checks if a given value is within the parameter range (inclusive).
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check.
    ///
    /// # Returns
    ///
    /// `true` if the value is within the range, `false` otherwise.
    pub fn contains(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }
}

/// Represents the parameter ranges for all dimensions.
pub struct RangeParameters {
    /// The range for the nucleon numbers `A`.
    pub nucleons: ParamRange,
    /// The range for the AlphaS values `as`.
    pub alphas: ParamRange,
    /// The range for the xi values.
    pub xi: ParamRange,
    /// The range for the delta values.
    pub delta: ParamRange,
    /// The range for the transverse momentum `kT`.
    pub kt: ParamRange,
    /// The range for the momentum fraction `x`.
    pub x: ParamRange,
    /// The range for the energy scale squared `q2`.
    pub q2: ParamRange,
}

impl RangeParameters {
    /// Creates a new `RangeParameters`.
    ///
    /// # Arguments
    ///
    /// * `nucleons` - The `ParamRange` for the nuleon numbers `A`.
    /// * `alphas` - The `ParamRange` for the strong coupling `as`.
    /// * `xi` - The `ParamRange` for the xi values.
    /// * `delta` - The `ParamRange` for the delta values.
    /// * `kt` - The `ParamRange` for the transverse momentum `kT`.
    /// * `x` - The `ParamRange` for the momentum fraction `x`.
    /// * `q2` - The `ParamRange` for the energy scale `q2`.
    pub fn new(
        nucleons: ParamRange,
        alphas: ParamRange,
        xi: ParamRange,
        delta: ParamRange,
        kt: ParamRange,
        x: ParamRange,
        q2: ParamRange,
    ) -> Self {
        Self {
            nucleons,
            alphas,
            xi,
            delta,
            kt,
            x,
            q2,
        }
    }
}

/// Enum to hold either 6D or 8D grid data for backward compatibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GridData {
    /// 6-dimensional grid data: [nucleons, alphas, pids, kT, x, Q2].
    Grid6D(Array6<f64>),
    /// 8-dimensional grid data: [nucleons, alphas, xi, delta, kT, pids, x, Q2].
    Grid8D(ArrayD<f64>),
}

impl GridData {
    /// Returns a view of the grid data for slicing operations.
    pub fn view(&self) -> ndarray::ArrayViewD<'_, f64> {
        match self {
            GridData::Grid6D(arr) => arr.view().into_dyn(),
            GridData::Grid8D(arr) => arr.view(),
        }
    }

    /// Returns a reference to the 6D grid, panicking if it's 8D.
    pub fn as_6d(&self) -> &Array6<f64> {
        match self {
            GridData::Grid6D(arr) => arr,
            GridData::Grid8D(_) => panic!("Cannot convert 8D grid to 6D"),
        }
    }

    /// Returns a reference to the 8D grid, panicking if it's 6D.
    pub fn as_8d(&self) -> &ArrayD<f64> {
        match self {
            GridData::Grid6D(_) => panic!("Cannot convert 6D grid to 8D"),
            GridData::Grid8D(arr) => arr,
        }
    }
}

/// Stores the PDF grid data for a single subgrid.
///
/// A subgrid represents a region of the phase space with a consistent
/// grid of `x` and `Q2` values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubGrid {
    /// Array of `x` values (momentum fraction).
    pub xs: Array1<f64>,
    /// Array of `Q²` values (energy scale squared).
    pub q2s: Array1<f64>,
    /// Array of `kT` values (transverse momentum).
    pub kts: Array1<f64>,
    /// Array of `xi` values.
    pub xis: Array1<f64>,
    /// Array of `delta` values.
    pub deltas: Array1<f64>,
    /// Grid data (either 6D or 7D for backward compatibility).
    pub grid: GridData,
    /// Array of nucleon number values.
    pub nucleons: Array1<f64>,
    /// Array of alpha_s values.
    pub alphas: Array1<f64>,
    /// The valid range for the `nucleons` parameter in this subgrid.
    pub nucleons_range: ParamRange,
    /// The valid range for the `AlphaS` parameter in this subgrid.
    pub alphas_range: ParamRange,
    /// The valid range for the `xi` parameter in this subgrid.
    pub xi_range: ParamRange,
    /// The valid range for the `delta` parameter in this subgrid.
    pub delta_range: ParamRange,
    /// The valid range for the `kT` parameter in this subgrid.
    pub kt_range: ParamRange,
    /// The valid range for the `x` parameter in this subgrid.
    pub x_range: ParamRange,
    /// The valid range for the `q2` parameter in this subgrid.
    pub q2_range: ParamRange,
}

impl SubGrid {
    /// Creates a new 6D `SubGrid` from raw data (for backward compatibility).
    ///
    /// # Arguments
    ///
    /// * `nucleon_numbers` - A vector of nucleon numbers.
    /// * `alphas_values` - A vector of alpha_s values.
    /// * `kt_subgrid` - A vector of `kT` values.
    /// * `x_subgrid` - A vector of `x` values.
    /// * `q2_subgrid` - A vector of `q2` values.
    /// * `nflav` - The number of quark flavors.
    /// * `grid_data` - A flat vector of grid data points.
    ///
    /// # Panics
    ///
    /// Panics if the grid data cannot be reshaped to the expected dimensions.
    pub fn new(
        nucleon_numbers: Vec<f64>,
        alphas_values: Vec<f64>,
        kt_subgrid: Vec<f64>,
        x_subgrid: Vec<f64>,
        q2_subgrid: Vec<f64>,
        nflav: usize,
        grid_data: Vec<f64>,
    ) -> Self {
        let xs_range = ParamRange::new(*x_subgrid.first().unwrap(), *x_subgrid.last().unwrap());
        let q2s_range = ParamRange::new(*q2_subgrid.first().unwrap(), *q2_subgrid.last().unwrap());
        let kts_range = ParamRange::new(*kt_subgrid.first().unwrap(), *kt_subgrid.last().unwrap());
        let ncs_range = ParamRange::new(
            *nucleon_numbers.first().unwrap(),
            *nucleon_numbers.last().unwrap(),
        );
        let as_range = ParamRange::new(
            *alphas_values.first().unwrap(),
            *alphas_values.last().unwrap(),
        );

        let subgrid = Array6::from_shape_vec(
            (
                nucleon_numbers.len(),
                alphas_values.len(),
                kt_subgrid.len(),
                x_subgrid.len(),
                q2_subgrid.len(),
                nflav,
            ),
            grid_data,
        )
        .expect("Failed to create grid")
        .permuted_axes([0, 1, 5, 2, 3, 4])
        .as_standard_layout()
        .to_owned();

        Self {
            xs: Array1::from_vec(x_subgrid),
            q2s: Array1::from_vec(q2_subgrid),
            kts: Array1::from_vec(kt_subgrid),
            xis: Array1::from_vec(vec![0.0]),
            deltas: Array1::from_vec(vec![0.0]),
            grid: GridData::Grid6D(subgrid),
            nucleons: Array1::from_vec(nucleon_numbers),
            alphas: Array1::from_vec(alphas_values),
            nucleons_range: ncs_range,
            alphas_range: as_range,
            xi_range: ParamRange::new(0.0, 0.0),
            delta_range: ParamRange::new(0.0, 0.0),
            kt_range: kts_range,
            x_range: xs_range,
            q2_range: q2s_range,
        }
    }

    /// Creates a new 8D `SubGrid` from raw data.
    ///
    /// # Arguments
    ///
    /// * `nucleon_numbers` - A vector of nucleon numbers.
    /// * `alphas_values` - A vector of alpha_s values.
    /// * `xi_values` - A vector of xi values.
    /// * `delta_values` - A vector of delta values.
    /// * `kt_subgrid` - A vector of `kT` values.
    /// * `x_subgrid` - A vector of `x` values.
    /// * `q2_subgrid` - A vector of `q2` values.
    /// * `nflav` - The number of quark flavors.
    /// * `grid_data` - A flat vector of grid data points.
    ///
    /// # Panics
    ///
    /// Panics if the grid data cannot be reshaped to the expected dimensions.
    #[allow(clippy::too_many_arguments)]
    pub fn new_8d(
        nucleon_numbers: Vec<f64>,
        alphas_values: Vec<f64>,
        xi_values: Vec<f64>,
        delta_values: Vec<f64>,
        kt_subgrid: Vec<f64>,
        x_subgrid: Vec<f64>,
        q2_subgrid: Vec<f64>,
        nflav: usize,
        grid_data: Vec<f64>,
    ) -> Self {
        let xs_range = ParamRange::new(*x_subgrid.first().unwrap(), *x_subgrid.last().unwrap());
        let q2s_range = ParamRange::new(*q2_subgrid.first().unwrap(), *q2_subgrid.last().unwrap());
        let kts_range = ParamRange::new(*kt_subgrid.first().unwrap(), *kt_subgrid.last().unwrap());
        let xis_range = ParamRange::new(*xi_values.first().unwrap(), *xi_values.last().unwrap());
        let deltas_range = ParamRange::new(
            *delta_values.first().unwrap(),
            *delta_values.last().unwrap(),
        );
        let ncs_range = ParamRange::new(
            *nucleon_numbers.first().unwrap(),
            *nucleon_numbers.last().unwrap(),
        );
        let as_range = ParamRange::new(
            *alphas_values.first().unwrap(),
            *alphas_values.last().unwrap(),
        );

        let shape = IxDyn(&[
            nucleon_numbers.len(),
            alphas_values.len(),
            xi_values.len(),
            delta_values.len(),
            kt_subgrid.len(),
            nflav,
            x_subgrid.len(),
            q2_subgrid.len(),
        ]);

        let subgrid = ArrayD::from_shape_vec(shape, grid_data).expect("Failed to create 8D grid");

        Self {
            xs: Array1::from_vec(x_subgrid),
            q2s: Array1::from_vec(q2_subgrid),
            kts: Array1::from_vec(kt_subgrid),
            xis: Array1::from_vec(xi_values),
            deltas: Array1::from_vec(delta_values),
            grid: GridData::Grid8D(subgrid),
            nucleons: Array1::from_vec(nucleon_numbers),
            alphas: Array1::from_vec(alphas_values),
            nucleons_range: ncs_range,
            alphas_range: as_range,
            xi_range: xis_range,
            delta_range: deltas_range,
            kt_range: kts_range,
            x_range: xs_range,
            q2_range: q2s_range,
        }
    }

    /// Checks if a point (..., `x`, `q2`) is within the boundaries of this subgrid.
    ///
    /// # Arguments
    ///
    /// * `points` - A slice of coordinates. The order is assumed to be
    ///   `(A, alpha_s, kT, x, Q2)`, with dimensions only present if they are part of
    ///   the grid.
    ///
    /// # Returns
    ///
    /// `true` if the point is within the subgrid, `false` otherwise.
    pub fn contains_point(&self, points: &[f64]) -> bool {
        let (expected_len, ranges) = match self.interpolation_config() {
            InterpolationConfig::TwoD => (2, vec![]),
            InterpolationConfig::ThreeDNucleons => (3, vec![&self.nucleons_range]),
            InterpolationConfig::ThreeDAlphas => (3, vec![&self.alphas_range]),
            InterpolationConfig::ThreeDXi => (3, vec![&self.xi_range]),
            InterpolationConfig::ThreeDDelta => (3, vec![&self.delta_range]),
            InterpolationConfig::ThreeDKt => (3, vec![&self.kt_range]),
            InterpolationConfig::FourDNucleonsAlphas => {
                (4, vec![&self.nucleons_range, &self.alphas_range])
            }
            InterpolationConfig::FourDNucleonsKt => (4, vec![&self.nucleons_range, &self.kt_range]),
            InterpolationConfig::FourDAlphasKt => (4, vec![&self.alphas_range, &self.kt_range]),
            InterpolationConfig::FourDXiDelta => (4, vec![&self.xi_range, &self.delta_range]),
            InterpolationConfig::FiveD => {
                (5, vec![&self.kt_range, &self.xi_range, &self.delta_range])
            }
            InterpolationConfig::SixD => (
                6,
                vec![
                    &self.nucleons_range,
                    &self.kt_range,
                    &self.xi_range,
                    &self.delta_range,
                ],
            ),
            InterpolationConfig::SevenD => (
                7,
                vec![
                    &self.nucleons_range,
                    &self.alphas_range,
                    &self.xi_range,
                    &self.delta_range,
                    &self.kt_range,
                ],
            ),
        };

        points.len() == expected_len
            && self.x_range.contains(points[expected_len - 2])
            && self.q2_range.contains(points[expected_len - 1])
            && ranges
                .iter()
                .zip(points)
                .all(|(range, &point)| range.contains(point))
    }

    /// Calculates the squared distance from a point to the subgrid's bounding box.
    pub fn distance_to_point(&self, points: &[f64]) -> f64 {
        self.parameter_ranges()
            .iter()
            .zip(points)
            .map(|(range, &point)| match point {
                p if p < range.min => (range.min - p) * (range.min - p),
                p if p > range.max => (p - range.max) * (p - range.max),
                _ => 0.0,
            })
            .sum()
    }

    /// Gathers the parameter ranges for the subgrid based on its configuration.
    fn parameter_ranges(&self) -> Vec<ParamRange> {
        let mut ranges = Vec::new();

        // Add ranges based on which dimensions are active
        if self.nucleons.len() > 1 {
            ranges.push(self.nucleons_range);
        }
        if self.alphas.len() > 1 {
            ranges.push(self.alphas_range);
        }
        if self.xis.len() > 1 {
            ranges.push(self.xi_range);
        }
        if self.deltas.len() > 1 {
            ranges.push(self.delta_range);
        }
        if self.kts.len() > 1 {
            ranges.push(self.kt_range);
        }

        // Always include x and q2
        ranges.extend([self.x_range, self.q2_range]);
        ranges
    }

    /// Gets the interpolation configuration for this subgrid.
    pub fn interpolation_config(&self) -> InterpolationConfig {
        InterpolationConfig::from_dimensions(
            self.nucleons.len(),
            self.alphas.len(),
            self.xis.len(),
            self.deltas.len(),
            self.kts.len(),
        )
    }

    /// Gets the parameter ranges for this subgrid.
    pub fn ranges(&self) -> RangeParameters {
        RangeParameters::new(
            self.nucleons_range,
            self.alphas_range,
            self.xi_range,
            self.delta_range,
            self.kt_range,
            self.x_range,
            self.q2_range,
        )
    }

    /// Gets a 2D slice of the grid for interpolation.
    ///
    /// This method is only valid for 2D interpolation configurations.
    ///
    /// # Arguments
    ///
    /// * `pid_index` - The index of the particle ID (flavor).
    ///
    /// # Panics
    ///
    /// Panics if called on a subgrid that is not 2D.
    pub fn grid_slice(&self, pid_index: usize) -> ArrayView2<'_, f64> {
        match self.interpolation_config() {
            InterpolationConfig::TwoD => match &self.grid {
                GridData::Grid6D(grid) => grid.slice(s![0, 0, pid_index, 0, .., ..]),
                GridData::Grid8D(grid) => grid.slice(s![0, 0, 0, 0, 0, pid_index, .., ..]),
            },
            _ => panic!("grid_slice only valid for 2D interpolation"),
        }
    }

    /// Returns a reference to the underlying grid (6D).
    ///
    /// # Panics
    ///
    /// Panics if the grid is 8D.
    pub fn grid_6d(&self) -> &Array6<f64> {
        match &self.grid {
            GridData::Grid6D(grid) => grid,
            GridData::Grid8D(_) => panic!("Cannot access 8D grid as 6D"),
        }
    }

    /// Returns a reference to the underlying grid (8D).
    ///
    /// # Panics
    ///
    /// Panics if the grid is 6D.
    pub fn grid_8d(&self) -> &ArrayD<f64> {
        match &self.grid {
            GridData::Grid6D(_) => panic!("Cannot access 6D grid as 8D"),
            GridData::Grid8D(grid) => grid,
        }
    }

    /// Returns true if this is an 8D grid.
    pub fn is_8d(&self) -> bool {
        matches!(self.grid, GridData::Grid8D(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_range() {
        let range = ParamRange::new(1.0, 10.0);
        assert!(range.contains(5.0));
        assert!(!range.contains(15.0));
    }
}
