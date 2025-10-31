use numpy::{IntoPyArray, PyArray2};
use pyo3::prelude::*;
use std::sync::Mutex;

use neopdf::gridpdf::ForcePositive;
use neopdf::pdf::PDF;

use super::gridpdf::PySubGrid;
use super::metadata::PyMetaData;

// Type aliases
type LazyType = Result<PDF, Box<dyn std::error::Error>>;

/// Python wrapper for the `ForcePositive` enum.
#[pyclass(name = "ForcePositive")]
#[derive(Clone)]
pub enum PyForcePositive {
    /// If the calculated PDF value is negative, it is forced to 0.
    ClipNegative,
    /// If the calculated PDF value is less than 1e-10, it is set to 1e-10.
    ClipSmall,
    /// No clipping is done, value is returned as it is.
    NoClipping,
}

impl From<PyForcePositive> for ForcePositive {
    fn from(fmt: PyForcePositive) -> Self {
        match fmt {
            PyForcePositive::ClipNegative => Self::ClipNegative,
            PyForcePositive::ClipSmall => Self::ClipSmall,
            PyForcePositive::NoClipping => Self::NoClipping,
        }
    }
}

impl From<&ForcePositive> for PyForcePositive {
    fn from(fmt: &ForcePositive) -> Self {
        match fmt {
            ForcePositive::ClipNegative => Self::ClipNegative,
            ForcePositive::ClipSmall => Self::ClipSmall,
            ForcePositive::NoClipping => Self::NoClipping,
        }
    }
}

/// Methods to load all the PDF members for a given set.
#[pyclass(name = "LoaderMethod")]
#[derive(Clone)]
pub enum PyLoaderMethod {
    /// Load the members in parallel using multi-threads.
    Parallel,
    /// Load the members in sequential.
    Sequential,
}

#[pymethods]
impl PyForcePositive {
    fn __eq__(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    fn __hash__(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        std::mem::discriminant(self).hash(&mut hasher);
        hasher.finish()
    }
}

/// This enum contains the different parameters that a grid can depend on.
#[pyclass(name = "GridParams")]
#[derive(Clone)]
pub enum PyGridParams {
    /// The nucleon mass number A.
    A,
    /// The strong coupling `alpha_s`.
    AlphaS,
    /// The momentum fraction.
    X,
    /// The transverse momentum.
    KT,
    /// The energy scale `Q^2`.
    Q2,
}

/// Python wrapper for the `neopdf::pdf::PDF` struct.
///
/// This class provides a Python-friendly interface to the core PDF
/// interpolation functionalities of the `neopdf` Rust library.
#[pyclass(name = "LazyPDFs")]
pub struct PyLazyPDFs {
    iter: Mutex<Box<dyn Iterator<Item = LazyType> + Send>>,
}

#[pymethods]
impl PyLazyPDFs {
    const fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[allow(clippy::needless_pass_by_value)]
    fn __next__(slf: PyRefMut<'_, Self>) -> PyResult<Option<PyPDF>> {
        let mut iter = slf.iter.lock().unwrap();
        match iter.next() {
            Some(Ok(pdf)) => Ok(Some(PyPDF { pdf })),
            Some(Err(e)) => Err(pyo3::exceptions::PyValueError::new_err(e.to_string())),
            None => Ok(None),
        }
    }
}

/// Python wrapper for the `neopdf::pdf::PDF` struct.
///
/// This class provides a Python-friendly interface to the core PDF
/// interpolation functionalities of the `neopdf` Rust library.
#[pyclass(name = "PDF")]
#[repr(transparent)]
pub struct PyPDF {
    pub(crate) pdf: PDF,
}

#[pymethods]
#[allow(clippy::doc_markdown)]
impl PyPDF {
    /// Creates a new `PDF` instance for a given PDF set and member.
    ///
    /// This is the primary constructor for the `PDF` class.
    ///
    /// Parameters
    /// ----------
    /// pdf_name : str
    ///     The name of the PDF set.
    /// member : int
    ///     The ID of the PDF member to load. Defaults to 0.
    ///
    /// Returns
    /// -------
    /// PDF
    ///     A new `PDF` instance.
    #[new]
    #[must_use]
    #[pyo3(signature = (pdf_name, member = 0))]
    pub fn new(pdf_name: &str, member: usize) -> Self {
        Self {
            pdf: PDF::load(pdf_name, member),
        }
    }

    /// Loads a given member of the PDF set.
    ///
    /// This is an alternative constructor for convenience, equivalent
    /// to `PDF(pdf_name, member)`.
    ///
    /// Parameters
    /// ----------
    /// pdf_name : str
    ///     The name of the PDF set.
    /// member : int
    ///     The ID of the PDF member. Defaults to 0.
    ///
    /// Returns
    /// -------
    /// PDF
    ///     A new `PDF` instance.
    #[must_use]
    #[staticmethod]
    #[pyo3(name = "mkPDF")]
    #[pyo3(signature = (pdf_name, member = 0))]
    pub fn mkpdf(pdf_name: &str, member: usize) -> Self {
        Self::new(pdf_name, member)
    }

    /// Loads all members of the PDF set.
    ///
    /// This function loads all available members for a given PDF set,
    /// returning a list of `PDF` instances.
    ///
    /// Parameters
    /// ----------
    /// pdf_name : str
    ///     The name of the PDF set.
    ///
    /// Returns
    /// -------
    /// list[PDF]
    ///     A list of `PDF` instances, one for each member.
    #[must_use]
    #[staticmethod]
    #[pyo3(name = "mkPDFs")]
    #[pyo3(signature = (pdf_name, method = &PyLoaderMethod::Parallel))]
    pub fn mkpdfs(pdf_name: &str, method: &PyLoaderMethod) -> Vec<Self> {
        let loader_method = match method {
            PyLoaderMethod::Parallel => PDF::load_pdfs,
            PyLoaderMethod::Sequential => PDF::load_pdfs_seq,
        };

        loader_method(pdf_name)
            .into_iter()
            .map(move |pdfobj| Self { pdf: pdfobj })
            .collect()
    }

    /// Creates an iterator that loads PDF members lazily.
    ///
    /// This function is suitable for `.neopdf.lz4` files, which support lazy loading.
    /// It returns an iterator that yields `PDF` instances on demand, which is useful
    /// for reducing memory consumption when working with large PDF sets.
    ///
    /// # Arguments
    ///
    /// * `pdf_name` - The name of the PDF set (must end with `.neopdf.lz4`).
    ///
    /// # Returns
    ///
    /// An iterator over `Result<PDF, Box<dyn std::error::Error>>`.
    #[must_use]
    #[staticmethod]
    #[pyo3(name = "mkPDFs_lazy")]
    pub fn mkpdfs_lazy(pdf_name: &str) -> PyLazyPDFs {
        PyLazyPDFs {
            iter: Mutex::new(Box::new(PDF::load_pdfs_lazy(pdf_name))),
        }
    }

    /// Returns the list of `PID` values.
    ///
    /// Returns
    /// -------
    /// list[int]
    ///     The PID values.
    #[must_use]
    pub fn pids(&self) -> Vec<i32> {
        self.pdf.pids().to_vec()
    }

    /// Returns the list of `Subgrid` objects.
    ///
    /// Returns
    /// -------
    /// list[PySubgrid]
    ///     The subgrids.
    #[must_use]
    pub fn subgrids(&self) -> Vec<PySubGrid> {
        self.pdf
            .subgrids()
            .iter()
            .map(|subgrid| PySubGrid {
                subgrid: subgrid.clone(),
            })
            .collect()
    }

    /// Returns the subgrid knots of a parameter for a given subgrid index.
    ///
    /// The parameter could be the nucleon numbers `A`, the strong coupling
    /// `alphas`, the momentum fraction `x`, or the momentum scale `Q2`.
    ///
    /// # Panics
    ///
    /// This panics if the parameter is not valid.
    ///
    /// Returns
    /// -------
    /// list[float]
    ///     The subgrid knots for a given parameter.
    #[must_use]
    pub fn subgrid_knots(&self, param: &PyGridParams, subgrid_index: usize) -> Vec<f64> {
        match param {
            PyGridParams::AlphaS => self.pdf.subgrid(subgrid_index).alphas.to_vec(),
            PyGridParams::X => self.pdf.subgrid(subgrid_index).xs.to_vec(),
            PyGridParams::Q2 => self.pdf.subgrid(subgrid_index).q2s.to_vec(),
            PyGridParams::A => self.pdf.subgrid(subgrid_index).nucleons.to_vec(),
            PyGridParams::KT => self.pdf.subgrid(subgrid_index).kts.to_vec(),
        }
    }

    /// Clip the negative or small values for the `PDF` object.
    ///
    /// Parameters
    /// ----------
    /// id : PyFrocePositive
    ///     The clipping method use to handle negative or small values.
    pub fn set_force_positive(&mut self, option: PyForcePositive) {
        self.pdf.set_force_positive(option.into());
    }

    /// Clip the negative or small values for all the `PDF` objects.
    ///
    /// Parameters
    /// ----------
    /// pdfs : list[PDF]
    ///     A list of `PDF` instances.
    /// option : PyForcePositive
    ///     The clipping method use to handle negative or small values.
    #[staticmethod]
    #[pyo3(name = "set_force_positive_members")]
    #[allow(clippy::needless_pass_by_value)]
    pub fn set_force_positive_members(pdfs: Vec<PyRefMut<Self>>, option: PyForcePositive) {
        for mut pypdf in pdfs {
            pypdf.set_force_positive(option.clone());
        }
    }

    /// Returns the clipping method used for a single `PDF` object.
    ///
    /// Returns
    /// -------
    /// PyForcePositive
    ///     The clipping method used for the `PDF` object.
    #[must_use]
    pub fn is_force_positive(&self) -> PyForcePositive {
        self.pdf.is_force_positive().into()
    }

    /// Retrieves the minimum x-value for this PDF set.
    ///
    /// Returns
    /// -------
    /// float
    ///     The minimum x-value.
    #[must_use]
    pub fn x_min(&self) -> f64 {
        self.pdf.param_ranges().x.min
    }

    /// Retrieves the maximum x-value for this PDF set.
    ///
    /// Returns
    /// -------
    /// float
    ///     The maximum x-value.
    #[must_use]
    pub fn x_max(&self) -> f64 {
        self.pdf.param_ranges().x.max
    }

    /// Retrieves the minimum Q2-value for this PDF set.
    ///
    /// Returns
    /// -------
    /// float
    ///     The minimum Q2-value.
    #[must_use]
    pub fn q2_min(&self) -> f64 {
        self.pdf.param_ranges().q2.min
    }

    /// Retrieves the maximum Q2-value for this PDF set.
    ///
    /// Returns
    /// -------
    /// float
    ///     The maximum Q2-value.
    #[must_use]
    pub fn q2_max(&self) -> f64 {
        self.pdf.param_ranges().q2.max
    }

    /// Retrieves the flavour PIDs for the PDF set.
    ///
    /// Returns
    /// -------
    /// list(int)
    ///     The flavour PID values.
    #[must_use]
    pub fn flavour_pids(&self) -> Vec<i32> {
        self.pdf.metadata().flavors.clone()
    }

    /// Interpolates the PDF value (xf) for a given flavor, x, and Q2.
    ///
    /// Parameters
    /// ----------
    /// id : int
    ///     The flavor ID (e.g., 21 for gluon, 1 for d-quark).
    /// x : float
    ///     The momentum fraction.
    /// q2 : float
    ///     The energy scale squared.
    ///
    /// Returns
    /// -------
    /// float
    ///     The interpolated PDF value. Returns 0.0 if extrapolation is
    ///     attempted and not allowed.
    #[must_use]
    #[pyo3(name = "xfxQ2")]
    pub fn xfxq2(&self, id: i32, x: f64, q2: f64) -> f64 {
        self.pdf.xfxq2(id, &[x, q2])
    }

    /// Interpolates the PDF value (xf) for a given set of parameters.
    ///
    /// Parameters
    /// ----------
    /// id : int
    ///     The flavor ID (e.g., 21 for gluon, 1 for d-quark).
    /// params: list[float]
    ///     A list of parameters that the grids depends on. If the PDF
    ///     grid only contains `x` and `Q2` dependence then its value is
    ///     `[x, q2]`; if it contains either the `A` and `alpha_s`
    ///     dependence, then its value is `[A, x, q2]` or `[alpha_s, x, q2]`
    ///     respectively; if it contains both, then `[A, alpha_s, x, q2]`.
    ///
    /// Returns
    /// -------
    /// float
    ///     The interpolated PDF value. Returns 0.0 if extrapolation is
    ///     attempted and not allowed.
    #[must_use]
    #[pyo3(name = "xfxQ2_ND")]
    #[allow(clippy::needless_pass_by_value)]
    pub fn xfxq2_nd(&self, id: i32, params: Vec<f64>) -> f64 {
        self.pdf.xfxq2(id, &params)
    }

    /// Interpolates the PDF value (xf) for a list containg a set of parameters.
    ///
    /// Parameters
    /// ----------
    /// id : int
    ///     The flavor ID (e.g., 21 for gluon, 1 for d-quark).
    /// params: list[list[float]]
    ///     A list containing the list of points. Each element in the list
    ///     is in turn a list containing the parameters that the grids depends
    ///     on. If the PDF grid only contains `x` and `Q2` dependence then its
    ///     value is `[x, q2]`; if it contains either the `A` and `alpha_s`
    ///     dependence, then its value is `[A, x, q2]` or `[alpha_s, x, q2]`
    ///     respectively; if it contains both, then `[A, alpha_s, x, q2]`.
    ///
    /// Returns
    /// -------
    /// float
    ///     The interpolated PDF value. Returns 0.0 if extrapolation is
    ///     attempted and not allowed.
    #[must_use]
    #[pyo3(name = "xfxQ2_Chebyshev_batch")]
    #[allow(clippy::needless_pass_by_value)]
    pub fn xfxq2_cheby_batch(&self, id: i32, params: Vec<Vec<f64>>) -> Vec<f64> {
        let slices: Vec<&[f64]> = params.iter().map(Vec::as_slice).collect();
        self.pdf.xfxq2_cheby_batch(id, &slices)
    }

    /// Interpolates the PDF value (xf) for lists of flavors, x-values,
    /// and Q2-values.
    ///
    /// Parameters
    /// ----------
    /// id : list[int]
    ///     A list of flavor IDs.
    /// xs : list[float]
    ///     A list of momentum fractions.
    /// q2s : list[float]
    ///     A list of energy scales squared.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     A 2D NumPy array containing the interpolated PDF values.
    #[must_use]
    #[pyo3(name = "xfxQ2s")]
    #[allow(clippy::needless_pass_by_value)]
    pub fn xfxq2s<'py>(
        &self,
        pids: Vec<i32>,
        xs: Vec<f64>,
        q2s: Vec<f64>,
        py: Python<'py>,
    ) -> Bound<'py, PyArray2<f64>> {
        let flatten_points: Vec<Vec<f64>> = xs
            .iter()
            .flat_map(|&x| q2s.iter().map(move |&q2| vec![x, q2]))
            .collect();
        let points_interp: Vec<&[f64]> = flatten_points.iter().map(Vec::as_slice).collect();
        let slice_points: &[&[f64]] = &points_interp;

        self.pdf.xfxq2s(pids, slice_points).into_pyarray(py)
    }

    /// Computes the alpha_s value at a given Q2.
    ///
    /// Parameters
    /// ----------
    /// q2 : float
    ///     The energy scale squared.
    ///
    /// Returns
    /// -------
    /// float
    ///     The interpolated alpha_s value.
    #[must_use]
    #[pyo3(name = "alphasQ2")]
    pub fn alphas_q2(&self, q2: f64) -> f64 {
        self.pdf.alphas_q2(q2)
    }

    /// Returns the metadata associated with this PDF set.
    ///
    /// Provides access to the metadata describing the PDF set, including information
    /// such as the set description, number of members, parameter ranges, and other
    /// relevant details.
    ///
    /// Returns
    /// -------
    /// MetaData
    ///     The metadata for this PDF set as a `MetaData` Python object.
    #[must_use]
    #[pyo3(name = "metadata")]
    pub fn metadata(&self) -> PyMetaData {
        PyMetaData {
            meta: self.pdf.metadata().clone(),
        }
    }
}

/// Registers the `pdf` submodule with the parent Python module.
///
/// This function is typically called during the initialization of the
/// `neopdf` Python package to expose the `PDF` class.
///
/// Parameters
/// ----------
/// `parent_module` : pyo3.Bound[pyo3.types.PyModule]
///     The parent Python module to which the `pdf` submodule will be added.
///
/// Returns
/// -------
/// pyo3.PyResult<()>
///     `Ok(())` if the registration is successful, or an error if the submodule
///     cannot be created or added.
///
/// # Errors
///
/// Raises an error if the (sub)module is not found or cannot be registered.
pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent_module.py(), "pdf")?;
    m.setattr(pyo3::intern!(m.py(), "__doc__"), "Interface for PDF.")?;
    pyo3::py_run!(
        parent_module.py(),
        m,
        "import sys; sys.modules['neopdf.pdf'] = m"
    );
    m.add_class::<PyPDF>()?;
    m.add_class::<PyLazyPDFs>()?;
    m.add_class::<PyForcePositive>()?;
    m.add_class::<PyGridParams>()?;
    m.add_class::<PyLoaderMethod>()?;
    parent_module.add_submodule(&m)
}
