//! This module contains the dynamic interpolation traits, InterpolatorFactory, and dynamic
//! dispatch logic for PDF grids.
//!
//! # Contents
//!
//! - [`DynInterpolator`]: Trait for dynamic, multi-dimensional interpolation.
//! - [`InterpolatorFactory`]: Factory for constructing interpolators for SubGrid.
//!
//! # Note
//!
//! Interpolation strategies are defined in `strategy.rs`.
//! The [`SubGrid`] struct is defined in `subgrid.rs`.

use ndarray::{s, OwnedRepr};
use ninterp::data::{InterpData2D, InterpData3D};
use ninterp::error::InterpolateError;
use ninterp::interpolator::{
    Extrapolate, Interp2D, Interp2DOwned, Interp3D, Interp3DOwned, InterpND, InterpNDOwned,
};
use ninterp::prelude::*;
use ninterp::strategy::traits::{Strategy2D, Strategy3D, StrategyND};
use ninterp::strategy::Linear;

use super::metadata::InterpolatorType;
use super::strategy::{
    BilinearInterpolation, LogBicubicInterpolation, LogBilinearInterpolation,
    LogChebyshevBatchInterpolation, LogChebyshevInterpolation, LogTricubicInterpolation,
};
use super::subgrid::SubGrid;

/// Represents the dimensionality and structure of interpolation needed.
///
/// This enum is used to select the appropriate interpolation strategy based on the
/// dimensions of the PDF grid data.
#[derive(Debug, Clone, Copy)]
pub enum InterpolationConfig {
    /// 2D interpolation, typically in `x` (momentum fraction) and `Q²` (energy scale).
    TwoD,
    /// 3D interpolation, including a dimension for varying nucleon numbers `A`,
    /// in addition to `x` and `Q²`.
    ThreeDNucleons,
    /// 3D interpolation, including a dimension for varying `alpha_s` values,
    /// in addition to `x` and `Q²`.
    ThreeDAlphas,
    /// 3D interpolation, including a dimension for varying `kT` values,
    /// in addition to `x` and `Q²`.
    ThreeDKt,
    /// 4D interpolation, covering nucleon numbers `A`, `alpha_s`, `x`, and `Q²`.
    FourDNucleonsAlphas,
    /// 4D interpolation, covering nucleon numbers `A`, kT, `x`, and `Q²`.
    FourDNucleonsKt,
    /// 4D interpolation, covering `alpha_s`, kT, `x`, and `Q²`.
    FourDAlphasKt,
    /// 5D interpolation, covering nucleon numbers `A`, `alpha_s`, `kT`, `x`, and `Q²`.
    FiveD,
}

impl InterpolationConfig {
    /// Determines the interpolation configuration from the number of nucleons and alpha_s values.
    ///
    /// # Panics
    ///
    /// Panics if the combination of `n_nucleons` and `n_alphas` is not supported.
    pub fn from_dimensions(n_nucleons: usize, n_alphas: usize, n_kts: usize) -> Self {
        match (n_nucleons > 1, n_alphas > 1, n_kts > 1) {
            (false, false, false) => Self::TwoD,
            (true, false, false) => Self::ThreeDNucleons,
            (false, true, false) => Self::ThreeDAlphas,
            (false, false, true) => Self::ThreeDKt,
            (true, true, false) => Self::FourDNucleonsAlphas,
            (true, false, true) => Self::FourDNucleonsKt,
            (false, true, true) => Self::FourDAlphasKt,
            (true, true, true) => Self::FiveD,
        }
    }
}

/// A trait for dynamic interpolation across different dimensions.
pub trait DynInterpolator: Send + Sync {
    fn interpolate_point(&self, point: &[f64]) -> Result<f64, InterpolateError>;
}

// Implement `DynInterpolator` for 2D interpolators.
impl<S> DynInterpolator for Interp2DOwned<f64, S>
where
    S: Strategy2D<OwnedRepr<f64>> + 'static + Clone + Send + Sync,
{
    fn interpolate_point(&self, point: &[f64]) -> Result<f64, InterpolateError> {
        let [x, y] = point
            .try_into()
            .map_err(|_| InterpolateError::Other("Expected 2D point".to_string()))?;
        self.interpolate(&[x, y])
    }
}

// Implement `DynInterpolator` for 3D interpolators.
impl<S> DynInterpolator for Interp3DOwned<f64, S>
where
    S: Strategy3D<OwnedRepr<f64>> + 'static + Clone + Send + Sync,
{
    fn interpolate_point(&self, point: &[f64]) -> Result<f64, InterpolateError> {
        let [x, y, z] = point
            .try_into()
            .map_err(|_| InterpolateError::Other("Expected 3D point".to_string()))?;
        self.interpolate(&[x, y, z])
    }
}

// Implement `DynInterpolator` for N-dimensional interpolators.
impl<S> DynInterpolator for InterpNDOwned<f64, S>
where
    S: StrategyND<OwnedRepr<f64>> + 'static + Clone + Send + Sync,
{
    fn interpolate_point(&self, point: &[f64]) -> Result<f64, InterpolateError> {
        self.interpolate(point)
    }
}

/// An enum to dispatch batch interpolation to the correct Chebyshev interpolator.
pub enum BatchInterpolator {
    Chebyshev2D(
        LogChebyshevBatchInterpolation<2>,
        InterpData2D<OwnedRepr<f64>>,
    ),
    Chebyshev3D(
        LogChebyshevBatchInterpolation<3>,
        InterpData3D<OwnedRepr<f64>>,
    ),
}

impl BatchInterpolator {
    /// Interpolates a batch of points.
    pub fn interpolate(&self, points: Vec<Vec<f64>>) -> Result<Vec<f64>, InterpolateError> {
        match self {
            BatchInterpolator::Chebyshev2D(strategy, data) => {
                let points_2d: Vec<[f64; 2]> = points
                    .into_iter()
                    .map(|p| p.try_into().expect("Invalid point dimension for 2D"))
                    .collect();
                strategy.interpolate(data, &points_2d)
            }
            BatchInterpolator::Chebyshev3D(strategy, data) => {
                let points_3d: Vec<[f64; 3]> = points
                    .into_iter()
                    .map(|p| p.try_into().expect("Invalid point dimension for 3D"))
                    .collect();
                strategy.interpolate(data, &points_3d)
            }
        }
    }
}

/// Factory for creating dynamic interpolators based on interpolation type and grid dimensions.
pub struct InterpolatorFactory;

impl InterpolatorFactory {
    pub fn create(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        match subgrid.interpolation_config() {
            InterpolationConfig::TwoD => Self::interpolator_xfxq2(interp_type, subgrid, pid_index),
            InterpolationConfig::ThreeDNucleons => {
                Self::interpolator_xfxq2_nucleons(interp_type, subgrid, pid_index)
            }
            InterpolationConfig::ThreeDAlphas => {
                Self::interpolator_xfxq2_alphas(interp_type, subgrid, pid_index)
            }
            InterpolationConfig::ThreeDKt => {
                Self::interpolator_xfxq2_kts(interp_type, subgrid, pid_index)
            }
            InterpolationConfig::FourDNucleonsAlphas => {
                Self::interpolator_xfxq2_nucleons_alphas(interp_type, subgrid, pid_index)
            }
            InterpolationConfig::FourDNucleonsKt => {
                Self::interpolator_xfxq2_nucleons_kts(interp_type, subgrid, pid_index)
            }
            InterpolationConfig::FourDAlphasKt => {
                Self::interpolator_xfxq2_alphas_kts(interp_type, subgrid, pid_index)
            }
            InterpolationConfig::FiveD => {
                Self::interpolator_xfxq2_5dim(interp_type, subgrid, pid_index)
            }
        }
    }

    fn interpolator_xfxq2(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_slice = subgrid.grid_slice(pid_index).to_owned();

        match interp_type {
            InterpolatorType::Bilinear => Box::new(
                Interp2D::new(
                    subgrid.xs.to_owned(),
                    subgrid.q2s.to_owned(),
                    grid_slice,
                    BilinearInterpolation,
                    Extrapolate::Clamp,
                )
                .expect("Failed to create 2D interpolator"),
            ),
            InterpolatorType::LogBilinear => Box::new(
                Interp2D::new(
                    subgrid.xs.mapv(f64::ln),
                    subgrid.q2s.mapv(f64::ln),
                    grid_slice,
                    LogBilinearInterpolation,
                    Extrapolate::Clamp,
                )
                .expect("Failed to create 2D interpolator"),
            ),
            InterpolatorType::LogBicubic => Box::new(
                Interp2D::new(
                    subgrid.xs.mapv(f64::ln),
                    subgrid.q2s.mapv(f64::ln),
                    grid_slice,
                    LogBicubicInterpolation::default(),
                    Extrapolate::Clamp,
                )
                .expect("Failed to create 2D interpolator"),
            ),
            InterpolatorType::LogChebyshev => Box::new(
                Interp2D::new(
                    subgrid.xs.mapv(f64::ln),
                    subgrid.q2s.mapv(f64::ln),
                    grid_slice,
                    LogChebyshevInterpolation::<2>::default(),
                    Extrapolate::Clamp,
                )
                .expect("Failed to create 2D interpolator"),
            ),
            _ => panic!("Unsupported 2D interpolator: {:?}", interp_type),
        }
    }

    fn interpolator_xfxq2_nucleons(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid
            .grid
            .slice(s![.., 0, pid_index, 0, .., ..])
            .to_owned();
        let reshaped_data = grid_data
            .into_shape_with_order((subgrid.nucleons.len(), subgrid.xs.len(), subgrid.q2s.len()))
            .expect("Failed to reshape 3D data");

        match interp_type {
            InterpolatorType::LogTricubic => Box::new(
                Interp3D::new(
                    subgrid.nucleons.mapv(f64::ln),
                    subgrid.xs.mapv(f64::ln),
                    subgrid.q2s.mapv(f64::ln),
                    reshaped_data,
                    LogTricubicInterpolation,
                    Extrapolate::Clamp,
                )
                .expect("Failed to create 3D interpolator"),
            ),
            InterpolatorType::LogChebyshev => Box::new(
                Interp3D::new(
                    subgrid.nucleons.mapv(f64::ln),
                    subgrid.xs.mapv(f64::ln),
                    subgrid.q2s.mapv(f64::ln),
                    reshaped_data,
                    LogChebyshevInterpolation::<3>::default(),
                    Extrapolate::Clamp,
                )
                .expect("Failed to create 3D interpolator"),
            ),
            _ => panic!("Unsupported 3D interpolator: {:?}", interp_type),
        }
    }

    fn interpolator_xfxq2_alphas(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid
            .grid
            .slice(s![0, .., pid_index, 0, .., ..])
            .to_owned();
        let reshaped_data = grid_data
            .into_shape_with_order((subgrid.alphas.len(), subgrid.xs.len(), subgrid.q2s.len()))
            .expect("Failed to reshape 3D data");

        match interp_type {
            InterpolatorType::LogTricubic => Box::new(
                Interp3D::new(
                    subgrid.alphas.mapv(f64::ln),
                    subgrid.xs.mapv(f64::ln),
                    subgrid.q2s.mapv(f64::ln),
                    reshaped_data,
                    LogTricubicInterpolation,
                    Extrapolate::Clamp,
                )
                .expect("Failed to create 3D interpolator"),
            ),
            InterpolatorType::LogChebyshev => Box::new(
                Interp3D::new(
                    subgrid.alphas.mapv(f64::ln),
                    subgrid.xs.mapv(f64::ln),
                    subgrid.q2s.mapv(f64::ln),
                    reshaped_data,
                    LogChebyshevInterpolation::<3>::default(),
                    Extrapolate::Clamp,
                )
                .expect("Failed to create 3D interpolator"),
            ),
            _ => panic!("Unsupported 3D interpolator: {:?}", interp_type),
        }
    }

    fn interpolator_xfxq2_kts(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid
            .grid
            .slice(s![0, 0, pid_index, .., .., ..])
            .to_owned();
        let reshaped_data = grid_data
            .into_shape_with_order((subgrid.kts.len(), subgrid.xs.len(), subgrid.q2s.len()))
            .expect("Failed to reshape 3D data");

        match interp_type {
            InterpolatorType::LogTricubic => Box::new(
                Interp3D::new(
                    subgrid.kts.mapv(f64::ln),
                    subgrid.xs.mapv(f64::ln),
                    subgrid.q2s.mapv(f64::ln),
                    reshaped_data,
                    LogTricubicInterpolation,
                    Extrapolate::Clamp,
                )
                .expect("Failed to create 3D interpolator"),
            ),
            InterpolatorType::LogChebyshev => Box::new(
                Interp3D::new(
                    subgrid.kts.mapv(f64::ln),
                    subgrid.xs.mapv(f64::ln),
                    subgrid.q2s.mapv(f64::ln),
                    reshaped_data,
                    LogChebyshevInterpolation::<3>::default(),
                    Extrapolate::Clamp,
                )
                .expect("Failed to create 3D interpolator"),
            ),
            _ => panic!("Unsupported 3D interpolator: {:?}", interp_type),
        }
    }

    fn interpolator_xfxq2_nucleons_alphas(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid
            .grid
            .slice(s![.., .., pid_index, 0, .., ..])
            .to_owned();
        let coords = vec![
            subgrid.nucleons.to_owned(),
            subgrid.alphas.to_owned(),
            subgrid.xs.to_owned(),
            subgrid.q2s.to_owned(),
        ];
        let reshaped_data = grid_data
            .into_shape_with_order((
                subgrid.nucleons.len(),
                subgrid.alphas.len(),
                subgrid.xs.len(),
                subgrid.q2s.len(),
            ))
            .expect("Failed to reshape 4D data");

        match interp_type {
            InterpolatorType::InterpNDLinear => Box::new(
                InterpND::new(coords, reshaped_data.into_dyn(), Linear, Extrapolate::Clamp)
                    .expect("Failed to create 4D interpolator"),
            ),
            _ => panic!("Unsupported 4D interpolator: {:?}", interp_type),
        }
    }

    fn interpolator_xfxq2_nucleons_kts(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid
            .grid
            .slice(s![.., 0, pid_index, .., .., ..])
            .to_owned();
        let coords = vec![
            subgrid.nucleons.mapv(f64::ln),
            subgrid.kts.mapv(f64::ln),
            subgrid.xs.mapv(f64::ln),
            subgrid.q2s.mapv(f64::ln),
        ];
        let reshaped_data = grid_data
            .into_shape_with_order((
                subgrid.nucleons.len(),
                subgrid.kts.len(),
                subgrid.xs.len(),
                subgrid.q2s.len(),
            ))
            .expect("Failed to reshape 4D data");

        match interp_type {
            InterpolatorType::InterpNDLinear => Box::new(
                InterpND::new(coords, reshaped_data.into_dyn(), Linear, Extrapolate::Clamp)
                    .expect("Failed to create 4D interpolator"),
            ),
            _ => panic!("Unsupported 4D interpolator: {:?}", interp_type),
        }
    }

    fn interpolator_xfxq2_alphas_kts(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid
            .grid
            .slice(s![0, .., pid_index, .., .., ..])
            .to_owned();
        let coords = vec![
            subgrid.alphas.mapv(f64::ln),
            subgrid.kts.mapv(f64::ln),
            subgrid.xs.mapv(f64::ln),
            subgrid.q2s.mapv(f64::ln),
        ];
        let reshaped_data = grid_data
            .into_shape_with_order((
                subgrid.alphas.len(),
                subgrid.kts.len(),
                subgrid.xs.len(),
                subgrid.q2s.len(),
            ))
            .expect("Failed to reshape 4D data");

        match interp_type {
            InterpolatorType::InterpNDLinear => Box::new(
                InterpND::new(coords, reshaped_data.into_dyn(), Linear, Extrapolate::Clamp)
                    .expect("Failed to create 4D interpolator"),
            ),
            _ => panic!("Unsupported 4D interpolator: {:?}", interp_type),
        }
    }

    fn interpolator_xfxq2_5dim(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid
            .grid
            .slice(s![.., .., pid_index, .., .., ..])
            .to_owned();
        let coords = vec![
            subgrid.nucleons.mapv(f64::ln),
            subgrid.alphas.mapv(f64::ln),
            subgrid.kts.mapv(f64::ln),
            subgrid.xs.mapv(f64::ln),
            subgrid.q2s.mapv(f64::ln),
        ];
        let reshaped_data = grid_data
            .into_shape_with_order((
                subgrid.nucleons.len(),
                subgrid.alphas.len(),
                subgrid.kts.len(),
                subgrid.xs.len(),
                subgrid.q2s.len(),
            ))
            .expect("Failed to reshape 5D data");

        match interp_type {
            InterpolatorType::InterpNDLinear => Box::new(
                InterpND::new(coords, reshaped_data.into_dyn(), Linear, Extrapolate::Clamp)
                    .expect("Failed to create 5D interpolator"),
            ),
            _ => panic!("Unsupported 5D interpolator: {:?}", interp_type),
        }
    }

    pub fn create_batch_interpolator(
        subgrid: &SubGrid,
        pid_idx: usize,
    ) -> Result<BatchInterpolator, String> {
        match subgrid.interpolation_config() {
            InterpolationConfig::TwoD => {
                let mut strategy = LogChebyshevBatchInterpolation::<2>::default();
                let grid_slice = subgrid.grid_slice(pid_idx).to_owned();

                let data = InterpData2D::new(
                    subgrid.xs.mapv(f64::ln),
                    subgrid.q2s.mapv(f64::ln),
                    grid_slice,
                )
                .map_err(|e| e.to_string())?;
                strategy.init(&data).map_err(|e| e.to_string())?;

                Ok(BatchInterpolator::Chebyshev2D(strategy, data))
            }
            InterpolationConfig::ThreeDNucleons => {
                let mut strategy = LogChebyshevBatchInterpolation::<3>::default();
                let grid_data = subgrid.grid.slice(s![.., 0, pid_idx, 0, .., ..]).to_owned();

                let reshaped_data = grid_data
                    .into_shape_with_order((
                        subgrid.nucleons.len(),
                        subgrid.xs.len(),
                        subgrid.q2s.len(),
                    ))
                    .expect("Failed to reshape 3D data");

                let data = InterpData3D::new(
                    subgrid.nucleons.mapv(f64::ln),
                    subgrid.xs.mapv(f64::ln),
                    subgrid.q2s.mapv(f64::ln),
                    reshaped_data,
                )
                .map_err(|e| e.to_string())?;
                strategy.init(&data).map_err(|e| e.to_string())?;

                Ok(BatchInterpolator::Chebyshev3D(strategy, data))
            }
            InterpolationConfig::ThreeDAlphas => {
                let mut strategy = LogChebyshevBatchInterpolation::<3>::default();
                let grid_data = subgrid.grid.slice(s![0, .., pid_idx, 0, .., ..]).to_owned();

                let reshaped_data = grid_data
                    .into_shape_with_order((
                        subgrid.alphas.len(),
                        subgrid.xs.len(),
                        subgrid.q2s.len(),
                    ))
                    .expect("Failed to reshape 3D data");

                let data = InterpData3D::new(
                    subgrid.alphas.mapv(f64::ln),
                    subgrid.xs.mapv(f64::ln),
                    subgrid.q2s.mapv(f64::ln),
                    reshaped_data,
                )
                .map_err(|e| e.to_string())?;
                strategy.init(&data).map_err(|e| e.to_string())?;

                Ok(BatchInterpolator::Chebyshev3D(strategy, data))
            }
            InterpolationConfig::ThreeDKt => {
                let mut strategy = LogChebyshevBatchInterpolation::<3>::default();
                let grid_data = subgrid.grid.slice(s![0, 0, pid_idx, .., .., ..]).to_owned();

                let reshaped_data = grid_data
                    .into_shape_with_order((subgrid.kts.len(), subgrid.xs.len(), subgrid.q2s.len()))
                    .expect("Failed to reshape 3D data");

                let data = InterpData3D::new(
                    subgrid.kts.mapv(f64::ln),
                    subgrid.xs.mapv(f64::ln),
                    subgrid.q2s.mapv(f64::ln),
                    reshaped_data,
                )
                .map_err(|e| e.to_string())?;
                strategy.init(&data).map_err(|e| e.to_string())?;

                Ok(BatchInterpolator::Chebyshev3D(strategy, data))
            }
            _ => Err("Unsupported dimension for batch interpolation".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subgrid::SubGrid;

    const MAXDIFF: f64 = 1e-15;

    fn mock_subgrid_2d() -> SubGrid {
        let xs = vec![0.1, 0.2];
        let q2s = vec![1.0, 2.0];
        let grid_data = vec![1.0, 2.0, 3.0, 4.0];
        SubGrid::new(vec![1.0], vec![0.118], vec![0.0], xs, q2s, 1, grid_data)
    }

    fn mock_subgrid_3d_nucleons() -> SubGrid {
        let nucleons = vec![1.0, 2.0, 3.0, 4.0];
        let xs = vec![0.1, 0.2, 0.3, 0.4];
        let q2s = vec![1.0, 2.0, 3.0, 4.0];
        let grid_data = (1..=64).map(|v| v as f64).collect();
        SubGrid::new(nucleons, vec![0.118], vec![0.0], xs, q2s, 1, grid_data)
    }

    fn mock_subgrid_3d_alphas() -> SubGrid {
        let alphas = vec![0.118, 0.120, 0.122, 0.124];
        let xs = vec![0.1, 0.2, 0.3, 0.4];
        let q2s = vec![1.0, 2.0, 3.0, 4.0];
        let grid_data = (1..=64).map(|v| v as f64).collect();
        SubGrid::new(vec![1.0], alphas, vec![0.0], xs, q2s, 1, grid_data)
    }

    fn mock_subgrid_3d_kts() -> SubGrid {
        let kts = vec![0.5, 1.0, 1.5, 2.0];
        let xs = vec![0.1, 0.2, 0.3, 0.4];
        let q2s = vec![1.0, 2.0, 3.0, 4.0];
        let grid_data = (1..=64).map(|v| v as f64).collect();
        SubGrid::new(vec![1.0], vec![0.118], kts, xs, q2s, 1, grid_data)
    }

    fn mock_subgrid_4d_nucleons_alphas() -> SubGrid {
        let nucleons = vec![1.0, 2.0];
        let alphas = vec![0.118, 0.120];
        let xs = vec![0.1, 0.2];
        let q2s = vec![1.0, 2.0];
        let grid_data = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ];
        SubGrid::new(nucleons, alphas, vec![0.0], xs, q2s, 1, grid_data)
    }

    #[test]
    fn test_interpolation_config() {
        assert!(matches!(
            InterpolationConfig::from_dimensions(1, 1, 1),
            InterpolationConfig::TwoD
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(2, 1, 1),
            InterpolationConfig::ThreeDNucleons
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(1, 2, 1),
            InterpolationConfig::ThreeDAlphas
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(1, 1, 2),
            InterpolationConfig::ThreeDKt
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(2, 2, 1),
            InterpolationConfig::FourDNucleonsAlphas
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(2, 1, 2),
            InterpolationConfig::FourDNucleonsKt
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(1, 2, 2),
            InterpolationConfig::FourDAlphasKt
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(2, 2, 2),
            InterpolationConfig::FiveD
        ));
    }

    #[test]
    fn test_2d_bilinear_interpolation() {
        let subgrid = mock_subgrid_2d();
        let interpolator = InterpolatorFactory::create(InterpolatorType::Bilinear, &subgrid, 0);
        let result = interpolator.interpolate_point(&[0.15, 1.5]).unwrap();
        assert!((result - 2.5).abs() < MAXDIFF);
    }

    #[test]
    fn test_3d_nucleons_interpolation() {
        let subgrid = mock_subgrid_3d_nucleons();
        let interpolator = InterpolatorFactory::create(InterpolatorType::LogTricubic, &subgrid, 0);
        let result = interpolator
            .interpolate_point(&[2.0f64.ln(), 0.2f64.ln(), 2.0f64.ln()])
            .unwrap();
        assert!((result - 22.0).abs() < MAXDIFF);
    }

    #[test]
    fn test_3d_alphas_interpolation() {
        let subgrid = mock_subgrid_3d_alphas();
        let interpolator = InterpolatorFactory::create(InterpolatorType::LogTricubic, &subgrid, 0);
        let result = interpolator
            .interpolate_point(&[0.120f64.ln(), 0.2f64.ln(), 2.0f64.ln()])
            .unwrap();
        assert!((result - 22.0).abs() < MAXDIFF);
    }

    #[test]
    fn test_3d_kts_interpolation() {
        let subgrid = mock_subgrid_3d_kts();
        let interpolator = InterpolatorFactory::create(InterpolatorType::LogTricubic, &subgrid, 0);
        let result = interpolator
            .interpolate_point(&[1.0f64.ln(), 0.2f64.ln(), 2.0f64.ln()])
            .unwrap();
        assert!((result - 22.0).abs() < MAXDIFF);
    }

    #[test]
    fn test_4d_nucleons_alphas_interpolation() {
        let subgrid = mock_subgrid_4d_nucleons_alphas();
        let interpolator =
            InterpolatorFactory::create(InterpolatorType::InterpNDLinear, &subgrid, 0);
        let result = interpolator
            .interpolate_point(&[1.5, 0.119, 0.15, 1.5])
            .unwrap();
        assert!((result - 8.5).abs() < MAXDIFF);
    }

    #[test]
    #[should_panic]
    fn test_unsupported_interpolator() {
        let subgrid = mock_subgrid_2d();
        InterpolatorFactory::create(InterpolatorType::LogTricubic, &subgrid, 0);
    }
}
