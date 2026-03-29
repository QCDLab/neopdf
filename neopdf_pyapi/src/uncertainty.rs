use numpy::{PyArray1, PyArrayMethods};
use pyo3::prelude::*;

/// Python wrapper for the NeoPDF `Uncertainty` struct.
#[pyclass(name = "Uncertainty")]
#[derive(Clone, Debug)]
pub struct PyUncertainty {
    /// Central value.
    #[pyo3(get)]
    pub central: f64,
    /// Negative error (absolute value).
    #[pyo3(get)]
    pub errminus: f64,
    /// Positive error (absolute value).
    #[pyo3(get)]
    pub errplus: f64,
}

#[pymethods]
impl PyUncertainty {
    fn __repr__(&self) -> String {
        format!(
            "Uncertainty(central={}, errminus={}, errplus={})",
            self.central, self.errminus, self.errplus
        )
    }

    /// The symmetric error, defined as the average of `errminus` and `errplus`.
    #[must_use]
    pub fn errsymm(&self) -> f64 {
        (self.errminus + self.errplus) / 2.0
    }
}

/// Compute PDF uncertainty from per-member values.
///
/// # Errors
///
/// Raises an error if the computation of the uncertainty fails.
///
/// Parameters
/// ----------
/// values : numpy.ndarray
///     1D array of values for all members, with element 0 being the central member
///     (best-fit or average) and the following elements corresponding to error
///     replicas / eigenvectors, matching the LHAPDF/NeoPDF convention.
/// `error_type` : str
///     String describing the error type, typically taken from the metadata
///     `ErrorType` field (e.g. ``"replicas"``, ``"hessian"``, ``"symmhessian"``
///     or ``"asymhessian"``).
/// `error_conf_level` : float, optional
///     Confidence level (in %) at which the PDF's error members were constructed.
/// cl : float
///     Two-sided confidence level in percent (e.g. 68.2689 for 1σ).
/// alternative : bool
///     If ``True``, replica sets use a quantile-based (asymmetric) interval
///     instead of the standard deviation.
///
/// Returns
/// -------
/// Uncertainty
///     A small object with attributes ``central``, ``errminus`` and ``errplus``.
#[pyfunction]
#[pyo3(name = "uncertainty")]
#[pyo3(signature = (values, error_type, error_conf_level=None, cl=68.268_949_213_708_58, alternative=false))]
pub fn py_uncertainty<'py>(
    _py: Python<'py>,
    values: &Bound<'py, PyArray1<f64>>,
    error_type: &str,
    error_conf_level: Option<f64>,
    cl: f64,
    alternative: bool,
) -> PyResult<PyUncertainty> {
    let slice = unsafe { values.as_slice()? };
    let unc = neopdf::uncertainty::uncertainty(
        slice,
        error_type,
        error_conf_level.unwrap_or(neopdf::uncertainty::CL_1_SIGMA),
        cl,
        alternative,
    )
    .map_err(pyo3::exceptions::PyValueError::new_err)?;

    Ok(PyUncertainty {
        central: unc.central,
        errminus: unc.errminus,
        errplus: unc.errplus,
    })
}

/// Register the `uncertainty` submodule on the parent Python module.
///
/// # Errors
///
/// TODO
pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent_module.py(), "uncertainty")?;
    m.setattr(
        pyo3::intern!(m.py(), "__doc__"),
        "PDF set uncertainty utilities.",
    )?;
    pyo3::py_run!(
        parent_module.py(),
        m,
        "import sys; sys.modules['neopdf.uncertainty'] = m"
    );
    m.add("CL_1_SIGMA", neopdf::uncertainty::CL_1_SIGMA)?;
    m.add("CL_2_SIGMA", neopdf::uncertainty::CL_2_SIGMA)?;
    m.add("CL_3_SIGMA", neopdf::uncertainty::CL_3_SIGMA)?;
    m.add("CL_90", neopdf::uncertainty::CL_90)?;
    m.add("CL_95", neopdf::uncertainty::CL_95)?;
    m.add_class::<PyUncertainty>()?;
    m.add_function(wrap_pyfunction!(py_uncertainty, &m)?)?;
    parent_module.add_submodule(&m)
}
