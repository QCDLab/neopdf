use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use neopdf::pdf::PDF;
use neopdf::precision::Arithmetic;
use neopdf::precision_pdf::{PrecisionError, PrecisionPDF};

/// A scalar of a Python arithmetic: any Python object.
struct Scalar(Py<PyAny>);

impl Clone for Scalar {
    fn clone(&self) -> Self {
        Python::attach(|py| Self(self.0.clone_ref(py)))
    }
}

struct Callables {
    lift: Py<PyAny>,
    ln: Py<PyAny>,
    is_finite: Py<PyAny>,
    /// The first exception raised by Python, which `Arithmetic` has no way to return.
    error: Mutex<Option<PyErr>>,
}

/// The arithmetic of Python numbers: `lift`, `ln` and `is_finite` are callables, and
/// the scalars' own operators add, subtract, multiply, divide and compare.
///
/// Once Python raises, every operation is skipped until `finish` takes the exception.
#[derive(Clone)]
struct PyArithmetic(Arc<Callables>);

impl PyArithmetic {
    /// Runs `op` unless Python already raised, keeping its exception and returning `fallback`
    /// if it raises. The lock is not held while Python runs.
    fn guard<T>(&self, fallback: T, op: impl FnOnce() -> PyResult<T>) -> T {
        if self.0.error.lock().expect("unpoisoned").is_some() {
            return fallback;
        }
        op().unwrap_or_else(|err| {
            self.0.error.lock().expect("unpoisoned").get_or_insert(err);
            fallback
        })
    }

    fn scalar(
        &self,
        op: impl for<'py> FnOnce(Python<'py>) -> PyResult<Bound<'py, PyAny>>,
    ) -> Scalar {
        Python::attach(|py| Scalar(self.guard(py.None(), || op(py).map(Bound::unbind))))
    }

    /// The exception raised by Python first, if any, or else the evaluation's own result.
    fn finish<T>(&self, result: Result<T, PrecisionError>) -> PyResult<T> {
        let raised = self.0.error.lock().expect("unpoisoned").take();
        raised.map_or_else(
            || result.map_err(|err| PyValueError::new_err(err.to_string())),
            Err,
        )
    }
}

impl Arithmetic for PyArithmetic {
    type Scalar = Scalar;

    fn lift(&self, value: f64) -> Scalar {
        self.scalar(|py| self.0.lift.bind(py).call1((value,)))
    }

    fn add(&self, a: &Scalar, b: &Scalar) -> Scalar {
        self.scalar(|py| a.0.bind(py).add(b.0.bind(py)))
    }

    fn sub(&self, a: &Scalar, b: &Scalar) -> Scalar {
        self.scalar(|py| a.0.bind(py).sub(b.0.bind(py)))
    }

    fn mul(&self, a: &Scalar, b: &Scalar) -> Scalar {
        self.scalar(|py| a.0.bind(py).mul(b.0.bind(py)))
    }

    fn div(&self, a: &Scalar, b: &Scalar) -> Scalar {
        self.scalar(|py| a.0.bind(py).div(b.0.bind(py)))
    }

    fn ln(&self, a: &Scalar) -> Scalar {
        self.scalar(|py| self.0.ln.bind(py).call1((a.0.bind(py),)))
    }

    fn compare(&self, a: &Scalar, b: &Scalar) -> Option<Ordering> {
        Python::attach(|py| {
            let (a, b) = (a.0.bind(py), b.0.bind(py));
            self.guard(None, || {
                Ok(if a.lt(b)? {
                    Some(Ordering::Less)
                } else if a.gt(b)? {
                    Some(Ordering::Greater)
                } else if a.eq(b)? {
                    Some(Ordering::Equal)
                } else {
                    None
                })
            })
        })
    }

    fn is_finite(&self, a: &Scalar) -> bool {
        Python::attach(|py| {
            self.guard(false, || {
                self.0
                    .is_finite
                    .bind(py)
                    .call1((a.0.bind(py),))?
                    .is_truthy()
            })
        })
    }
}

/// The collinear `xfxQ2` and `alphasQ2` of a PDF member in a caller-chosen
/// arithmetic.
///
/// Obtained from `PDF.with_arithmetic`. The view holds its own copy of the
/// member's data, rebuilt in that arithmetic.
#[pyclass(name = "PrecisionPDF", frozen)]
pub struct PyPrecisionPDF {
    view: PrecisionPDF<PyArithmetic>,
    arithmetic: PyArithmetic,
}

impl PyPrecisionPDF {
    pub(crate) fn new(
        pdf: &PDF,
        lift: Py<PyAny>,
        ln: Py<PyAny>,
        is_finite: Py<PyAny>,
    ) -> PyResult<Self> {
        let arithmetic = PyArithmetic(Arc::new(Callables {
            lift,
            ln,
            is_finite,
            error: Mutex::new(None),
        }));
        let view = arithmetic.finish(pdf.with_arithmetic(arithmetic.clone()))?;
        Ok(Self { view, arithmetic })
    }
}

// The exceptions are documented in the docstrings' `Raises` sections.
#[pymethods]
#[allow(clippy::doc_markdown, clippy::missing_errors_doc)]
impl PyPrecisionPDF {
    /// Interpolates `x f(x, Q2)` in the view's arithmetic.
    ///
    /// Parameters
    /// ----------
    /// id : int
    ///     The flavor ID (e.g., 21 for gluon, 1 for d-quark).
    /// x : Any
    ///     The momentum fraction, as a number of the view's arithmetic.
    /// q2 : Any
    ///     The energy scale squared, as a number of the view's arithmetic.
    ///
    /// Returns
    /// -------
    /// Any
    ///     The interpolated value, as a number of the view's arithmetic.
    ///     A flavor that the set does not contain gives `lift(0.0)`.
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     If `(x, Q2)` lies outside every subgrid (there is no extrapolation),
    ///     or is not finite.
    #[pyo3(name = "xfxQ2")]
    pub fn xfxq2(&self, id: i32, x: Py<PyAny>, q2: Py<PyAny>) -> PyResult<Py<PyAny>> {
        let value = self.view.xfxq2(id, &Scalar(x), &Scalar(q2));
        self.arithmetic.finish(value).map(|value| value.0)
    }

    /// Interpolates the strong coupling `alpha_s(Q2)` from the set's table,
    /// in the view's arithmetic.
    ///
    /// Parameters
    /// ----------
    /// q2 : Any
    ///     The energy scale squared, as a number of the view's arithmetic.
    ///
    /// Returns
    /// -------
    /// Any
    ///     The interpolated value, as a number of the view's arithmetic.
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     If `Q2` lies outside the table, or the set has an analytic `alpha_s`.
    #[pyo3(name = "alphasQ2")]
    pub fn alphas_q2(&self, q2: Py<PyAny>) -> PyResult<Py<PyAny>> {
        let value = self.view.alphas_q2(&Scalar(q2));
        self.arithmetic.finish(value).map(|value| value.0)
    }
}
