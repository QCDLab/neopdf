//! This module provides implementations for calculating the strong coupling constant.
//!
//! It includes support for different calculation methods, such as analytic formulas and
//! interpolation from tabulated values, mirroring the functionality available in `LHAPDF`.

use ninterp::interpolator::Extrapolate;
use ninterp::prelude::*;
use std::collections::HashMap;
use thiserror::Error;

use super::metadata::MetaData;
use super::strategy::AlphaSCubicInterpolation;

/// Errors that can occur during the analytical computations of `alpha_s`.
#[derive(Debug, Error)]
pub enum Error {
    /// Error indicating that no `Lambda_QCD` value is defined for the given `nf`.
    #[error("No subgrid LambdaQCD for nf={nf}")]
    LambdaQCDValueNotFound {
        /// The number of active flavors.
        nf: u32,
    },
    /// Error indicating that zero active flavor is not accepted.
    #[error("Invalid zero nf value")]
    NfZeroValueError,
    /// Error indicating that the order to compute the Beta function is not supported.
    #[error("Invalid order value to compute Beta o={order}")]
    BetaOrderValueError {
        /// Order to compute the Beta function.
        order: u32,
    },
}

/// Enum representing the different methods for alpha_s calculation.
pub enum AlphaS {
    Analytic(AlphaSAnalytic),
    Interpol(AlphaSInterpol),
}

impl AlphaS {
    /// Creates a new `AlphaS` calculator from PDF metadata.
    pub fn from_metadata(meta: &MetaData) -> Result<Self, String> {
        // TODO: Use `meta.alphas_type` for the logics.
        if meta.alphas_vals.is_empty() {
            Ok(AlphaS::Analytic(AlphaSAnalytic::from_metadata(meta)?))
        } else {
            Ok(AlphaS::Interpol(AlphaSInterpol::from_metadata(meta)?))
        }
    }

    /// Calculates the strong coupling `alpha_s` at a given `Q^2`.
    pub fn alphas_q2(&self, q2: f64) -> f64 {
        match self {
            AlphaS::Analytic(analytic) => analytic.alphas_q2(q2),
            AlphaS::Interpol(interpol) => interpol.alphas_q2(q2),
        }
    }
}

/// Strong coupling calculator using the analytic formulas.
pub struct AlphaSAnalytic {
    qcd_order: u32,
    fl_scheme: String,
    lambda_maps: HashMap<u32, f64>,
    mc_sq: f64,
    mb_sq: f64,
    mt_sq: f64,
    num_fl: u32,
}

impl AlphaSAnalytic {
    pub fn from_metadata(meta: &MetaData) -> Result<Self, String> {
        let mut lambda_maps = HashMap::new();
        // TODO: decide what to do about these hardcoded values.
        lambda_maps.insert(3, 0.339);
        lambda_maps.insert(4, 0.296);
        lambda_maps.insert(5, 0.213);

        let alphas_order_qcd = if meta.alphas_order_qcd == 0 {
            meta.order_qcd
        } else {
            meta.alphas_order_qcd
        };

        Ok(Self {
            qcd_order: alphas_order_qcd,
            lambda_maps,
            mc_sq: meta.m_charm * meta.m_charm,
            mb_sq: meta.m_bottom * meta.m_bottom,
            mt_sq: meta.m_top * meta.m_top,
            num_fl: meta.number_flavors,
            fl_scheme: meta.flavor_scheme.clone(),
        })
    }

    fn number_flavors_q2(&self, q2: f64) -> u32 {
        match () {
            _ if self.fl_scheme.to_uppercase() == "FIXED" => self.num_fl,
            _ if q2 > self.mt_sq && self.mt_sq > 0.0 => 6,
            _ if q2 > self.mb_sq && self.mb_sq > 0.0 => 5,
            _ if q2 > self.mc_sq && self.mc_sq > 0.0 => 4,
            _ => 3,
        }
    }

    fn lambda_qcd(&self, nf: u32) -> Result<f64, Error> {
        // NOTE: This is better be checked using `alphas_type`.
        match self.fl_scheme.to_uppercase().as_str() {
            "FIXED" => match self.lambda_maps.get(&self.num_fl) {
                Some(lambda_value) => Ok(*lambda_value),
                None => Err(Error::LambdaQCDValueNotFound { nf: self.num_fl }),
            },
            _ => {
                if nf == 0 {
                    return Err(Error::NfZeroValueError);
                }
                match self.lambda_maps.get(&nf) {
                    Some(lambda_value) => Ok(*lambda_value),
                    None => self.lambda_qcd(nf - 1),
                }
            }
        }
    }

    fn betas(&self, bto: u32, nf: u32) -> Result<f64, Error> {
        // Copied from https://gitlab.com/hepcedar/lhapdf/-/blob/main/src/AlphaS.cc
        let nf = nf as f64;
        let (nf2, nf3, nf4) = (nf * nf, nf * nf * nf, nf * nf * nf * nf);
        match bto {
            0 => Ok(0.875352187 - 0.053051647 * nf),
            1 => Ok(0.6459225457 - 0.0802126037 * nf),
            2 => Ok(0.719864327 - 0.140904490 * nf + 0.00303291339 * nf2),
            3 => Ok(1.172686 - 0.2785458 * nf + 0.01624467 * nf2 + 0.0000601247 * nf3),
            4 => Ok(1.714138 - 0.5940794 * nf + 0.05607482 * nf2
                - 0.0007380571 * nf3
                - 0.00000587968 * nf4),
            _ => Err(Error::BetaOrderValueError { order: bto }),
        }
    }

    /// Calculates alpha_s(Q2) using the analytic running formula.
    pub fn alphas_q2(&self, q2: f64) -> f64 {
        // Copied from https://gitlab.com/hepcedar/lhapdf/-/blob/main/src/AlphaS_Analytic.cc
        let nf = self.number_flavors_q2(q2);
        let lambda_qcd = self.lambda_qcd(nf).unwrap();

        if q2 <= lambda_qcd * lambda_qcd {
            return f64::INFINITY;
        }

        let lnx = (q2 / (lambda_qcd * lambda_qcd)).ln();
        let (lnlnx, lnlnx2, lnlnx3) = {
            let lnlnx = lnx.ln();
            (lnlnx, lnlnx * lnlnx, lnlnx * lnlnx * lnlnx)
        };
        let y = 1.0 / lnx;

        let beta0 = self.betas(0, nf).unwrap();
        let beta1 = self.betas(1, nf).unwrap();
        let (beta02, beta12) = (beta0 * beta0, beta1 * beta1);
        let prefac = 1.0 / beta0;
        let mut tmp = 1.0;

        if self.qcd_order == 0 {
            return 0.118; // _alpha_mz reference value
        }

        if self.qcd_order > 1 {
            let a_1 = beta1 * lnlnx / beta02;
            tmp -= a_1 * y;
        }

        if self.qcd_order > 2 {
            let beta2 = self.betas(2, nf).unwrap();

            let prefac_b = beta12 / (beta02 * beta02);
            let a_20 = lnlnx2 - lnlnx;
            let a_21 = beta2 * beta0 / beta12;
            let a_22 = 1.0;
            tmp += prefac_b * y * y * (a_20 + a_21 - a_22);
        }

        if self.qcd_order > 3 {
            let beta2 = self.betas(2, nf).unwrap();
            let beta3 = self.betas(3, nf).unwrap();

            let prefac_c = 1. / (beta02 * beta02 * beta02);
            let a_30 = (beta12 * beta1) * (lnlnx3 - (5.0 / 2.0) * lnlnx2 - 2.0 * lnlnx + 0.5);
            let a_31 = 3.0 * beta0 * beta1 * beta2 * lnlnx;
            let a_32 = 0.5 * beta02 * beta3;
            tmp -= prefac_c * y * y * y * (a_30 + a_31 - a_32);
        }

        prefac * y * tmp
    }
}

/// Strong coupling calculator using interpolation.
pub struct AlphaSInterpol {
    interpolator: Interp1DOwned<f64, AlphaSCubicInterpolation>,
}

impl AlphaSInterpol {
    pub fn from_metadata(meta: &MetaData) -> Result<Self, String> {
        let (q_values, alphas_vals): (Vec<_>, Vec<_>) = meta
            .alphas_q_values
            .iter()
            .zip(&meta.alphas_vals)
            .enumerate()
            .filter(|(i, (&q, _))| *i == 0 || q != meta.alphas_q_values[i - 1])
            .map(|(_, (&q, &alpha))| (q, alpha))
            .unzip();

        let q2_values: Vec<f64> = q_values.iter().map(|&q| (q * q).ln()).collect();

        let interpolator = Interp1D::new(
            q2_values.into(),
            alphas_vals.into(),
            AlphaSCubicInterpolation,
            Extrapolate::Error,
        )
        .map_err(|e| e.to_string())?;

        Ok(Self { interpolator })
    }

    pub fn alphas_q2(&self, q2: f64) -> f64 {
        self.interpolator.interpolate(&[q2.ln()]).unwrap_or(0.0)
    }
}
