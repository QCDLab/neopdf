//! This module defines metadata structures and types for describing PDF sets.
//!
//! It includes the `MetaData` struct (deserialized from .info files), PDF set
//! and interpolator type enums, and related utilities for handling PDF set information.
use serde::{Deserialize, Serialize};

/// Represents the type of PDF set.
#[repr(C)]
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SetType {
    #[default]
    SpaceLike,
    TimeLike,
}

/// Represents the type of interpolator used for the PDF.
/// WARNING: When adding elements, always append to the end!!!
#[repr(C)]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum InterpolatorType {
    Bilinear,
    LogBilinear,
    #[default]
    LogBicubic,
    LogTricubic,
    InterpNDLinear,
    LogChebyshev,
    LogFourCubic,
}

/// Represents the information block of a given PDF set.
///
/// This struct is influenced by LHAPDF `.info` files and extends the format
/// with support for 7-dimensional grids: (A, alphas, xi, delta, kt, x, Q2).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MetaDataV2 {
    /// Description of the PDF set.
    #[serde(rename = "SetDesc")]
    pub set_desc: String,
    /// Index of the PDF set.
    #[serde(rename = "SetIndex")]
    pub set_index: u32,
    /// Number of members in the PDF set (e.g., for error analysis).
    #[serde(rename = "NumMembers")]
    pub num_members: u32,
    /// Minimum x-value for which the PDF is valid.
    #[serde(rename = "XMin")]
    pub x_min: f64,
    /// Maximum x-value for which the PDF is valid.
    #[serde(rename = "XMax")]
    pub x_max: f64,
    /// Minimum Q-value (energy scale) for which the PDF is valid.
    #[serde(rename = "QMin")]
    pub q_min: f64,
    /// Maximum Q-value (energy scale) for which the PDF is valid.
    #[serde(rename = "QMax")]
    pub q_max: f64,
    /// List of particle data group (PDG) IDs for the flavors included in the PDF.
    #[serde(rename = "Flavors")]
    pub flavors: Vec<i32>,
    /// Format of the PDF data.
    #[serde(rename = "Format")]
    pub format: String,
    /// AlphaS Q values (non-squared) for interpolation.
    #[serde(rename = "AlphaS_Qs", default)]
    pub alphas_q_values: Vec<f64>,
    /// AlphaS values for interpolation.
    #[serde(rename = "AlphaS_Vals", default)]
    pub alphas_vals: Vec<f64>,
    /// Polarisation of the hadrons.
    #[serde(rename = "Polarized", default)]
    pub polarised: bool,
    /// Type of the hadrons.
    #[serde(rename = "SetType", default)]
    pub set_type: SetType,
    /// Type of interpolator used for the PDF (e.g., "LogBicubic").
    #[serde(rename = "InterpolatorType", default)]
    pub interpolator_type: InterpolatorType,
    /// The error type representation of the PDF.
    #[serde(rename = "ErrorType", default)]
    pub error_type: String,
    /// The hadron PID value representation of the PDF.
    #[serde(rename = "Particle", default)]
    pub hadron_pid: i32,
    /// The git version of the code that generated the PDF.
    #[serde(rename = "GitVersion", default)]
    pub git_version: String,
    /// The code version (CARGO_PKG_VERSION) that generated the PDF.
    #[serde(rename = "CodeVersion", default)]
    pub code_version: String,
    /// Scheme for the treatment of heavy flavors
    #[serde(rename = "FlavorScheme", default)]
    pub flavor_scheme: String,
    /// Number of QCD loops in the calculation of PDF evolution.
    #[serde(rename = "OrderQCD", default)]
    pub order_qcd: u32,
    /// Number of QCD loops in the calculation of `alpha_s`.
    #[serde(rename = "AlphaS_OrderQCD", default)]
    pub alphas_order_qcd: u32,
    /// Value of the W boson mass.
    #[serde(rename = "MW", default)]
    pub m_w: f64,
    /// Value of the Z boson mass.
    #[serde(rename = "MZ", default)]
    pub m_z: f64,
    /// Value of the Up quark mass.
    #[serde(rename = "MUp", default)]
    pub m_up: f64,
    /// Value of the Down quark mass.
    #[serde(rename = "MDown", default)]
    pub m_down: f64,
    /// Value of the Strange quark mass.
    #[serde(rename = "MStrange", default)]
    pub m_strange: f64,
    /// Value of the Charm quark mass.
    #[serde(rename = "MCharm", default)]
    pub m_charm: f64,
    /// Value of the Bottom quark mass.
    #[serde(rename = "MBottom", default)]
    pub m_bottom: f64,
    /// Value of the Top quark mass.
    #[serde(rename = "MTop", default)]
    pub m_top: f64,
    /// Type of strong coupling computations.
    #[serde(rename = "AlphaS_Type", default)]
    pub alphas_type: String,
    /// Number of active PDF flavors.
    #[serde(rename = "NumFlavors", default)]
    pub number_flavors: u32,
    /// Minimum xi-value for which the PDF is valid.
    #[serde(rename = "XiMin", default)]
    pub xi_min: f64,
    /// Maximum xi-value for which the PDF is valid.
    #[serde(rename = "XiMax", default)]
    pub xi_max: f64,
    /// Minimum delta-value for which the PDF is valid.
    #[serde(rename = "DeltaMin", default)]
    pub delta_min: f64,
    /// Maximum delta-value for which the PDF is valid.
    #[serde(rename = "DeltaMax", default)]
    pub delta_max: f64,
}

impl MetaDataV2 {
    /// Helper to get quark masses as a tuple
    pub fn quark_masses(&self) -> (f64, f64, f64, f64, f64, f64) {
        (
            self.m_up,
            self.m_down,
            self.m_strange,
            self.m_charm,
            self.m_bottom,
            self.m_top,
        )
    }
}

impl std::fmt::Display for MetaDataV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SetDesc: {}", self.set_desc)?;
        writeln!(f, "SetIndex: {}", self.set_index)?;
        writeln!(f, "NumMembers: {}", self.num_members)?;
        writeln!(f, "XMin: {}", self.x_min)?;
        writeln!(f, "XMax: {}", self.x_max)?;
        writeln!(f, "QMin: {}", self.q_min)?;
        writeln!(f, "QMax: {}", self.q_max)?;
        writeln!(f, "Flavors: {:?}", self.flavors)?;
        writeln!(f, "Format: {}", self.format)?;
        writeln!(f, "AlphaS_Qs: {:?}", self.alphas_q_values)?;
        writeln!(f, "AlphaS_Vals: {:?}", self.alphas_vals)?;
        writeln!(f, "Polarized: {}", self.polarised)?;
        writeln!(f, "SetType: {:?}", self.set_type)?;
        writeln!(f, "InterpolatorType: {:?}", self.interpolator_type)?;
        writeln!(f, "ErrorType: {}", self.error_type)?;
        writeln!(f, "Particle: {}", self.hadron_pid)?;
        writeln!(f, "GitVersion: {}", self.git_version)?;
        writeln!(f, "CodeVersion: {}", self.code_version)?;
        writeln!(f, "FlavorScheme: {}", self.flavor_scheme)?;
        writeln!(f, "OrderQCD: {}", self.order_qcd)?;
        writeln!(f, "AlphaS_OrderQCD: {}", self.alphas_order_qcd)?;
        writeln!(f, "MW: {}", self.m_w)?;
        writeln!(f, "MZ: {}", self.m_z)?;
        writeln!(f, "MUp: {}", self.m_up)?;
        writeln!(f, "MDown: {}", self.m_down)?;
        writeln!(f, "MStrange: {}", self.m_strange)?;
        writeln!(f, "MCharm: {}", self.m_charm)?;
        writeln!(f, "MBottom: {}", self.m_bottom)?;
        writeln!(f, "MTop: {}", self.m_top)?;
        writeln!(f, "AlphaS_Type: {}", self.alphas_type)?;
        writeln!(f, "NumFlavors: {}", self.number_flavors)?;
        writeln!(f, "XiMin: {}", self.xi_min)?;
        writeln!(f, "XiMax: {}", self.xi_max)?;
        writeln!(f, "DeltaMin: {}", self.delta_min)?;
        write!(f, "DeltaMax: {}", self.delta_max)
    }
}

/// Main metadata type for v0.2.1+
/// This is now a simple type alias to MetaDataV2 for the new format.
/// For backward compatibility with v0.2.0, use the conversion functions.
pub type MetaData = MetaDataV2;

/// Converts from legacy v0.2.0 MetaData to new v0.2.1 format
impl From<neopdf_legacy::metadata::MetaData> for MetaData {
    fn from(legacy: neopdf_legacy::metadata::MetaData) -> Self {
        // Convert from v0.2.0 format to v0.2.1 MetaDataV2
        // Add default values for new xi and delta fields
        Self {
            set_desc: legacy.set_desc.clone(),
            set_index: legacy.set_index,
            num_members: legacy.num_members,
            x_min: legacy.x_min,
            x_max: legacy.x_max,
            q_min: legacy.q_min,
            q_max: legacy.q_max,
            flavors: legacy.flavors.clone(),
            format: legacy.format.clone(),
            alphas_q_values: legacy.alphas_q_values.clone(),
            alphas_vals: legacy.alphas_vals.clone(),
            polarised: legacy.polarised,
            set_type: match legacy.set_type {
                neopdf_legacy::metadata::SetType::SpaceLike => SetType::SpaceLike,
                neopdf_legacy::metadata::SetType::TimeLike => SetType::TimeLike,
            },
            interpolator_type: match legacy.interpolator_type {
                neopdf_legacy::metadata::InterpolatorType::Bilinear => InterpolatorType::Bilinear,
                neopdf_legacy::metadata::InterpolatorType::LogBilinear => {
                    InterpolatorType::LogBilinear
                }
                neopdf_legacy::metadata::InterpolatorType::LogBicubic => {
                    InterpolatorType::LogBicubic
                }
                neopdf_legacy::metadata::InterpolatorType::LogTricubic => {
                    InterpolatorType::LogTricubic
                }
                neopdf_legacy::metadata::InterpolatorType::InterpNDLinear => {
                    InterpolatorType::InterpNDLinear
                }
                neopdf_legacy::metadata::InterpolatorType::LogChebyshev => {
                    InterpolatorType::LogChebyshev
                }
            },
            error_type: legacy.error_type.clone(),
            hadron_pid: legacy.hadron_pid,
            git_version: legacy.git_version.clone(),
            code_version: legacy.code_version.clone(),
            flavor_scheme: legacy.flavor_scheme.clone(),
            order_qcd: legacy.order_qcd,
            alphas_order_qcd: legacy.alphas_order_qcd,
            m_w: legacy.m_w,
            m_z: legacy.m_z,
            m_up: legacy.m_up,
            m_down: legacy.m_down,
            m_strange: legacy.m_strange,
            m_charm: legacy.m_charm,
            m_bottom: legacy.m_bottom,
            m_top: legacy.m_top,
            alphas_type: legacy.alphas_type.clone(),
            number_flavors: legacy.number_flavors,
            // New V2 fields with defaults
            xi_min: 1.0,
            xi_max: 1.0,
            delta_min: 0.0,
            delta_max: 0.0,
        }
    }
}
