//! Generate `PyO3` interface for `neopdf`

#![allow(unsafe_op_in_unsafe_fn)]

use pyo3::prelude::*;

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
/// Python bindings for the `writer` module.
pub mod writer;

/// `PyO3` Python module that contains all exposed classes from Rust.
#[pymodule]
fn neopdf(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("version", env!("CARGO_PKG_VERSION"))?;
    pdf::register(m)?;
    metadata::register(m)?;
    converter::register(m)?;
    gridpdf::register(m)?;
    manage::register(m)?;
    parser::register(m)?;
    writer::register(m)?;
    Ok(())
}
