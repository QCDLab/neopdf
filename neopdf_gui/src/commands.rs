//! Tauri command handlers for the `NeoPDF` GUI.

use std::collections::HashMap;

use neopdf::pdf::PDF;
use rayon::prelude::*;
use tauri::{AppHandle, State};

use crate::plotting;
use crate::settings;
use crate::state::{AppState, LoadedPdfSet};
use crate::types::{ActiveParameters, ParamRangeInfo, PdfSetMetadata, PlotData, PlotRequest};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Canonical ordering of the extra parameter dimensions.
/// The `points` slice passed to `pdf.xfxq2(pid, &points)` is built in this
/// order: active extra params first (nucleons, alphas, xi, delta, kt),
/// then x, then q2.
const PARAM_ORDER: &[(&str, f64)] = &[
    ("nucleons", 1.0),
    ("alphas", 0.118),
    ("xi", 0.0),
    ("delta", 0.0),
    ("kt", 0.0),
    ("x", 0.1),
    ("Q2", 100.0),
];

/// Extract the `ParamRangeInfo` for one dimension from a PDF.
fn param_info_from_pdf(pdf: &PDF, name: &str, default_value: f64) -> ParamRangeInfo {
    let ranges = pdf.param_ranges();
    let (min, max) = match name {
        "nucleons" => (ranges.nucleons.min, ranges.nucleons.max),
        "alphas" => (ranges.alphas.min, ranges.alphas.max),
        "xi" => (ranges.xi.min, ranges.xi.max),
        "delta" => (ranges.delta.min, ranges.delta.max),
        "kt" => (ranges.kt.min, ranges.kt.max),
        "x" => (ranges.x.min, ranges.x.max),
        "Q2" => (ranges.q2.min, ranges.q2.max),
        _ => (0.0, 0.0),
    };
    let active = min < max;
    ParamRangeInfo {
        name: name.to_string(),
        active,
        min,
        max,
        default_value,
    }
}

/// Build the flat `points` slice for `xfxq2` from the active parameters,
/// the x-axis variable, and the current sweep value.
fn build_points(
    active: &[ParamRangeInfo],
    x_axis_var: &str,
    sweep_val: f64,
    fixed: &HashMap<String, f64>,
) -> Vec<f64> {
    active
        .iter()
        .filter(|p| p.active)
        .map(|p| {
            if p.name == x_axis_var {
                sweep_val
            } else {
                fixed.get(&p.name).copied().unwrap_or(p.default_value)
            }
        })
        .collect()
}

/// Generate linearly- or log-spaced sample points.
#[allow(clippy::cast_precision_loss)]
fn linspace(min: f64, max: f64, n: usize, log: bool) -> Vec<f64> {
    if n <= 1 {
        return vec![min];
    }
    if log {
        let log_min = min.ln();
        let log_max = max.ln();
        (0..n)
            .map(|i| (log_min + (log_max - log_min) * i as f64 / (n - 1) as f64).exp())
            .collect()
    } else {
        (0..n)
            .map(|i| min + (max - min) * i as f64 / (n - 1) as f64)
            .collect()
    }
}

/// Compute mean and +/- 1-sigma band from member evaluations.
#[allow(clippy::cast_precision_loss)]
fn mean_and_band(values: &[f64]) -> (f64, f64, f64) {
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
    let std = variance.sqrt();
    (mean, mean + std, mean - std)
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// Load a PDF set and store it in the app state. Returns metadata.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn load_pdf_set(name: String, state: State<'_, AppState>) -> Result<PdfSetMetadata, String> {
    let members = PDF::load_pdfs(&name);
    if members.is_empty() {
        return Err(format!("No members found for PDF set '{name}'"));
    }

    let meta = members[0].metadata();
    let metadata = PdfSetMetadata {
        set_name: name.clone(),
        set_desc: meta.set_desc.clone(),
        num_members: meta.num_members,
        x_min: meta.x_min,
        x_max: meta.x_max,
        q_min: meta.q_min,
        q_max: meta.q_max,
        flavors: meta.flavors.clone(),
        error_type: meta.error_type.clone(),
        flavor_scheme: meta.flavor_scheme.clone(),
        order_qcd: meta.order_qcd,
        alphas_order_qcd: meta.alphas_order_qcd,
        m_z: meta.m_z,
        m_w: meta.m_w,
        m_charm: meta.m_charm,
        m_bottom: meta.m_bottom,
        m_top: meta.m_top,
        format: meta.format.clone(),
        polarised: meta.polarised,
        interpolator_type: format!("{:?}", meta.interpolator_type),
        git_version: meta.git_version.clone(),
        code_version: meta.code_version.clone(),
    };

    state
        .sets
        .lock()
        .map_err(|e| e.to_string())?
        .insert(name, LoadedPdfSet { members });

    Ok(metadata)
}

/// Remove a PDF set from the loaded state.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn remove_pdf_set(name: String, state: State<'_, AppState>) -> Result<(), String> {
    state.sets.lock().map_err(|e| e.to_string())?.remove(&name);

    Ok(())
}

/// Return the names of all currently loaded PDF sets.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_loaded_sets(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let sets = state.sets.lock().map_err(|e| e.to_string())?;
    Ok(sets.keys().cloned().collect())
}

/// Detect which parameter dimensions are active across the given PDF sets.
///
/// A dimension is active if it has a non-trivial range (min < max) in *all*
/// of the selected sets (intersection semantics, mirroring the original
/// C++ `updateParametersUI`).
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::significant_drop_tightening)]
pub fn detect_active_parameters(
    set_names: Vec<String>,
    state: State<'_, AppState>,
) -> Result<ActiveParameters, String> {
    let params = {
        let sets = state.sets.lock().map_err(|e| e.to_string())?;

        let mut result: Option<Vec<ParamRangeInfo>> = None;

        for name in &set_names {
            let loaded = sets
                .get(name)
                .ok_or_else(|| format!("Set '{name}' not loaded"))?;
            let pdf = &loaded.members[0];

            let infos: Vec<ParamRangeInfo> = PARAM_ORDER
                .iter()
                .map(|(pname, default)| param_info_from_pdf(pdf, pname, *default))
                .collect();

            result = Some(match result {
                None => infos,
                Some(prev) => prev
                    .into_iter()
                    .zip(infos)
                    .map(|(mut a, b)| {
                        // Intersection: only active if active in both
                        a.active = a.active && b.active;
                        if a.active {
                            a.min = a.min.max(b.min);
                            a.max = a.max.min(b.max);
                        }
                        a
                    })
                    .collect(),
            });
        }

        result.unwrap_or_default()
    };

    Ok(ActiveParameters { params })
}

/// Get metadata for a specific loaded PDF set.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::significant_drop_tightening)]
pub fn get_set_metadata(
    name: String,
    state: State<'_, AppState>,
) -> Result<PdfSetMetadata, String> {
    let meta = {
        let sets = state.sets.lock().map_err(|e| e.to_string())?;
        let loaded = sets
            .get(&name)
            .ok_or_else(|| format!("Set '{name}' not loaded"))?;
        loaded.members[0].metadata().clone()
    };

    Ok(PdfSetMetadata {
        set_name: name,
        set_desc: meta.set_desc.clone(),
        num_members: meta.num_members,
        x_min: meta.x_min,
        x_max: meta.x_max,
        q_min: meta.q_min,
        q_max: meta.q_max,
        flavors: meta.flavors.clone(),
        error_type: meta.error_type.clone(),
        flavor_scheme: meta.flavor_scheme.clone(),
        order_qcd: meta.order_qcd,
        alphas_order_qcd: meta.alphas_order_qcd,
        m_z: meta.m_z,
        m_w: meta.m_w,
        m_charm: meta.m_charm,
        m_bottom: meta.m_bottom,
        m_top: meta.m_top,
        format: meta.format.clone(),
        polarised: meta.polarised,
        interpolator_type: format!("{:?}", meta.interpolator_type),
        git_version: meta.git_version.clone(),
        code_version: meta.code_version,
    })
}

/// Return the intersection of available PIDs across the selected sets.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::significant_drop_tightening)]
pub fn get_available_pids(
    set_names: Vec<String>,
    state: State<'_, AppState>,
) -> Result<Vec<i32>, String> {
    let mut pids = {
        let sets = state.sets.lock().map_err(|e| e.to_string())?;

        let mut common_pids: Option<Vec<i32>> = None;

        for name in &set_names {
            let loaded = sets
                .get(name)
                .ok_or_else(|| format!("Set '{name}' not loaded"))?;
            let pids: Vec<i32> = loaded.members[0].pids().to_vec();

            common_pids = Some(match common_pids {
                None => pids,
                Some(prev) => prev.into_iter().filter(|p| pids.contains(p)).collect(),
            });
        }

        common_pids.unwrap_or_default()
    };
    // Sort: gluon (21) first, then positive quarks ascending, then negative descending
    pids.sort_by_key(|&p| {
        if p == 21 {
            (-1000, 0)
        } else if p > 0 {
            (0, p)
        } else {
            (1, -p)
        }
    });
    Ok(pids)
}

/// Evaluate `alpha_s(Q^2)` at the given Q^2 values using the first member
/// of the specified PDF set.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::significant_drop_tightening)]
pub fn compute_alphas(
    set_name: String,
    q2_values: Vec<f64>,
    state: State<'_, AppState>,
) -> Result<Vec<f64>, String> {
    let result = {
        let sets = state.sets.lock().map_err(|e| e.to_string())?;
        let loaded = sets
            .get(&set_name)
            .ok_or_else(|| format!("Set '{set_name}' not loaded"))?;
        let pdf = &loaded.members[0];

        q2_values.iter().map(|&q2| pdf.alphas_q2(q2)).collect()
    };

    Ok(result)
}

/// Core plotting command: evaluate PDF members, compute statistics,
/// and render the plot via matplotlib (PNG encoded as base64).
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::significant_drop_tightening)]
pub fn generate_plot(request: PlotRequest, state: State<'_, AppState>) -> Result<String, String> {
    // First detect the active parameters from the first set
    let (_, active_params) = {
        let sets = state.sets.lock().map_err(|e| e.to_string())?;

        let first_name = request.set_names.first().ok_or("No sets specified")?;
        let first_set = sets
            .get(first_name)
            .ok_or_else(|| format!("Set '{first_name}' not loaded"))?;

        let active_params: Vec<ParamRangeInfo> = PARAM_ORDER
            .iter()
            .map(|(pname, default)| param_info_from_pdf(&first_set.members[0], pname, *default))
            .collect();

        (first_name.clone(), active_params)
    };

    let x_values = linspace(
        request.range_min,
        request.range_max,
        request.n_points,
        request.x_log,
    );

    // Compute PlotData for each requested set
    let mut all_plot_data: Vec<PlotData> = Vec::new();

    for set_name in &request.set_names {
        let sets = state.sets.lock().map_err(|e| e.to_string())?;
        let loaded = sets
            .get(set_name)
            .ok_or_else(|| format!("Set '{set_name}' not loaded"))?;

        let mut means = Vec::with_capacity(request.n_points);
        let mut uppers = Vec::with_capacity(request.n_points);
        let mut lowers = Vec::with_capacity(request.n_points);

        for &xv in &x_values {
            let points = build_points(
                &active_params,
                &request.x_axis_var,
                xv,
                &request.fixed_values,
            );

            // Evaluate all members in parallel
            let results: Vec<f64> = loaded
                .members
                .par_iter()
                .map(|pdf| pdf.xfxq2(request.pid, &points))
                .collect();

            let (mean, upper, lower) = mean_and_band(&results);
            means.push(mean);
            uppers.push(upper);
            lowers.push(lower);
        }

        all_plot_data.push(PlotData {
            set_name: set_name.clone(),
            x_values: x_values.clone(),
            mean_values: means,
            upper_band: uppers,
            lower_band: lowers,
        });
    }

    // Handle ratio mode
    if request.plot_type == "ratio" {
        if let Some(ref_name) = &request.ratio_reference {
            let ref_data = all_plot_data
                .iter()
                .find(|d| &d.set_name == ref_name)
                .ok_or_else(|| format!("Ratio reference set '{ref_name}' not found in results"))?
                .clone();

            for d in &mut all_plot_data {
                for i in 0..d.x_values.len() {
                    let denom = ref_data.mean_values[i];
                    if denom.abs() > 1e-30 {
                        d.mean_values[i] /= denom;
                        d.upper_band[i] /= denom;
                        d.lower_band[i] /= denom;
                    } else {
                        d.mean_values[i] = 1.0;
                        d.upper_band[i] = 1.0;
                        d.lower_band[i] = 1.0;
                    }
                }
            }
        }
    }

    let y_label = if request.plot_type == "ratio" {
        format!(
            "Ratio to {}",
            request.ratio_reference.as_deref().unwrap_or("ref")
        )
    } else {
        "xf".to_string()
    };

    let title = format!("PID = {}", request.pid);

    plotting::render_plot_html(
        &all_plot_data,
        &request.x_axis_var,
        &y_label,
        request.x_log,
        request.y_log,
        &title,
    )
}

/// Export the plot to a file (PNG, PDF, SVG).
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::significant_drop_tightening)]
pub fn export_plot_png(
    request: PlotRequest,
    output_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let active_params = {
        let sets = state.sets.lock().map_err(|e| e.to_string())?;

        let first_name = request.set_names.first().ok_or("No sets specified")?;
        let first_set = sets
            .get(first_name)
            .ok_or_else(|| format!("Set '{first_name}' not loaded"))?;

        let active_params: Vec<ParamRangeInfo> = PARAM_ORDER
            .iter()
            .map(|(pname, default)| param_info_from_pdf(&first_set.members[0], pname, *default))
            .collect();

        active_params
    };

    let x_values = linspace(
        request.range_min,
        request.range_max,
        request.n_points,
        request.x_log,
    );

    let mut all_plot_data: Vec<PlotData> = Vec::new();

    for set_name in &request.set_names {
        let sets = state.sets.lock().map_err(|e| e.to_string())?;
        let loaded = sets
            .get(set_name)
            .ok_or_else(|| format!("Set '{set_name}' not loaded"))?;

        let mut means = Vec::with_capacity(request.n_points);
        let mut uppers = Vec::with_capacity(request.n_points);
        let mut lowers = Vec::with_capacity(request.n_points);

        for &xv in &x_values {
            let points = build_points(
                &active_params,
                &request.x_axis_var,
                xv,
                &request.fixed_values,
            );

            let results: Vec<f64> = loaded
                .members
                .par_iter()
                .map(|pdf| pdf.xfxq2(request.pid, &points))
                .collect();

            let (mean, upper, lower) = mean_and_band(&results);
            means.push(mean);
            uppers.push(upper);
            lowers.push(lower);
        }

        all_plot_data.push(PlotData {
            set_name: set_name.clone(),
            x_values: x_values.clone(),
            mean_values: means,
            upper_band: uppers,
            lower_band: lowers,
        });
    }

    if request.plot_type == "ratio" {
        if let Some(ref_name) = &request.ratio_reference {
            let ref_data = all_plot_data
                .iter()
                .find(|d| &d.set_name == ref_name)
                .ok_or_else(|| format!("Ratio reference set '{ref_name}' not found"))?
                .clone();

            for d in &mut all_plot_data {
                for i in 0..d.x_values.len() {
                    let denom = ref_data.mean_values[i];
                    if denom.abs() > 1e-30 {
                        d.mean_values[i] /= denom;
                        d.upper_band[i] /= denom;
                        d.lower_band[i] /= denom;
                    } else {
                        d.mean_values[i] = 1.0;
                        d.upper_band[i] = 1.0;
                        d.lower_band[i] = 1.0;
                    }
                }
            }
        }
    }

    let y_label = if request.plot_type == "ratio" {
        format!(
            "Ratio to {}",
            request.ratio_reference.as_deref().unwrap_or("ref")
        )
    } else {
        "xf".to_string()
    };

    let title = format!("PID = {}", request.pid);

    plotting::save_plot_to_file(
        &all_plot_data,
        &request.x_axis_var,
        &y_label,
        request.x_log,
        request.y_log,
        &title,
        &output_path,
    )
}

// ---------------------------------------------------------------------------
// Settings: grid data path (NEOPDF_DATA_PATH)
// ---------------------------------------------------------------------------

/// Return the currently stored grid data folder path, or empty string if not set.
#[tauri::command]
pub fn get_data_path_setting(app: AppHandle) -> Result<String, String> {
    let config = settings::load_config(&app)?;
    Ok(config.data_path.unwrap_or_default())
}

/// Set the grid data folder path. Pass an empty string or null to clear.
/// This is persisted and applied to NEOPDF_DATA_PATH for subsequent PDF loads.
#[tauri::command]
pub fn set_data_path_setting(app: AppHandle, path: Option<String>) -> Result<(), String> {
    let path_opt = path.map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    let config = settings::GuiConfig {
        data_path: path_opt.clone(),
    };
    settings::save_config(&app, &config)?;
    if let Some(p) = path_opt {
        std::env::set_var("NEOPDF_DATA_PATH", p);
    } else {
        std::env::remove_var("NEOPDF_DATA_PATH");
    }
    Ok(())
}
