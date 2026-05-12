//! This module defines the [`SubGrid`] struct and its implementation for PDF grid handling.
//!
//! # Contents
//!
//! - [`ParamRange`], [`RangeParameters`]: Parameter range types for grid axes.
//! - [`SubGrid`]: Represents a region of phase space with a consistent grid and provides
//!   methods for subgrid logic.

use ndarray::{s, Array1, Array6, ArrayView2};
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

/// Represents the parameter ranges for `x` and `q2`.
pub struct RangeParameters {
    /// The range for the nucleon numbers `A`.
    pub nucleons: ParamRange,
    /// The range for the AlphaS values `as`.
    pub alphas: ParamRange,
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
    /// * `kt` - The `ParamRange` for the transverse momentum `kT`.
    /// * `x` - The `ParamRange` for the momentum fraction `x`.
    /// * `q2` - The `ParamRange` for the energy scale `q2`.
    pub fn new(
        nucleons: ParamRange,
        alphas: ParamRange,
        kt: ParamRange,
        x: ParamRange,
        q2: ParamRange,
    ) -> Self {
        Self {
            nucleons,
            alphas,
            kt,
            x,
            q2,
        }
    }
}

/// Stores the PDF grid data for a single subgrid.
///
/// A subgrid represents a region of the phase space with a consistent
/// grid of `x` and `Q²` values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubGrid {
    /// Array of `x` values (momentum fraction).
    pub xs: Array1<f64>,
    /// Array of `Q²` values (energy scale squared).
    pub q2s: Array1<f64>,
    /// Array of `kT` values (transverse momentum).
    pub kts: Array1<f64>,
    /// 6-dimensional grid data: [nucleons, alphas, pids, kT, x, Q²].
    pub grid: Array6<f64>,
    /// Array of nucleon number values.
    pub nucleons: Array1<f64>,
    /// Array of alpha_s values.
    pub alphas: Array1<f64>,
    /// The valid range for the `nucleons` parameter in this subgrid.
    pub nucleons_range: ParamRange,
    /// The valid range for the `AlphaS` parameter in this subgrid.
    pub alphas_range: ParamRange,
    /// The valid range for the `kT` parameter in this subgrid.
    pub kt_range: ParamRange,
    /// The valid range for the `x` parameter in this subgrid.
    pub x_range: ParamRange,
    /// The valid range for the `q2` parameter in this subgrid.
    pub q2_range: ParamRange,
}

impl SubGrid {
    /// Creates a new `SubGrid` from raw data.
    ///
    /// # Arguments
    ///
    /// * `nucleon_numbers` - A vector of nucleon numbers.
    /// * `alphas_values` - A vector of alpha_s values.
    /// * `kt_subgrid` - A vector of `kT` values.
    /// * `xs` - A vector of `x` values.
    /// * `q2s` - A vector of `q2` values.
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
            grid: subgrid,
            nucleons: Array1::from_vec(nucleon_numbers),
            alphas: Array1::from_vec(alphas_values),
            nucleons_range: ncs_range,
            alphas_range: as_range,
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
            InterpolationConfig::ThreeDKt => (3, vec![&self.kt_range]),
            InterpolationConfig::FourDNucleonsAlphas => {
                (4, vec![&self.nucleons_range, &self.alphas_range])
            }
            InterpolationConfig::FourDNucleonsKt => (4, vec![&self.nucleons_range, &self.kt_range]),
            InterpolationConfig::FourDAlphasKt => (4, vec![&self.alphas_range, &self.kt_range]),
            InterpolationConfig::FiveD => (
                5,
                vec![&self.nucleons_range, &self.alphas_range, &self.kt_range],
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
        let mut ranges = match self.interpolation_config() {
            InterpolationConfig::TwoD => vec![],
            InterpolationConfig::ThreeDNucleons => vec![self.nucleons_range],
            InterpolationConfig::ThreeDAlphas => vec![self.alphas_range],
            InterpolationConfig::ThreeDKt => vec![self.kt_range],
            InterpolationConfig::FourDNucleonsAlphas => {
                vec![self.nucleons_range, self.alphas_range]
            }
            InterpolationConfig::FourDNucleonsKt => vec![self.nucleons_range, self.kt_range],
            InterpolationConfig::FourDAlphasKt => vec![self.alphas_range, self.kt_range],
            InterpolationConfig::FiveD => {
                vec![self.nucleons_range, self.alphas_range, self.kt_range]
            }
        };
        ranges.extend([self.x_range, self.q2_range]);
        ranges
    }

    /// Gets the interpolation configuration for this subgrid.
    pub fn interpolation_config(&self) -> InterpolationConfig {
        InterpolationConfig::from_dimensions(self.nucleons.len(), self.alphas.len(), self.kts.len())
    }

    /// Gets the parameter ranges for this subgrid.
    pub fn ranges(&self) -> RangeParameters {
        RangeParameters::new(
            self.nucleons_range,
            self.alphas_range,
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
            InterpolationConfig::TwoD => self.grid.slice(s![0, 0, pid_index, 0, .., ..]),
            _ => panic!("grid_slice only valid for 2D interpolation"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_subgrid_2d() -> SubGrid {
        SubGrid::new(
            vec![1.0],
            vec![0.118],
            vec![0.0],
            vec![0.1, 0.2],
            vec![1.0, 2.0],
            1,
            vec![1.0, 2.0, 3.0, 4.0],
        )
    }

    fn mock_subgrid_3d_nucleons() -> SubGrid {
        SubGrid::new(
            vec![1.0, 2.0],
            vec![0.118],
            vec![0.0],
            vec![0.1, 0.2],
            vec![1.0, 2.0],
            1,
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
        )
    }

    #[test]
    fn test_param_range() {
        let range = ParamRange::new(1.0, 10.0);
        assert!(range.contains(5.0));
        assert!(range.contains(1.0));
        assert!(range.contains(10.0));
        assert!(!range.contains(15.0));
        assert!(!range.contains(0.5));
    }

    #[test]
    fn test_range_parameters_new() {
        let rp = RangeParameters::new(
            ParamRange::new(1.0, 2.0),
            ParamRange::new(0.1, 0.2),
            ParamRange::new(0.0, 1.0),
            ParamRange::new(1e-4, 1.0),
            ParamRange::new(1.0, 1000.0),
        );
        assert_eq!(rp.nucleons.min, 1.0);
        assert_eq!(rp.q2.max, 1000.0);
    }

    #[test]
    fn test_subgrid_new() {
        let sg = mock_subgrid_2d();
        assert_eq!(sg.xs.len(), 2);
        assert_eq!(sg.q2s.len(), 2);
        assert_eq!(sg.grid.shape(), &[1, 1, 1, 1, 2, 2]);
        assert_eq!(sg.x_range.min, 0.1);
        assert_eq!(sg.x_range.max, 0.2);
        assert_eq!(sg.q2_range.min, 1.0);
        assert_eq!(sg.q2_range.max, 2.0);
    }

    #[test]
    fn test_contains_point_2d() {
        let sg = mock_subgrid_2d();
        assert!(sg.contains_point(&[0.15, 1.5]));
        assert!(sg.contains_point(&[0.1, 1.0]));
        assert!(!sg.contains_point(&[0.5, 1.5]));
        assert!(!sg.contains_point(&[0.15, 5.0]));
        assert!(!sg.contains_point(&[0.15]));
    }

    #[test]
    fn test_contains_point_3d_nucleons() {
        let sg = mock_subgrid_3d_nucleons();
        assert!(sg.contains_point(&[1.5, 0.15, 1.5]));
        assert!(!sg.contains_point(&[5.0, 0.15, 1.5]));
    }

    #[test]
    fn test_distance_to_point() {
        let sg = mock_subgrid_2d();
        assert_eq!(sg.distance_to_point(&[0.15, 1.5]), 0.0);
        let d = sg.distance_to_point(&[0.3, 1.5]);
        assert!(d > 0.0);
    }

    #[test]
    fn test_ranges() {
        let sg = mock_subgrid_2d();
        let r = sg.ranges();
        assert_eq!(r.x.min, 0.1);
        assert_eq!(r.x.max, 0.2);
        assert_eq!(r.q2.min, 1.0);
        assert_eq!(r.q2.max, 2.0);
    }

    #[test]
    fn test_grid_slice_2d() {
        let sg = mock_subgrid_2d();
        let slice = sg.grid_slice(0);
        assert_eq!(slice.shape(), &[2, 2]);
    }

    #[test]
    #[should_panic]
    fn test_grid_slice_panics_for_3d() {
        let sg = mock_subgrid_3d_nucleons();
        sg.grid_slice(0);
    }

    #[test]
    fn test_interpolation_config_from_subgrid() {
        let sg_2d = mock_subgrid_2d();
        assert!(matches!(
            sg_2d.interpolation_config(),
            InterpolationConfig::TwoD
        ));
        let sg_3d = mock_subgrid_3d_nucleons();
        assert!(matches!(
            sg_3d.interpolation_config(),
            InterpolationConfig::ThreeDNucleons
        ));
    }
}
