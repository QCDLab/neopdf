//! Generate `PyO3` interface for `neopdf`

#![allow(unsafe_op_in_unsafe_fn)]

use pyo3::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};

use crate::pdf::{PyLoaderMethod, PyPDF, PyPDFSet};
use ::neopdf::manage::ManageData;

/// Python bindings for the `converter` module.
pub mod converter;
/// Python bindings for the `gridpdf` module.
pub mod gridpdf;
/// Python bindings for the `manage` module.
pub mod manage;
/// Python bindings for the `metadata` module.
pub mod metadata;
/// Python bindings for the `parser` module.
pub mod parser;
/// Python bindings for the `PDF` module.
pub mod pdf;
/// Python bindings for the `uncertainty` module.
pub mod uncertainty;
/// Python bindings for the `writer` module.
pub mod writer;

static VERBOSITY: AtomicI32 = AtomicI32::new(0);

/// Set the `NeoPDF` verbosity level (no-op, exists for LHAPDF API compatibility).
#[pyfunction]
#[pyo3(name = "setVerbosity")]
fn set_verbosity(level: i32) {
    VERBOSITY.store(level, Ordering::Relaxed);
}

/// Return the current verbosity level.
#[pyfunction]
fn verbosity() -> i32 {
    VERBOSITY.load(Ordering::Relaxed)
}

/// Return a `PDFSet` object for the named set (LHAPDF-compatible).
#[pyfunction]
#[pyo3(name = "getPDFSet")]
fn get_pdf_set(setname: &str) -> PyPDFSet {
    PyPDFSet::new(setname)
}

/// Load a single PDF member by set name and member index (LHAPDF-compatible).
#[pyfunction]
#[pyo3(name = "mkPDF")]
#[pyo3(signature = (pdf_name, member = 0))]
fn mk_pdf(pdf_name: &str, member: usize) -> PyPDF {
    PyPDF::new(pdf_name, member)
}

/// Load all members of a PDF set (LHAPDF-compatible).
#[pyfunction]
#[pyo3(name = "mkPDFs")]
fn mk_pdfs(pdf_name: &str) -> Vec<PyPDF> {
    PyPDF::mkpdfs(pdf_name, &PyLoaderMethod::Parallel)
}

/// Return a list of PDF set names available in the current data path.
#[pyfunction]
#[pyo3(name = "availablePDFSets")]
fn available_pdf_sets() -> Vec<String> {
    let data_path = ManageData::get_data_path();
    let mut sets = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&data_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                if path.join(format!("{name}.info")).exists() {
                    sets.push(name);
                }
            }
        }
    }
    sets.sort();
    sets
}

/// Return the list of data paths searched for PDF sets.
#[pyfunction]
fn paths() -> Vec<String> {
    vec![ManageData::get_data_path().to_string_lossy().into_owned()]
}

/// No-op: path management is controlled via the `NEOPDF_DATA_PATH` env var.
#[pyfunction]
#[pyo3(name = "setPaths")]
fn set_paths(_paths: Vec<String>) {}

/// No-op: path management is controlled via the `NEOPDF_DATA_PATH` env var.
#[pyfunction]
#[pyo3(name = "pathsAppend")]
fn paths_append(_path: String) {}

/// No-op: path management is controlled via the `NEOPDF_DATA_PATH` env var.
#[pyfunction]
#[pyo3(name = "pathsPrepend")]
fn paths_prepend(_path: String) {}

/// Return the NeoPDF version string (LHAPDF-compatible callable).
#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// `PyO3` Python module that contains all exposed classes from Rust.
#[pymodule]
fn neopdf(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    pdf::register(m)?;
    metadata::register(m)?;
    converter::register(m)?;
    gridpdf::register(m)?;
    manage::register(m)?;
    parser::register(m)?;
    writer::register(m)?;
    uncertainty::register(m)?;
    m.add_function(wrap_pyfunction!(set_verbosity, m)?)?;
    m.add_function(wrap_pyfunction!(verbosity, m)?)?;
    m.add_function(wrap_pyfunction!(get_pdf_set, m)?)?;
    m.add_function(wrap_pyfunction!(mk_pdf, m)?)?;
    m.add_function(wrap_pyfunction!(mk_pdfs, m)?)?;
    m.add_function(wrap_pyfunction!(available_pdf_sets, m)?)?;
    m.add_function(wrap_pyfunction!(paths, m)?)?;
    m.add_function(wrap_pyfunction!(set_paths, m)?)?;
    m.add_function(wrap_pyfunction!(paths_append, m)?)?;
    m.add_function(wrap_pyfunction!(paths_prepend, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}
