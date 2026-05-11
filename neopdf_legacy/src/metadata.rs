//! This module defines metadata structures and types for describing PDF sets.
//!
//! It includes the `MetaData` struct (deserialized from .info files), PDF set
//! and interpolator type enums, and related utilities for handling PDF set information.
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::ops::{Deref, DerefMut};

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
}

/// Represents the information block of a given set.
///
/// In order to support LHAPDF formats, the fields here are very much influenced by the
/// LHAPDF `.info` file. This struct is generally deserialized from a YAML-like format.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MetaDataV1 {
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
}

/// Version-aware metadata wrapper that handles serialization compatibility.
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum MetaData {
    V1(MetaDataV1),
}

impl MetaData {
    /// Creates a new instance of V1 `MetaData`.
    pub fn new_v1(data: MetaDataV1) -> Self {
        Self::V1(data)
    }

    /// Gets the current version as the latest available version.
    pub fn current_v1(data: MetaDataV1) -> Self {
        Self::V1(data)
    }

    /// Gets the underlying data as the latest version.
    pub fn as_latest(&self) -> MetaDataV1 {
        match self {
            MetaData::V1(data) => data.clone(),
        }
    }
}

impl Deref for MetaData {
    type Target = MetaDataV1;

    fn deref(&self) -> &Self::Target {
        match self {
            MetaData::V1(data) => data,
        }
    }
}

impl DerefMut for MetaData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            MetaData::V1(data) => data,
        }
    }
}

impl<'de> Deserialize<'de> for MetaData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v1 = MetaDataV1::deserialize(deserializer)?;

        Ok(MetaData::V1(v1))
    }
}

impl fmt::Display for MetaData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Set Description: {}", self.set_desc)?;
        writeln!(f, "Set Index: {}", self.set_index)?;
        writeln!(f, "Number of Members: {}", self.num_members)?;
        writeln!(f, "XMin: {}", self.x_min)?;
        writeln!(f, "XMax: {}", self.x_max)?;
        writeln!(f, "QMin: {}", self.q_min)?;
        writeln!(f, "QMax: {}", self.q_max)?;
        writeln!(f, "Flavors: {:?}", self.flavors)?;
        writeln!(f, "Format: {}", self.format)?;
        writeln!(f, "AlphaS Q Values: {:?}", self.alphas_q_values)?;
        writeln!(f, "AlphaS Values: {:?}", self.alphas_vals)?;
        writeln!(f, "Polarized: {}", self.polarised)?;
        writeln!(f, "Set Type: {:?}", self.set_type)?;
        writeln!(f, "Interpolator Type: {:?}", self.interpolator_type)?;
        writeln!(f, "Error Type: {}", self.error_type)?;
        writeln!(f, "Particle: {}", self.hadron_pid)?;
        writeln!(f, "Flavor Scheme: {}", self.flavor_scheme)?;
        writeln!(f, "Order QCD: {}", self.order_qcd)?;
        writeln!(f, "AlphaS Order QCD: {}", self.alphas_order_qcd)?;
        writeln!(f, "MW: {}", self.m_w)?;
        writeln!(f, "MZ: {}", self.m_z)?;
        writeln!(f, "MUp: {}", self.m_up)?;
        writeln!(f, "MDown: {}", self.m_down)?;
        writeln!(f, "MStrange: {}", self.m_strange)?;
        writeln!(f, "MCharm: {}", self.m_charm)?;
        writeln!(f, "MBottom: {}", self.m_bottom)?;
        writeln!(f, "MTop: {}", self.m_top)?;
        writeln!(f, "AlphaS Type: {}", self.alphas_type)?;
        writeln!(f, "Number of PDF flavors: {}", self.number_flavors)
    }
}
