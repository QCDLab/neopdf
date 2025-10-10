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
    LogFourCubic,
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

/// Represents the information block of a given set (Version 2).
///
/// This version extends V1 with support for additional dimensions (xi and delta)
/// for 7-dimensional grids: (A, alphas, xi, delta, kt, x, Q2).
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
    /// Converts V2 metadata to V1 by dropping xi and delta fields.
    pub fn to_v1(&self) -> MetaDataV1 {
        MetaDataV1 {
            set_desc: self.set_desc.clone(),
            set_index: self.set_index,
            num_members: self.num_members,
            x_min: self.x_min,
            x_max: self.x_max,
            q_min: self.q_min,
            q_max: self.q_max,
            flavors: self.flavors.clone(),
            format: self.format.clone(),
            alphas_q_values: self.alphas_q_values.clone(),
            alphas_vals: self.alphas_vals.clone(),
            polarised: self.polarised,
            set_type: self.set_type.clone(),
            interpolator_type: self.interpolator_type.clone(),
            error_type: self.error_type.clone(),
            hadron_pid: self.hadron_pid,
            git_version: self.git_version.clone(),
            code_version: self.code_version.clone(),
            flavor_scheme: self.flavor_scheme.clone(),
            order_qcd: self.order_qcd,
            alphas_order_qcd: self.alphas_order_qcd,
            m_w: self.m_w,
            m_z: self.m_z,
            m_up: self.m_up,
            m_down: self.m_down,
            m_strange: self.m_strange,
            m_charm: self.m_charm,
            m_bottom: self.m_bottom,
            m_top: self.m_top,
            alphas_type: self.alphas_type.clone(),
            number_flavors: self.number_flavors,
        }
    }
}

impl From<MetaDataV1> for MetaDataV2 {
    /// Converts V1 metadata to V2 with default xi and delta values.
    fn from(v1: MetaDataV1) -> Self {
        Self {
            set_desc: v1.set_desc,
            set_index: v1.set_index,
            num_members: v1.num_members,
            x_min: v1.x_min,
            x_max: v1.x_max,
            q_min: v1.q_min,
            q_max: v1.q_max,
            flavors: v1.flavors,
            format: v1.format,
            alphas_q_values: v1.alphas_q_values,
            alphas_vals: v1.alphas_vals,
            polarised: v1.polarised,
            set_type: v1.set_type,
            interpolator_type: v1.interpolator_type,
            error_type: v1.error_type,
            hadron_pid: v1.hadron_pid,
            git_version: v1.git_version,
            code_version: v1.code_version,
            flavor_scheme: v1.flavor_scheme,
            order_qcd: v1.order_qcd,
            alphas_order_qcd: v1.alphas_order_qcd,
            m_w: v1.m_w,
            m_z: v1.m_z,
            m_up: v1.m_up,
            m_down: v1.m_down,
            m_strange: v1.m_strange,
            m_charm: v1.m_charm,
            m_bottom: v1.m_bottom,
            m_top: v1.m_top,
            alphas_type: v1.alphas_type,
            number_flavors: v1.number_flavors,
            xi_min: 1.0,
            xi_max: 1.0,
            delta_min: 0.0,
            delta_max: 0.0,
        }
    }
}

/// Version-aware metadata wrapper that handles serialization compatibility.
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum MetaData {
    V2(MetaDataV2),
    V1(MetaDataV1),
}

impl MetaData {
    /// Creates a new instance of V1 `MetaData`.
    pub fn new_v1(data: MetaDataV1) -> Self {
        Self::V1(data)
    }

    /// Creates a new instance of V2 `MetaData`.
    pub fn new_v2(data: MetaDataV2) -> Self {
        Self::V2(data)
    }

    /// Gets the current version as the latest available version.
    pub fn current_v1(data: MetaDataV1) -> Self {
        Self::V1(data)
    }

    /// Gets the current version as V2.
    pub fn current_v2(data: MetaDataV2) -> Self {
        Self::V2(data)
    }

    /// Gets the underlying data as V1 (latest compatible version for V1).
    pub fn as_latest(&self) -> MetaDataV1 {
        match self {
            MetaData::V1(data) => data.clone(),
            MetaData::V2(data) => data.to_v1(),
        }
    }

    /// Gets the underlying data as V2 (true latest version).
    pub fn as_latest_v2(&self) -> MetaDataV2 {
        match self {
            MetaData::V1(data) => data.clone().into(),
            MetaData::V2(data) => data.clone(),
        }
    }

    /// Checks if this metadata is V2.
    pub fn is_v2(&self) -> bool {
        matches!(self, MetaData::V2(_))
    }

    /// Gets the interpolator type (common field access).
    pub fn interpolator_type(&self) -> &InterpolatorType {
        match self {
            MetaData::V1(data) => &data.interpolator_type,
            MetaData::V2(data) => &data.interpolator_type,
        }
    }

    /// Gets the set description (common field access).
    pub fn set_desc(&self) -> &str {
        match self {
            MetaData::V1(data) => &data.set_desc,
            MetaData::V2(data) => &data.set_desc,
        }
    }

    /// Gets the alphas Q values (common field access).
    pub fn alphas_q_values(&self) -> &[f64] {
        match self {
            MetaData::V1(data) => &data.alphas_q_values,
            MetaData::V2(data) => &data.alphas_q_values,
        }
    }

    /// Gets the alphas values (common field access).
    pub fn alphas_vals(&self) -> &[f64] {
        match self {
            MetaData::V1(data) => &data.alphas_vals,
            MetaData::V2(data) => &data.alphas_vals,
        }
    }

    /// Gets the alphas order QCD (common field access).
    pub fn alphas_order_qcd(&self) -> u32 {
        match self {
            MetaData::V1(data) => data.alphas_order_qcd,
            MetaData::V2(data) => data.alphas_order_qcd,
        }
    }

    /// Gets the MZ value (common field access).
    pub fn m_z(&self) -> f64 {
        match self {
            MetaData::V1(data) => data.m_z,
            MetaData::V2(data) => data.m_z,
        }
    }

    /// Gets the quark masses (common field access).
    pub fn quark_masses(&self) -> (f64, f64, f64, f64, f64, f64) {
        match self {
            MetaData::V1(data) => (data.m_up, data.m_down, data.m_strange, data.m_charm, data.m_bottom, data.m_top),
            MetaData::V2(data) => (data.m_up, data.m_down, data.m_strange, data.m_charm, data.m_bottom, data.m_top),
        }
    }

    /// Gets the alphas type (common field access).
    pub fn alphas_type(&self) -> &str {
        match self {
            MetaData::V1(data) => &data.alphas_type,
            MetaData::V2(data) => &data.alphas_type,
        }
    }
}

impl Deref for MetaData {
    type Target = MetaDataV1;

    fn deref(&self) -> &Self::Target {
        // Note: This returns a reference to a temporary for V2, which is not ideal
        // but maintains backward compatibility. Consider using as_latest() directly instead.
        match self {
            MetaData::V1(data) => data,
            MetaData::V2(_) => {
                // For V2, we need to construct a V1 on the fly
                // This is a limitation of Deref - consider deprecating this
                panic!("Cannot use Deref on MetaData::V2 - use as_latest() or as_latest_v2() instead")
            }
        }
    }
}

impl DerefMut for MetaData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            MetaData::V1(data) => data,
            MetaData::V2(_) => {
                panic!("Cannot use DerefMut on MetaData::V2 - use as_latest_v2() instead")
            }
        }
    }
}

impl<'de> Deserialize<'de> for MetaData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Try deserializing as V2 first (which has all fields of V1 plus extras)
        // If it succeeds and has non-default V2-specific values OR LogFourCubic, it's V2
        // Otherwise, treat as V1 for backward compatibility
        let v2 = MetaDataV2::deserialize(deserializer)?;

        // Check if it has V2-specific data (non-zero xi or delta ranges or LogFourCubic)
        // Note: f64::default() is 0.0, so missing fields will be 0.0
        let has_v2_ranges = v2.xi_min.abs() > 1e-10
            || v2.xi_max.abs() > 1e-10
            || v2.delta_min.abs() > 1e-10
            || v2.delta_max.abs() > 1e-10;

        let has_v2_interpolator = matches!(v2.interpolator_type, InterpolatorType::LogFourCubic);

        if has_v2_ranges || has_v2_interpolator {
            Ok(MetaData::V2(v2))
        } else {
            // Convert to V1 for backward compatibility (all xi/delta fields are 0.0)
            Ok(MetaData::V1(v2.to_v1()))
        }
    }
}

impl fmt::Display for MetaData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MetaData::V1(data) => {
                writeln!(f, "Set Description: {}", data.set_desc)?;
                writeln!(f, "Set Index: {}", data.set_index)?;
                writeln!(f, "Number of Members: {}", data.num_members)?;
                writeln!(f, "XMin: {}", data.x_min)?;
                writeln!(f, "XMax: {}", data.x_max)?;
                writeln!(f, "QMin: {}", data.q_min)?;
                writeln!(f, "QMax: {}", data.q_max)?;
                writeln!(f, "Flavors: {:?}", data.flavors)?;
                writeln!(f, "Format: {}", data.format)?;
                writeln!(f, "AlphaS Q Values: {:?}", data.alphas_q_values)?;
                writeln!(f, "AlphaS Values: {:?}", data.alphas_vals)?;
                writeln!(f, "Polarized: {}", data.polarised)?;
                writeln!(f, "Set Type: {:?}", data.set_type)?;
                writeln!(f, "Interpolator Type: {:?}", data.interpolator_type)?;
                writeln!(f, "Error Type: {}", data.error_type)?;
                writeln!(f, "Particle: {}", data.hadron_pid)?;
                writeln!(f, "Flavor Scheme: {}", data.flavor_scheme)?;
                writeln!(f, "Order QCD: {}", data.order_qcd)?;
                writeln!(f, "AlphaS Order QCD: {}", data.alphas_order_qcd)?;
                writeln!(f, "MW: {}", data.m_w)?;
                writeln!(f, "MZ: {}", data.m_z)?;
                writeln!(f, "MUp: {}", data.m_up)?;
                writeln!(f, "MDown: {}", data.m_down)?;
                writeln!(f, "MStrange: {}", data.m_strange)?;
                writeln!(f, "MCharm: {}", data.m_charm)?;
                writeln!(f, "MBottom: {}", data.m_bottom)?;
                writeln!(f, "MTop: {}", data.m_top)?;
                writeln!(f, "AlphaS Type: {}", data.alphas_type)?;
                writeln!(f, "Number of PDF flavors: {}", data.number_flavors)
            }
            MetaData::V2(data) => {
                writeln!(f, "Set Description: {}", data.set_desc)?;
                writeln!(f, "Set Index: {}", data.set_index)?;
                writeln!(f, "Number of Members: {}", data.num_members)?;
                writeln!(f, "XMin: {}", data.x_min)?;
                writeln!(f, "XMax: {}", data.x_max)?;
                writeln!(f, "QMin: {}", data.q_min)?;
                writeln!(f, "QMax: {}", data.q_max)?;
                writeln!(f, "XiMin: {}", data.xi_min)?;
                writeln!(f, "XiMax: {}", data.xi_max)?;
                writeln!(f, "DeltaMin: {}", data.delta_min)?;
                writeln!(f, "DeltaMax: {}", data.delta_max)?;
                writeln!(f, "Flavors: {:?}", data.flavors)?;
                writeln!(f, "Format: {}", data.format)?;
                writeln!(f, "AlphaS Q Values: {:?}", data.alphas_q_values)?;
                writeln!(f, "AlphaS Values: {:?}", data.alphas_vals)?;
                writeln!(f, "Polarized: {}", data.polarised)?;
                writeln!(f, "Set Type: {:?}", data.set_type)?;
                writeln!(f, "Interpolator Type: {:?}", data.interpolator_type)?;
                writeln!(f, "Error Type: {}", data.error_type)?;
                writeln!(f, "Particle: {}", data.hadron_pid)?;
                writeln!(f, "Flavor Scheme: {}", data.flavor_scheme)?;
                writeln!(f, "Order QCD: {}", data.order_qcd)?;
                writeln!(f, "AlphaS Order QCD: {}", data.alphas_order_qcd)?;
                writeln!(f, "MW: {}", data.m_w)?;
                writeln!(f, "MZ: {}", data.m_z)?;
                writeln!(f, "MUp: {}", data.m_up)?;
                writeln!(f, "MDown: {}", data.m_down)?;
                writeln!(f, "MStrange: {}", data.m_strange)?;
                writeln!(f, "MCharm: {}", data.m_charm)?;
                writeln!(f, "MBottom: {}", data.m_bottom)?;
                writeln!(f, "MTop: {}", data.m_top)?;
                writeln!(f, "AlphaS Type: {}", data.alphas_type)?;
                writeln!(f, "Number of PDF flavors: {}", data.number_flavors)
            }
        }
    }
}
