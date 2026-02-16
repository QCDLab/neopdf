//! Serializable types shared between the Rust backend and the JS frontend.

use serde::{Deserialize, Serialize};

/// Information about a single parameter dimension and its valid range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamRangeInfo {
    /// Parameter name (e.g. "x", "Q2", "nucleons", "alphas", "xi", "delta", "kt").
    pub name: String,
    /// Whether this dimension is active (has a non-trivial range).
    pub active: bool,
    /// Minimum value of the parameter range.
    pub min: f64,
    /// Maximum value of the parameter range.
    pub max: f64,
    /// Sensible default value for this parameter.
    pub default_value: f64,
}

/// The set of active parameters detected across the selected PDF sets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveParameters {
    /// One entry per potential dimension, in the canonical ordering:
    /// nucleons, alphas, xi, delta, kt, x, q2.
    pub params: Vec<ParamRangeInfo>,
}

/// Metadata for a loaded PDF set, surfaced to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfSetMetadata {
    pub set_name: String,
    pub set_desc: String,
    pub num_members: u32,
    pub x_min: f64,
    pub x_max: f64,
    pub q_min: f64,
    pub q_max: f64,
    pub flavors: Vec<i32>,
    pub error_type: String,
    pub flavor_scheme: String,
    pub order_qcd: u32,
    pub alphas_order_qcd: u32,
    pub m_z: f64,
    pub m_w: f64,
    pub m_charm: f64,
    pub m_bottom: f64,
    pub m_top: f64,
    pub format: String,
    pub polarised: bool,
    pub interpolator_type: String,
    pub git_version: String,
    pub code_version: String,
}

/// A request from the frontend to generate a plot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotRequest {
    /// Names of the PDF sets to plot.
    pub set_names: Vec<String>,
    /// Which parameter to sweep on the x-axis (e.g. "x", "Q2", "kt").
    pub x_axis_var: String,
    /// Parton flavor ID.
    pub pid: i32,
    /// Fixed values for each non-swept active parameter, keyed by name.
    pub fixed_values: std::collections::HashMap<String, f64>,
    /// Lower bound of the sweep range.
    pub range_min: f64,
    /// Upper bound of the sweep range.
    pub range_max: f64,
    /// Number of sample points.
    pub n_points: usize,
    /// Use logarithmic spacing for x-axis.
    pub x_log: bool,
    /// Use logarithmic scale for y-axis.
    pub y_log: bool,
    /// "standard" or "ratio".
    pub plot_type: String,
    /// For ratio plots: name of the reference set (denominator).
    pub ratio_reference: Option<String>,
}

/// Computed plot data for a single PDF set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotData {
    pub set_name: String,
    pub x_values: Vec<f64>,
    pub mean_values: Vec<f64>,
    pub upper_band: Vec<f64>,
    pub lower_band: Vec<f64>,
}
