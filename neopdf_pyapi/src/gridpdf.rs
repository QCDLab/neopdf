use ndarray::{Array1, Dimension};
use numpy::{PyArrayMethods, PyReadonlyArrayDyn};
use pyo3::prelude::*;

use neopdf::gridpdf::GridArray;
use neopdf::subgrid::{GridData, ParamRange, SubGrid};

/// Python wrapper for the `SubGrid` struct.
#[pyclass(name = "SubGrid")]
pub struct PySubGrid {
    pub(crate) subgrid: SubGrid,
}

#[pymethods]
impl PySubGrid {
    /// Constructs a new `SubGrid` instance from the provided axes and grid data.
    ///
    /// # Parameters
    ///
    /// - `xs`: The x-axis values.
    /// - `q2s`: The Q^2-axis values.
    /// - `kts`: The kT-axis values.
    /// - `xsis`: The Skeweness `\xi`.
    /// - `deltas`: The total momentum fraction `\Delta`.
    /// - `nucleons`: The nucleon number axis values.
    /// - `alphas`: The `alpha_s` axis values.
    /// - `grid`: The 6D grid data as a `NumPy` array.
    ///
    /// # Returns
    ///
    /// Returns a new `PySubGrid` instance.
    ///
    /// # Panics
    ///
    /// Panics if any of the input vectors are empty.
    ///
    /// # Errors
    ///
    /// Returns a `PyErr` if the grid cannot be constructed from the input data.
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(
        xs: Vec<f64>,
        q2s: Vec<f64>,
        kts: Vec<f64>,
        xsis: Vec<f64>,
        deltas: Vec<f64>,
        nucleons: Vec<f64>,
        alphas: Vec<f64>,
        grid: PyReadonlyArrayDyn<f64>,
    ) -> PyResult<Self> {
        let alphas_range = ParamRange::new(*alphas.first().unwrap(), *alphas.last().unwrap());
        let x_range = ParamRange::new(*xs.first().unwrap(), *xs.last().unwrap());
        let q2_range = ParamRange::new(*q2s.first().unwrap(), *q2s.last().unwrap());
        let xsi_range = ParamRange::new(*xsis.first().unwrap(), *xsis.last().unwrap());
        let delta_range = ParamRange::new(*deltas.first().unwrap(), *deltas.last().unwrap());
        let kt_range = ParamRange::new(*kts.first().unwrap(), *kts.last().unwrap());
        let nucleons_range = ParamRange::new(*nucleons.first().unwrap(), *nucleons.last().unwrap());

        let subgrid = SubGrid {
            xs: Array1::from(xs),
            q2s: Array1::from(q2s),
            kts: Array1::from(kts),
            xis: Array1::from(xsis),
            deltas: Array1::from(deltas),
            grid: GridData::Grid8D(grid.to_owned_array()),
            nucleons: Array1::from(nucleons),
            alphas: Array1::from(alphas),
            nucleons_range,
            alphas_range,
            xi_range: xsi_range,
            delta_range,
            kt_range,
            x_range,
            q2_range,
        };

        Ok(Self { subgrid })
    }

    /// Returns the minimum and maximum values of the `alpha_s` axis.
    #[must_use]
    pub const fn alphas_range(&self) -> (f64, f64) {
        (self.subgrid.alphas_range.min, self.subgrid.alphas_range.max)
    }

    /// Returns the minimum and maximum values of the momentum fraction `x`.
    #[must_use]
    pub const fn x_range(&self) -> (f64, f64) {
        (self.subgrid.x_range.min, self.subgrid.x_range.max)
    }

    /// Returns the minimum and maximum values of the momentum scale `Q^2`.
    #[must_use]
    pub const fn q2_range(&self) -> (f64, f64) {
        (self.subgrid.q2_range.min, self.subgrid.q2_range.max)
    }

    /// Returns the minimum and maximum values of the skeweness `xi`.
    #[must_use]
    pub const fn xi_range(&self) -> (f64, f64) {
        (self.subgrid.xi_range.min, self.subgrid.xi_range.max)
    }

    /// Returns the minimum and maximum values of the total momentum fraction `delta`.
    #[must_use]
    pub const fn delta_range(&self) -> (f64, f64) {
        (self.subgrid.delta_range.min, self.subgrid.delta_range.max)
    }

    /// Returns the minimum and maximum values of the Nucleon number `A`.
    #[must_use]
    pub const fn nucleons_range(&self) -> (f64, f64) {
        (
            self.subgrid.nucleons_range.min,
            self.subgrid.nucleons_range.max,
        )
    }

    /// Returns the minimum and maximum values of the transverse momentum `kT`.
    #[must_use]
    pub const fn kt_range(&self) -> (f64, f64) {
        (self.subgrid.kt_range.min, self.subgrid.kt_range.max)
    }

    /// Returns the shape of the subgrid
    ///
    /// # Panics
    ///
    /// TODO
    #[must_use]
    pub fn grid_shape(&self) -> Vec<usize> {
        match &self.subgrid.grid {
            GridData::Grid6D(grid) => {
                let (d0, d1, d2, d3, d4, d5) = grid.dim();
                vec![d0, d1, d2, d3, d4, d5]
            }
            GridData::Grid8D(grid) => grid.dim().as_array_view().to_vec(),
        }
    }
}

/// Python wrapper for the `GridArray` struct.
#[pyclass(name = "GridArray")]
#[repr(transparent)]
pub struct PyGridArray {
    pub(crate) gridarray: GridArray,
}

#[pymethods]
impl PyGridArray {
    /// Constructs a new `GridArray` from a list of particle IDs and subgrids.
    ///
    /// # Parameters
    ///
    /// - `pids`: The list of particle IDs.
    /// - `subgrids`: The list of subgrid objects.
    ///
    /// # Returns
    ///
    /// Returns a new `PyGridArray` instance.
    #[new]
    #[must_use]
    pub fn new(pids: Vec<i32>, subgrids: Vec<PyRef<PySubGrid>>) -> Self {
        let subgrids = subgrids
            .into_iter()
            .map(|py_ref| py_ref.subgrid.clone())
            .collect();

        let gridarray = GridArray {
            pids: Array1::from(pids),
            subgrids,
        };
        Self { gridarray }
    }

    /// Returns the particle IDs associated with this grid array.
    #[must_use]
    pub fn pids(&self) -> Vec<i32> {
        self.gridarray.pids.to_vec()
    }

    /// Returns the subgrids contained in this grid array.
    #[must_use]
    pub fn subgrids(&self) -> Vec<PySubGrid> {
        self.gridarray
            .subgrids
            .iter()
            .cloned()
            .map(|sg| PySubGrid { subgrid: sg })
            .collect()
    }
}

/// Registers the gridpdf module with the parent Python module.
///
/// Adds the `gridpdf` submodule to the parent Python module, exposing grid
/// interpolation utilities to Python.
///
/// # Errors
///
/// Returns a `PyErr` if the submodule cannot be created or added, or if any
/// class registration fails.
pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent_module.py(), "gridpdf")?;
    m.setattr(
        pyo3::intern!(m.py(), "__doc__"),
        "GridPDF interpolation interface.",
    )?;
    pyo3::py_run!(
        parent_module.py(),
        m,
        "import sys; sys.modules['neopdf.gridpdf'] = m"
    );
    m.add_class::<PySubGrid>()?;
    m.add_class::<PyGridArray>()?;
    parent_module.add_submodule(&m)
}
