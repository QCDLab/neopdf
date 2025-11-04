use neopdf::gridpdf::GridArray;
use neopdf::writer::GridArrayCollection;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use super::gridpdf::PyGridArray;
use super::metadata::PyMetaData;

/// Python interface for `GridArrayCollection` utilities.
///
/// This module provides functions to compress, decompress, and extract metadata from
/// collections of PDF grids.
///
/// # Errors
///
/// Functions in this module may return a `PyRuntimeError` if the underlying compression
/// or decompression fails, such as due to missing files, invalid input, or I/O errors.
#[pymodule]
pub fn writer(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_compress, m)?)?;
    m.add_function(wrap_pyfunction!(py_decompress, m)?)?;
    m.add_function(wrap_pyfunction!(py_extract_metadata, m)?)?;
    Ok(())
}

/// Compresses and writes a collection of `GridArrays` and shared metadata to a file.
///
/// Compresses the provided grids and metadata and writes them to the specified file path.
///
/// # Parameters
///
/// - `grids`: The list of grid arrays to compress.
/// - `metadata`: The shared metadata for the grids.
/// - `path`: The output file path.
///
/// # Returns
///
/// Returns `Ok(())` if the compression succeeds, or a `PyRuntimeError` if the operation fails.
///
/// # Errors
///
/// Returns a `PyRuntimeError` if the compression process fails due to invalid input or I/O errors.
#[pyfunction(name = "compress")]
#[allow(clippy::needless_pass_by_value)]
pub fn py_compress(
    grids: Vec<PyRef<PyGridArray>>,
    metadata: &PyMetaData,
    path: &str,
) -> PyResult<()> {
    let grids: Vec<&GridArray> = grids.iter().map(|g| &g.gridarray).collect();
    GridArrayCollection::compress(&grids, &metadata.meta, path)
        .map_err(|e| PyRuntimeError::new_err(format!("Compress failed: {e}")))
}

/// Decompresses and loads all `GridArrays` and shared metadata from a file.
///
/// Loads and decompresses all grid arrays and their associated metadata from the specified file.
///
/// # Parameters
///
/// - `path`: The path to the compressed file.
///
/// # Returns
///
/// Returns a vector of tuples, each containing the metadata and grid array for a member.
///
/// # Panics
///
/// Panics if the file cannot be read or is not in the expected format.
#[must_use]
#[pyfunction(name = "decompress")]
pub fn py_decompress(path: &str) -> Vec<(PyMetaData, PyGridArray)> {
    let grid_meta = GridArrayCollection::decompress(path).unwrap();
    grid_meta
        .into_iter()
        .map(|gm| {
            let meta = PyMetaData {
                meta: gm.metadata.as_ref().clone(),
            };
            let gridarray = PyGridArray { gridarray: gm.grid };
            (meta, gridarray)
        })
        .collect()
}

/// Extracts just the metadata from a compressed file without loading the grids.
///
/// Loads only the metadata from the specified compressed file, without decompressing
/// the grid arrays.
///
/// # Parameters
///
/// - `path`: The path to the compressed file.
///
/// # Returns
///
/// Returns the metadata for the PDF set.
///
/// # Panics
///
/// Panics if the file cannot be read or is not in the expected format.
#[must_use]
#[pyfunction(name = "extract_metadata")]
pub fn py_extract_metadata(path: &str) -> PyMetaData {
    let meta = GridArrayCollection::extract_metadata(path).unwrap();
    PyMetaData { meta }
}

/// Registers the writer module with the parent Python module.
///
/// Adds the `writer` submodule to the parent Python module, exposing PDF grid writer
/// utilities to Python.
///
/// # Panics
///
/// Panics if the submodule cannot be created or added.
///
/// # Errors
///
/// Returns a `PyErr` if any function registration fails.
pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent_module.py(), "writer")?;
    m.setattr(
        pyo3::intern!(m.py(), "__doc__"),
        "PDF grid writer utilities.",
    )?;
    pyo3::py_run!(
        parent_module.py(),
        m,
        "import sys; sys.modules['neopdf.writer'] = m"
    );
    m.add_function(wrap_pyfunction!(py_compress, &m)?)?;
    m.add_function(wrap_pyfunction!(py_decompress, &m)?)?;
    m.add_function(wrap_pyfunction!(py_extract_metadata, &m)?)?;
    parent_module.add_submodule(&m)
}
