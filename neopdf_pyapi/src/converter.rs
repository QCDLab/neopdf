use neopdf::converter::{combine_lhapdf_npdfs, convert_lhapdf};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

/// Python interface for PDF set conversion utilities.
///
/// This module provides functions to convert LHAPDF sets to `NeoPDF` format and to combine
/// nuclear PDF sets into a single `NeoPDF` file.
///
/// # Errors
///
/// Functions in this module may return a `PyRuntimeError` if the underlying conversion or
/// combination fails, such as due to missing files, invalid input, or I/O errors.
#[pymodule]
pub fn converter(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_convert_lhapdf, m)?)?;
    m.add_function(wrap_pyfunction!(py_combine_lhapdf_npdfs, m)?)?;
    Ok(())
}

/// Converts an LHAPDF set to the `NeoPDF` format and writes it to disk.
///
/// Converts the specified LHAPDF set into the `NeoPDF` format and saves the result to the given
/// output path.
///
/// # Parameters
///
/// - `pdf_name`: The name of the LHAPDF set (e.g., "`NNPDF40_nnlo_as_01180`").
/// - `output_path`: The path to the output `NeoPDF` file.
///
/// # Returns
///
/// Returns `Ok(())` if the conversion succeeds, or a `PyRuntimeError` if the conversion fails.
///
/// # Errors
///
/// Returns a `PyRuntimeError` if the conversion process fails due to missing files, invalid
/// input, or I/O errors.
#[pyfunction(name = "convert_lhapdf")]
pub fn py_convert_lhapdf(pdf_name: &str, output_path: &str) -> PyResult<()> {
    convert_lhapdf(pdf_name, output_path)
        .map_err(|e| PyRuntimeError::new_err(format!("Conversion failed: {e}")))
}

/// Combines a list of nuclear PDF sets into a single `NeoPDF` file with explicit A dependence.
///
/// Combines multiple LHAPDF nuclear PDF sets into a single `NeoPDF` file, allowing for explicit
/// nuclear dependence.
///
/// # Parameters
///
/// - `pdf_names`: List of PDF set names (each with a different A).
/// - `output_path`: Output `NeoPDF` file path.
///
/// # Returns
///
/// Returns `Ok(())` if the combination succeeds, or a `PyRuntimeError` if the operation fails.
///
/// # Errors
///
/// Returns a `PyRuntimeError` if the combination process fails due to missing files, invalid input,
/// or I/O errors.
#[pyfunction(name = "combine_lhapdf_npdfs")]
#[allow(clippy::needless_pass_by_value)]
pub fn py_combine_lhapdf_npdfs(pdf_names: Vec<String>, output_path: &str) -> PyResult<()> {
    let pdf_names: Vec<&str> = pdf_names.iter().map(std::string::String::as_str).collect();
    combine_lhapdf_npdfs(&pdf_names, output_path)
        .map_err(|e| PyRuntimeError::new_err(format!("Combine failed: {e}")))
}

/// Registers the converter module with the parent Python module.
///
/// Adds the `converter` submodule to the parent Python module, exposing PDF set conversion
/// utilities to Python.
///
/// # Errors
///
/// Returns a `PyErr` if the submodule cannot be created or added, or if any function
/// registration fails.
pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent_module.py(), "converter")?;
    m.setattr(
        pyo3::intern!(m.py(), "__doc__"),
        "PDF set conversion utilities.",
    )?;
    pyo3::py_run!(
        parent_module.py(),
        m,
        "import sys; sys.modules['neopdf.converter'] = m"
    );
    m.add_function(wrap_pyfunction!(py_convert_lhapdf, &m)?)?;
    m.add_function(wrap_pyfunction!(py_combine_lhapdf_npdfs, &m)?)?;
    parent_module.add_submodule(&m)
}
