//! Python subprocess bridge to matplotlib / mpld3 for rendering PDF plots.
//!
//! We invoke a Python subprocess rather than embedding via PyO3 because the
//! workspace's `neopdf_pyapi` already uses `pyo3` with the `extension-module`
//! feature, which is mutually exclusive with the `auto-initialize` feature
//! needed for embedding. Using a subprocess avoids this conflict entirely.

use std::io::Write;
use std::process::Command;

use crate::types::PlotData;

/// The embedded Python plotting script source.
const PLOTTER_PY: &str = include_str!("../python/neopdf_plotter.py");

/// Build a complete Python script that includes the plotter module and a
/// driver snippet. The plot data is passed as JSON via stdin.
fn make_script(driver: &str) -> String {
    format!(
        "import sys, json\n\
         {PLOTTER_PY}\n\
         {driver}\n"
    )
}

/// Render an interactive HTML plot via a Python subprocess running
/// matplotlib + mpld3.
///
/// Returns an HTML string containing the mpld3 figure.
pub fn render_plot_html(
    plot_data: &[PlotData],
    x_label: &str,
    y_label: &str,
    x_log: bool,
    y_log: bool,
    title: &str,
) -> Result<String, String> {
    let data_json =
        serde_json::to_string(plot_data).map_err(|e| format!("JSON serialization error: {e}"))?;

    let driver = format!(
        "data = json.loads(sys.stdin.read())\n\
         html = render_plot(data, {}, {}, {}, {}, {})\n\
         sys.stdout.write(html)",
        py_str(x_label),
        py_str(y_label),
        py_bool(x_log),
        py_bool(y_log),
        py_str(title),
    );

    run_python(&make_script(&driver), &data_json)
}

/// Save a plot to a file (PNG, PDF, SVG) via a Python subprocess.
pub fn save_plot_to_file(
    plot_data: &[PlotData],
    x_label: &str,
    y_label: &str,
    x_log: bool,
    y_log: bool,
    title: &str,
    output_path: &str,
) -> Result<(), String> {
    let data_json =
        serde_json::to_string(plot_data).map_err(|e| format!("JSON serialization error: {e}"))?;

    let driver = format!(
        "data = json.loads(sys.stdin.read())\n\
         save_plot(data, {}, {}, {}, {}, {}, {})",
        py_str(x_label),
        py_str(y_label),
        py_bool(x_log),
        py_bool(y_log),
        py_str(title),
        py_str(output_path),
    );

    run_python(&make_script(&driver), &data_json)?;
    Ok(())
}

/// Run a Python script via subprocess, feeding `stdin_data` on stdin.
/// Returns stdout on success.
fn run_python(script: &str, stdin_data: &str) -> Result<String, String> {
    let mut child = Command::new("python3")
        .args(["-c", script])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn python3: {e}"))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(stdin_data.as_bytes())
            .map_err(|e| format!("Failed to write to python stdin: {e}"))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for python3: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Python error: {stderr}"));
    }

    String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8 from python: {e}"))
}

/// Format a Rust string as a Python string literal.
fn py_str(s: &str) -> String {
    // Use JSON encoding which produces valid Python string literals
    serde_json::to_string(s).unwrap_or_else(|_| format!("\"{s}\""))
}

/// Format a bool as a Python boolean literal.
fn py_bool(b: bool) -> &'static str {
    if b {
        "True"
    } else {
        "False"
    }
}
