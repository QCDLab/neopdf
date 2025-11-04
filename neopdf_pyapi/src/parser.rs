use neopdf::parser::{LhapdfSet, NeopdfSet};
use pyo3::prelude::*;

use super::gridpdf::PyGridArray;
use super::metadata::PyMetaData;

/// Python wrapper for the `LhapdfSet` struct.
#[pyclass(name = "LhapdfSet")]
pub struct PyLhapdfSet {
    pub(crate) inner: LhapdfSet,
}

#[pymethods]
impl PyLhapdfSet {
    /// Create a new `LhapdfSet` instance for a given PDF set name.
    #[new]
    #[must_use]
    pub fn new(pdf_name: &str) -> Self {
        Self {
            inner: LhapdfSet::new(pdf_name),
        }
    }

    /// Get the metadata for this set.
    #[must_use]
    pub fn info(&self) -> PyMetaData {
        PyMetaData {
            meta: self.inner.info.clone(),
        }
    }

    /// Get a member's metadata and grid array by index.
    #[must_use]
    pub fn member(&self, member: usize) -> (PyMetaData, PyGridArray) {
        let (meta, gridarray) = self.inner.member(member);
        let meta = PyMetaData { meta };
        let gridarray = PyGridArray { gridarray };

        (meta, gridarray)
    }

    /// Get all members' metadata and grid arrays.
    #[must_use]
    pub fn members(&self) -> Vec<(PyMetaData, PyGridArray)> {
        // TODO: Use the parallelized `members` in the crate
        let num_members = self.inner.members().len();
        (0..num_members).map(|idx| self.member(idx)).collect()
    }
}

/// Python wrapper for the `NeopdfSet` struct.
#[pyclass(name = "NeopdfSet")]
pub struct PyNeopdfSet {
    pub(crate) inner: NeopdfSet,
}

#[pymethods]
impl PyNeopdfSet {
    /// Create a new `NeopdfSet` instance for a given PDF set name.
    #[new]
    #[must_use]
    pub fn new(pdf_name: &str) -> Self {
        Self {
            inner: NeopdfSet::new(pdf_name),
        }
    }

    /// Get the metadata for this set.
    #[must_use]
    pub fn info(&self) -> PyMetaData {
        PyMetaData {
            meta: self.inner.info.clone(),
        }
    }

    /// Get a member's metadata and grid array by index.
    #[must_use]
    pub fn member(&self, member: usize) -> (PyMetaData, PyGridArray) {
        let (meta, gridarray) = self.inner.member(member);
        let meta = PyMetaData { meta };
        let gridarray = PyGridArray { gridarray };

        (meta, gridarray)
    }
}

/// Registers the parser module with the parent Python module.
///
/// Adds the `parser` submodule to the parent Python module, exposing
/// PDF set parser utilities to Python.
///
/// # Errors
///
/// Returns a `PyErr` if the submodule cannot be created or added, or
/// if any class registration fails.
pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent_module.py(), "parser")?;
    m.setattr(
        pyo3::intern!(m.py(), "__doc__"),
        "PDF set parser utilities.",
    )?;
    pyo3::py_run!(
        parent_module.py(),
        m,
        "import sys; sys.modules['neopdf.parser'] = m"
    );
    m.add_class::<PyLhapdfSet>()?;
    m.add_class::<PyNeopdfSet>()?;
    parent_module.add_submodule(&m)
}
