use pyo3::prelude::*;

use neopdf::metadata::{InterpolatorType, MetaData, SetType};

/// The type of the set.
#[pyclass(eq, eq_int, name = "SetType")]
#[derive(Clone, PartialEq, Eq)]
pub enum PySetType {
    /// Parton Distribution Function.
    SpaceLike,
    /// Fragmentation Function.
    TimeLike,
}

impl From<&SetType> for PySetType {
    fn from(set_type: &SetType) -> Self {
        match set_type {
            SetType::SpaceLike => Self::SpaceLike,
            SetType::TimeLike => Self::TimeLike,
        }
    }
}

impl From<&PySetType> for SetType {
    fn from(set_type: &PySetType) -> Self {
        match set_type {
            PySetType::SpaceLike => Self::SpaceLike,
            PySetType::TimeLike => Self::TimeLike,
        }
    }
}

/// The interpolation method used for the grid.
#[pyclass(eq, eq_int, name = "InterpolatorType")]
#[derive(Clone, PartialEq, Eq)]
pub enum PyInterpolatorType {
    /// Bilinear interpolation strategy.
    Bilinear,
    /// Bilinear logarithmic interpolation strategy.
    LogBilinear,
    /// Bicubic logarithmic interpolation strategy.
    LogBicubic,
    /// Tricubic logarithmic interpolation strategy.
    LogTricubic,
    /// Linear interpolation for N-dimensional data.
    NDLinear,
    /// Chebyshev logarithmic interpolation strategy.
    LogChebyshev,
    /// Four-dimensional cubic logarithmic interpolation strategy.
    LogFourCubic,
}

impl From<&InterpolatorType> for PyInterpolatorType {
    fn from(basis: &InterpolatorType) -> Self {
        match basis {
            InterpolatorType::Bilinear => Self::Bilinear,
            InterpolatorType::LogBilinear => Self::LogBilinear,
            InterpolatorType::LogBicubic => Self::LogBicubic,
            InterpolatorType::LogTricubic => Self::LogTricubic,
            InterpolatorType::InterpNDLinear => Self::NDLinear,
            InterpolatorType::LogChebyshev => Self::LogChebyshev,
            InterpolatorType::LogFourCubic => Self::LogFourCubic,
        }
    }
}

impl From<&PyInterpolatorType> for InterpolatorType {
    fn from(basis: &PyInterpolatorType) -> Self {
        match basis {
            PyInterpolatorType::Bilinear => Self::Bilinear,
            PyInterpolatorType::LogBilinear => Self::LogBilinear,
            PyInterpolatorType::LogBicubic => Self::LogBicubic,
            PyInterpolatorType::LogTricubic => Self::LogTricubic,
            PyInterpolatorType::NDLinear => Self::InterpNDLinear,
            PyInterpolatorType::LogChebyshev => Self::LogChebyshev,
            PyInterpolatorType::LogFourCubic => Self::LogFourCubic,
        }
    }
}

/// Physical Parameters of the PDF set.
#[pyclass(name = "PhysicsParameters")]
#[derive(Debug, Clone)]
pub struct PyPhysicsParameters {
    pub(crate) flavor_scheme: String,
    pub(crate) order_qcd: u32,
    pub(crate) alphas_order_qcd: u32,
    pub(crate) m_w: f64,
    pub(crate) m_z: f64,
    pub(crate) m_up: f64,
    pub(crate) m_down: f64,
    pub(crate) m_strange: f64,
    pub(crate) m_charm: f64,
    pub(crate) m_bottom: f64,
    pub(crate) m_top: f64,
    pub(crate) alphas_type: String,
    pub(crate) number_flavors: u32,
}

#[pymethods]
impl PyPhysicsParameters {
    /// Constructor for PyPhysicsParameters.
    #[new]
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        flavor_scheme = "None".to_string(),
        order_qcd = 0,
        alphas_order_qcd = 0,
        m_w = 0.0,
        m_z = 0.0,
        m_up = 0.0,
        m_down = 0.0,
        m_strange = 0.0,
        m_charm = 0.0,
        m_bottom = 0.0,
        m_top = 0.0,
        alphas_type = "None".to_string(),
        number_flavors = 0,
    ))]
    pub const fn new(
        flavor_scheme: String,
        order_qcd: u32,
        alphas_order_qcd: u32,
        m_w: f64,
        m_z: f64,
        m_up: f64,
        m_down: f64,
        m_strange: f64,
        m_charm: f64,
        m_bottom: f64,
        m_top: f64,
        alphas_type: String,
        number_flavors: u32,
    ) -> Self {
        Self {
            flavor_scheme,
            order_qcd,
            alphas_order_qcd,
            m_w,
            m_z,
            m_up,
            m_down,
            m_strange,
            m_charm,
            m_bottom,
            m_top,
            alphas_type,
            number_flavors,
        }
    }

    /// Convert to Python dictionary.
    ///
    /// # Errors
    ///
    /// Raises an error if the values are not Python compatible.
    pub fn to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item("flavor_scheme", &self.flavor_scheme)?;
        dict.set_item("order_qcd", self.order_qcd)?;
        dict.set_item("alphas_order_qcd", self.alphas_order_qcd)?;
        dict.set_item("m_w", self.m_w)?;
        dict.set_item("m_z", self.m_z)?;
        dict.set_item("m_up", self.m_up)?;
        dict.set_item("m_down", self.m_down)?;
        dict.set_item("m_strange", self.m_strange)?;
        dict.set_item("m_charm", self.m_charm)?;
        dict.set_item("m_bottom", self.m_bottom)?;
        dict.set_item("m_top", self.m_top)?;

        Ok(dict.into())
    }
}

impl Default for PyPhysicsParameters {
    fn default() -> Self {
        Self {
            flavor_scheme: String::new(),
            order_qcd: 0,
            alphas_order_qcd: 0,
            m_w: 0.0,
            m_z: 0.0,
            m_up: 0.0,
            m_down: 0.0,
            m_strange: 0.0,
            m_charm: 0.0,
            m_bottom: 0.0,
            m_top: 0.0,
            alphas_type: String::new(),
            number_flavors: 0,
        }
    }
}

/// Grid metadata.
#[pyclass(name = "MetaData")]
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct PyMetaData {
    pub(crate) meta: MetaData,
}

#[pymethods]
impl PyMetaData {
    /// Constructor for PyMetaData.
    #[new]
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        set_desc,
        set_index,
        num_members,
        x_min,
        x_max,
        q_min,
        q_max,
        flavors,
        format,
        alphas_q_values = vec![],
        alphas_vals = vec![],
        polarised = false,
        set_type = PySetType::SpaceLike,
        interpolator_type = PyInterpolatorType::LogBicubic,
        error_type = "replicas".to_string(),
        hadron_pid = 2212,
        phys_params = PyPhysicsParameters::default(),
    ))]
    pub fn new(
        set_desc: String,
        set_index: u32,
        num_members: u32,
        x_min: f64,
        x_max: f64,
        q_min: f64,
        q_max: f64,
        flavors: Vec<i32>,
        format: String,
        alphas_q_values: Vec<f64>,
        alphas_vals: Vec<f64>,
        polarised: bool,
        set_type: PySetType,
        interpolator_type: PyInterpolatorType,
        error_type: String,
        hadron_pid: i32,
        phys_params: PyPhysicsParameters,
    ) -> Self {
        Self {
            meta: MetaData {
                set_desc,
                set_index,
                num_members,
                x_min,
                x_max,
                q_min,
                q_max,
                flavors,
                format,
                alphas_q_values,
                alphas_vals,
                polarised,
                set_type: SetType::from(&set_type),
                interpolator_type: InterpolatorType::from(&interpolator_type),
                error_type,
                hadron_pid,
                git_version: String::new(), // placeholder to be overwritten
                code_version: String::new(), // placeholder to be overwritten
                flavor_scheme: phys_params.flavor_scheme,
                order_qcd: phys_params.order_qcd,
                alphas_order_qcd: phys_params.alphas_order_qcd,
                m_w: phys_params.m_w,
                m_z: phys_params.m_z,
                m_up: phys_params.m_up,
                m_down: phys_params.m_down,
                m_strange: phys_params.m_strange,
                m_charm: phys_params.m_charm,
                m_bottom: phys_params.m_bottom,
                m_top: phys_params.m_top,
                alphas_type: phys_params.alphas_type,
                number_flavors: phys_params.number_flavors,
                // New V2 fields with defaults
                xi_min: 1.0,
                xi_max: 1.0,
                delta_min: 0.0,
                delta_max: 0.0,
            },
        }
    }

    /// Convert to Python dictionary
    ///
    /// # Errors
    ///
    /// Raises an erro if the values are not Python compatible.
    pub fn to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new(py);

        let set_type = match &self.meta.set_type {
            SetType::SpaceLike => "PDF",
            SetType::TimeLike => "FragFn",
        };

        let interpolator_type = match &self.meta.interpolator_type {
            InterpolatorType::Bilinear => "Bilinear",
            InterpolatorType::LogBilinear => "LogBilinear",
            InterpolatorType::LogBicubic => "LogBicubic",
            InterpolatorType::LogTricubic => "LogTricubic",
            InterpolatorType::InterpNDLinear => "NDLinear",
            InterpolatorType::LogChebyshev => "LogChebyshev",
            InterpolatorType::LogFourCubic => "LogFourCubic",
        };

        dict.set_item("set_desc", &self.meta.set_desc)?;
        dict.set_item("set_index", self.meta.set_index)?;
        dict.set_item("num_members", self.meta.num_members)?;
        dict.set_item("x_min", self.meta.x_min)?;
        dict.set_item("x_max", self.meta.x_max)?;
        dict.set_item("q_min", self.meta.q_min)?;
        dict.set_item("q_max", self.meta.q_max)?;
        dict.set_item("flavors", &self.meta.flavors)?;
        dict.set_item("format", &self.meta.format)?;
        dict.set_item("alphas_q_values", &self.meta.alphas_q_values)?;
        dict.set_item("alphas_vals", &self.meta.alphas_vals)?;
        dict.set_item("polarised", self.meta.polarised)?;
        dict.set_item("set_type", set_type)?;
        dict.set_item("interpolator_type", interpolator_type)?;
        dict.set_item("error_type", &self.meta.error_type)?;
        dict.set_item("hadron_pid", self.meta.hadron_pid)?;
        dict.set_item("git_version", &self.meta.git_version)?;
        dict.set_item("code_version", &self.meta.code_version)?;
        dict.set_item("flavor_scheme", &self.meta.flavor_scheme)?;
        dict.set_item("order_qcd", self.meta.order_qcd)?;
        dict.set_item("alphas_order_qcd", self.meta.alphas_order_qcd)?;
        dict.set_item("m_w", self.meta.m_w)?;
        dict.set_item("m_z", self.meta.m_z)?;
        dict.set_item("m_up", self.meta.m_up)?;
        dict.set_item("m_down", self.meta.m_down)?;
        dict.set_item("m_strange", self.meta.m_strange)?;
        dict.set_item("m_charm", self.meta.m_charm)?;
        dict.set_item("m_bottom", self.meta.m_bottom)?;
        dict.set_item("m_top", self.meta.m_top)?;

        Ok(dict.into())
    }

    /// The description of the set.
    #[must_use]
    pub fn set_desc(&self) -> &String {
        &self.meta.set_desc
    }

    /// The index of the grid.
    #[must_use]
    pub fn set_index(&self) -> u32 {
        self.meta.set_index
    }

    /// The number of sets in the grid.
    #[must_use]
    pub fn number_sets(&self) -> u32 {
        self.meta.num_members
    }

    /// The minimum value of `x` in the grid.
    #[must_use]
    pub fn x_min(&self) -> f64 {
        self.meta.x_min
    }

    /// The maximum value of `x` in the grid.
    #[must_use]
    pub fn x_max(&self) -> f64 {
        self.meta.x_max
    }

    /// The minimum value of `q` in the grid.
    #[must_use]
    pub fn q_min(&self) -> f64 {
        self.meta.q_min
    }

    /// The maximum value of `q` in the grid.
    #[must_use]
    pub fn q_max(&self) -> f64 {
        self.meta.q_max
    }

    /// The particle IDs of the grid.
    #[must_use]
    pub fn pids(&self) -> &Vec<i32> {
        &self.meta.flavors
    }

    /// The format of the grid.
    #[must_use]
    pub fn format(&self) -> &String {
        &self.meta.format
    }

    /// The values of `q` for the running of the strong coupling constant.
    #[must_use]
    pub fn alphas_q(&self) -> &Vec<f64> {
        &self.meta.alphas_q_values
    }

    /// The values of the running of the strong coupling constant.
    #[must_use]
    pub fn alphas_values(&self) -> &Vec<f64> {
        &self.meta.alphas_vals
    }

    /// Whether the grid is polarised.
    #[must_use]
    pub fn is_polarised(&self) -> bool {
        self.meta.polarised
    }

    /// The type of the set.
    #[must_use]
    pub fn set_type(&self) -> PySetType {
        PySetType::from(&self.meta.set_type)
    }

    /// The interpolation method used for the grid.
    #[must_use]
    pub fn interpolator_type(&self) -> PyInterpolatorType {
        PyInterpolatorType::from(&self.meta.interpolator_type)
    }

    /// The type of error.
    #[must_use]
    pub fn error_type(&self) -> &String {
        &self.meta.error_type
    }

    /// The hadron PID.
    #[must_use]
    pub fn hadron_pid(&self) -> i32 {
        self.meta.hadron_pid
    }
}

/// Registers the `metadata` submodule with the parent Python module.
///
/// Parameters
/// ----------
/// `parent_module` : pyo3.Bound[pyo3.types.PyModule]
///     The parent Python module to which the `metadata` submodule will be added.
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
    let m = PyModule::new(parent_module.py(), "metadata")?;
    m.setattr(pyo3::intern!(m.py(), "__doc__"), "Interface for PDF.")?;
    pyo3::py_run!(
        parent_module.py(),
        m,
        "import sys; sys.modules['neopdf.metadata'] = m"
    );
    m.add_class::<PySetType>()?;
    m.add_class::<PyInterpolatorType>()?;
    m.add_class::<PyPhysicsParameters>()?;
    m.add_class::<PyMetaData>()?;
    parent_module.add_submodule(&m)
}
