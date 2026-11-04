//! The Python interface to device grids (feature `python`).
//!
//! `libneopdf_cuda` is the `neopdf_cuda` extension module. Host arrays are `NumPy`
//! arrays; device arrays are any object exposing `__cuda_array_interface__`
//! (`CuPy`, `PyTorch`, `Numba`, ...), whose memory is used in place.

use std::error::Error;
use std::iter::successors;

use cuda_core::PinnedHostBuffer;
use numpy::ndarray::{Array3, ArrayView3};
use numpy::{AllowTypeChange, IntoPyArray, PyArray3, PyArrayDyn, PyArrayLike1, PyArrayMethods};
use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PySlice};
use pyo3::{intern, PyErr};

use crate::device::DeviceError;
use crate::foreign::{CUstream, NeoPDFDeviceGrid, NeoPDFDeviceGridError};

/// The error and its sources, as one message; a source already quoted is skipped.
fn message(err: &(dyn Error + 'static)) -> String {
    successors(err.source(), |&err| err.source()).fold(err.to_string(), |message, source| {
        let source = source.to_string();
        if message.ends_with(&source) {
            message
        } else {
            format!("{message}: {source}")
        }
    })
}

impl From<DeviceError> for PyErr {
    fn from(err: DeviceError) -> Self {
        match err {
            DeviceError::Shape { .. } | DeviceError::TooLarge(_) => {
                PyValueError::new_err(message(&err))
            }
            _ => PyRuntimeError::new_err(message(&err)),
        }
    }
}

impl From<NeoPDFDeviceGridError> for PyErr {
    fn from(err: NeoPDFDeviceGridError) -> Self {
        match err {
            NeoPDFDeviceGridError::Layout(err) => PyValueError::new_err(message(&err)),
            NeoPDFDeviceGridError::Device(err) => err.into(),
        }
    }
}

/// A C-contiguous `float64` device array, read from its `__cuda_array_interface__`.
struct CudaArray {
    ptr: usize,
    len: usize,
    readonly: bool,
}

impl CudaArray {
    fn new(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        let interface = obj
            .getattr(intern!(obj.py(), "__cuda_array_interface__"))
            .map_err(|_| {
                PyTypeError::new_err("expected a device array exposing `__cuda_array_interface__`")
            })?;
        let interface = interface.cast_into::<PyDict>()?;
        let item = |key: &str| {
            interface.get_item(key)?.ok_or_else(|| {
                PyValueError::new_err(format!("`__cuda_array_interface__` lacks `{key}`"))
            })
        };
        let typestr: String = item("typestr")?.extract()?;
        if typestr != "<f8" {
            return Err(PyTypeError::new_err(format!(
                "expected a float64 device array, found `{typestr}`"
            )));
        }
        let shape: Vec<usize> = item("shape")?.extract()?;
        // C-contiguous arrays may omit their strides or set them to `None`.
        if let Some(strides) = interface.get_item("strides")?.filter(|s| !s.is_none()) {
            let strides: Vec<isize> = strides.extract()?;
            let mut stride = size_of::<f64>();
            for (&dim, &got) in shape.iter().zip(&strides).rev() {
                if dim > 1 && usize::try_from(got) != Ok(stride) {
                    return Err(PyValueError::new_err(
                        "expected a C-contiguous device array",
                    ));
                }
                stride *= dim;
            }
        }
        let (ptr, readonly) = item("data")?.extract()?;
        Ok(Self {
            ptr,
            len: shape.iter().product(),
            readonly,
        })
    }
}

/// Owner of the page-locked memory behind the arrays of `DeviceGrid.host_buffer`.
#[pyclass(frozen)]
struct PinnedMemory(#[allow(dead_code)] PinnedHostBuffer<f64>);

/// Every member of a PDF set, resident on a GPU.
///
/// The batch methods evaluate all members and flavors, plus `alpha_s`, at a batch
/// of `(x, Q2)` points, with an output laid out `[n_events][n_members][n_pids + 1]`.
#[pyclass(name = "DeviceGrid", frozen)]
pub struct PyDeviceGrid(NeoPDFDeviceGrid);

#[pymethods]
impl PyDeviceGrid {
    /// Loads every member of `pdf_name` and uploads it to GPU `device`.
    ///
    /// Raises `ValueError` if the set is not supported on the device, and
    /// `RuntimeError` if the device can't be used.
    #[new]
    #[pyo3(signature = (pdf_name, device = 0))]
    fn new(pdf_name: &str, device: usize) -> PyResult<Self> {
        Ok(Self(NeoPDFDeviceGrid::new(pdf_name, device)?))
    }

    /// Number of members on the device.
    const fn num_members(&self) -> usize {
        self.0.n_members()
    }

    /// Flavor PIDs, in output column order.
    fn pids(&self) -> Vec<i32> {
        self.0.pids().to_vec()
    }

    /// Length of the batch output for `n_events` events:
    /// `n_events * num_members() * (len(pids()) + 1)`.
    const fn output_len(&self, n_events: usize) -> usize {
        self.0.output_len(n_events)
    }

    /// Evaluates the host arrays `xs` and `q2s`, returning a `NumPy` array of shape
    /// `(n_events, num_members(), len(pids()) + 1)`.
    ///
    /// With `out`, a C-contiguous `float64` array of at least `output_len(n_events)`
    /// values, its first `output_len(n_events)` values are filled in place and returned
    /// with that shape. For large batches, an `out` from `host_buffer` is several times
    /// faster than a new array.
    #[pyo3(name = "xfxQ2_batch", signature = (xs, q2s, out = None))]
    #[allow(clippy::needless_pass_by_value)]
    fn xfxq2_batch<'py>(
        &self,
        py: Python<'py>,
        xs: PyArrayLike1<'py, f64, AllowTypeChange>,
        q2s: PyArrayLike1<'py, f64, AllowTypeChange>,
        out: Option<Bound<'py, PyArrayDyn<f64>>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let (xs, q2s) = (xs.as_slice()?, q2s.as_slice()?);
        let len = self.0.output_len(xs.len());
        let shape = (xs.len(), self.0.n_members(), self.0.pids().len() + 1);
        let Some(out) = out else {
            let mut out = vec![0.0; len];
            py.detach(|| self.0.xfxq2_batch_host(xs, q2s, &mut out))?;
            return Ok(Array3::from_shape_vec(shape, out)
                .expect("the output length matches its shape")
                .into_pyarray(py)
                .into_any());
        };
        {
            let mut values = out.readwrite();
            let values = values
                .as_slice_mut()
                .map_err(|_| PyValueError::new_err("`out` must be C-contiguous"))?;
            let available = values.len();
            let values = values.get_mut(..len).ok_or_else(|| {
                PyValueError::new_err(format!(
                    "`out` holds {available} values, the batch needs {len}"
                ))
            })?;
            py.detach(|| self.0.xfxq2_batch_host(xs, q2s, values))?;
        }
        out.call_method1(intern!(py, "reshape"), (-1,))?
            .get_item(PySlice::new(py, 0, len.try_into()?, 1))?
            .call_method1(intern!(py, "reshape"), (shape,))
    }

    /// A zeroed `NumPy` array of shape `(n_events, num_members(), len(pids()) + 1)` in
    /// page-locked host memory, for the `out` of `xfxQ2_batch`: the results are copied
    /// into it from the GPU several times faster. Allocating it is slow, so allocate it
    /// once and reuse it for batches of up to `n_events` events.
    fn host_buffer<'py>(&self, py: Python<'py>, n_events: usize) -> PyResult<Bound<'py, PyAny>> {
        let mut buffer = self.0.pinned_output(n_events)?;
        let ptr = buffer.as_mut_ptr();
        let shape = (n_events, self.0.n_members(), self.0.pids().len() + 1);
        // The heap allocation does not move with `buffer` into its owner.
        let owner = Bound::new(py, PinnedMemory(buffer))?;
        // SAFETY: `ptr` holds `output_len(n_events)` values, the product of `shape`,
        // and stays valid while `owner`, the base object of the array, is alive.
        let array = unsafe {
            let view = ArrayView3::from_shape_ptr(shape, ptr.cast_const());
            PyArray3::borrow_from_array(&view, owner.into_any())
        };
        Ok(array.into_any())
    }

    /// Enqueues the same evaluation on device arrays, writing into `out` in place
    /// and ordered on the CUDA `stream` (a raw `cudaStream_t` handle, `0` for the
    /// default stream), without synchronizing the host.
    ///
    /// `xs`, `q2s` and `out` are C-contiguous `float64` arrays on the grid's device
    /// exposing `__cuda_array_interface__`; `out` holds `output_len(len(xs))` values,
    /// in any shape.
    #[pyo3(name = "xfxQ2_batch_device", signature = (xs, q2s, out, stream = 0))]
    fn xfxq2_batch_device(
        &self,
        xs: &Bound<'_, PyAny>,
        q2s: &Bound<'_, PyAny>,
        out: &Bound<'_, PyAny>,
        stream: usize,
    ) -> PyResult<()> {
        let (xs, q2s, out) = (
            CudaArray::new(xs)?,
            CudaArray::new(q2s)?,
            CudaArray::new(out)?,
        );
        let expected = self.0.output_len(xs.len);
        if xs.len != q2s.len || out.len != expected {
            return Err(DeviceError::Shape {
                xs: xs.len,
                q2s: q2s.len,
                out: out.len,
                expected,
            }
            .into());
        }
        if out.readonly {
            return Err(PyValueError::new_err("`out` is read-only"));
        }
        // SAFETY: the interfaces describe device allocations of the checked lengths;
        // that they live on the grid's device, and `stream` too, is the caller's contract.
        Ok(unsafe {
            self.0.xfxq2_batch_raw(
                xs.ptr as *const f64,
                q2s.ptr as *const f64,
                xs.len,
                out.ptr as *mut f64,
                stream as CUstream,
            )
        }?)
    }
}

/// GPU batch evaluation of `NeoPDF`'s collinear PDF grids.
#[pymodule]
fn neopdf_cuda(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDeviceGrid>()
}
