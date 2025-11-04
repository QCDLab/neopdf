use neopdf::manage::{ManageData, PdfSetFormat};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

/// Python wrapper for the `PdfSetFormat` enum.
#[pyclass(name = "PdfSetFormat")]
#[derive(Clone)]
pub enum PyPdfSetFormat {
    /// LHAPDF format (standard PDF set format used by LHAPDF).
    Lhapdf,
    /// `NeoPDF` format (native format for this library).
    Neopdf,
}

impl From<PyPdfSetFormat> for PdfSetFormat {
    fn from(fmt: PyPdfSetFormat) -> Self {
        match fmt {
            PyPdfSetFormat::Lhapdf => Self::Lhapdf,
            PyPdfSetFormat::Neopdf => Self::Neopdf,
        }
    }
}

/// Python wrapper for the `ManageData` struct.
#[pyclass(name = "ManageData")]
pub struct PyManageData {
    pub(crate) inner: ManageData,
}

#[pymethods]
impl PyManageData {
    /// Create a new `ManageData` instance.
    #[new]
    #[must_use]
    pub fn new(set_name: &str, format: PyPdfSetFormat) -> Self {
        Self {
            inner: ManageData::new(set_name, format.into()),
        }
    }

    /// Download the PDF set and extract it into the designated path.
    ///
    /// Attempts to download the PDF set and extract it to the appropriate directory.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the download and extraction succeed, or a `PyRuntimeError`
    /// if the operation fails.
    ///
    /// # Errors
    ///
    /// Returns a `PyRuntimeError` if the download or extraction fails due to network
    /// issues, missing files, or I/O errors.
    pub fn download_pdf(&self) -> PyResult<()> {
        self.inner
            .download_pdf()
            .map_err(|e| PyRuntimeError::new_err(format!("{e}")))
    }

    /// Check that the PDF set is installed in the correct path.
    ///
    /// Returns `true` if the PDF set is installed, `false` otherwise.
    #[must_use]
    pub fn is_pdf_installed(&self) -> bool {
        self.inner.is_pdf_installed()
    }

    /// Ensure that the PDF set is installed, otherwise download it.
    ///
    /// Ensures the PDF set is present; if not, attempts to download and install it.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the set is installed or successfully downloaded, or a
    /// `PyRuntimeError` if the operation fails.
    ///
    /// # Errors
    ///
    /// Returns a `PyRuntimeError` if the download or installation fails due to network
    /// issues, missing files, or I/O errors.
    pub fn ensure_pdf_installed(&self) -> PyResult<()> {
        self.inner
            .ensure_pdf_installed()
            .map_err(|e| PyRuntimeError::new_err(format!("{e}")))
    }

    /// Get the name of the PDF set.
    #[must_use]
    pub fn set_name(&self) -> &str {
        self.inner.set_name()
    }

    /// Get the path where PDF sets are stored.
    #[must_use]
    pub fn data_path(&self) -> String {
        self.inner.data_path().to_string_lossy().to_string()
    }

    /// Get the full path to this specific PDF set.
    #[must_use]
    pub fn set_path(&self) -> String {
        self.inner.set_path().to_string_lossy().to_string()
    }
}

/// Registers the manage module with the parent Python module.
///
/// Adds the `manage` submodule to the parent Python module, exposing PDF set
/// management utilities to Python.
///
/// # Errors
///
/// Returns a `PyErr` if the submodule cannot be created or added, or if any
/// class registration fails.
pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent_module.py(), "manage")?;
    m.setattr(
        pyo3::intern!(m.py(), "__doc__"),
        "PDF set management utilities.",
    )?;
    pyo3::py_run!(
        parent_module.py(),
        m,
        "import sys; sys.modules['neopdf.manage'] = m"
    );
    m.add_class::<PyPdfSetFormat>()?;
    m.add_class::<PyManageData>()?;
    parent_module.add_submodule(&m)
}
