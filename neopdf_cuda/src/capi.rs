//! The C-language interface to device grids (feature `capi`).
//!
//! Mirrors `neopdf_capi`'s conventions: opaque, heap-allocated handles released with a
//! `_free` function, and status codes for fallible calls. A grid is built from a set
//! name, so this library is self-contained and can be used next to `neopdf_capi`.

use std::ffi::{c_char, c_void, CStr};
use std::ops::{Deref, DerefMut};
use std::slice;
use std::sync::Arc;

use cuda_core::sys::{
    cuEventRecord, cuStreamWaitEvent, CUevent_wait_flags_enum_CU_EVENT_WAIT_DEFAULT, CUstream,
};
use cuda_core::{CudaContext, CudaStream, DeviceBuffer, IntoResult};
use neopdf::pdf::PDF;

use crate::device::{DeviceError, DeviceGrid};
use crate::grid_prep::DeviceGridLayout;

/// Result codes for `neopdf_cuda` operations.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeopdfCudaResult {
    /// Operation completed successfully.
    Success = 0,
    /// A null pointer was encountered where a valid pointer was expected.
    ErrorNullPointer = -1,
    /// The provided length or size argument was invalid.
    ErrorInvalidLength = -4,
    /// A CUDA driver call (allocation, copy, launch, synchronization) failed.
    ErrorDevice = -5,
}

impl From<Result<(), DeviceError>> for NeopdfCudaResult {
    fn from(result: Result<(), DeviceError>) -> Self {
        match result {
            Ok(()) => Self::Success,
            Err(DeviceError::Shape { .. } | DeviceError::TooLarge(_)) => Self::ErrorInvalidLength,
            Err(_) => Self::ErrorDevice,
        }
    }
}

/// Opaque pointer to every member of a PDF set, resident on a GPU.
pub struct NeoPDFDeviceGrid {
    grid: DeviceGrid,
    stream: Arc<CudaStream>,
    pids: Vec<i32>,
}

impl NeoPDFDeviceGrid {
    fn new(pdf_name: &str, device: usize) -> Option<Self> {
        let layout = DeviceGridLayout::build(&PDF::load_pdfs(pdf_name)).ok()?;
        let stream = CudaContext::new(device).ok()?.new_stream().ok()?;
        let grid = DeviceGrid::upload(&stream, &layout).ok()?;
        Some(Self {
            grid,
            stream,
            pids: layout.pids,
        })
    }

    /// Runs the batch on the caller's `stream`: the kernel is enqueued on this
    /// grid's own stream, fenced by events so it starts after, and finishes before,
    /// whatever the caller enqueues on `stream` around this call.
    fn xfxq2_batch_on(
        &self,
        stream: CUstream,
        xs: &DeviceBuffer<f64>,
        q2s: &DeviceBuffer<f64>,
        out: &mut DeviceBuffer<f64>,
    ) -> Result<(), DeviceError> {
        let ctx = self.stream.context();
        let ready = ctx.new_event(None)?;
        // SAFETY: `stream` is a valid stream of this context, as required by the caller.
        unsafe { cuEventRecord(ready.cu_event(), stream).result()? };
        self.stream.wait(&ready)?;
        self.grid.xfxq2_batch_device(&self.stream, xs, q2s, out)?;
        let done = self.stream.record_event(None)?;
        // SAFETY: as above.
        unsafe {
            cuStreamWaitEvent(
                stream,
                done.cu_event(),
                CUevent_wait_flags_enum_CU_EVENT_WAIT_DEFAULT,
            )
            .result()?;
        };
        Ok(())
    }
}

/// A caller-owned device allocation viewed as a [`DeviceBuffer`], never freed here.
struct Borrowed(Option<DeviceBuffer<f64>>);

impl Borrowed {
    /// # Safety
    ///
    /// `ptr` must be a device allocation of `ctx` holding at least `len` values.
    unsafe fn new(ptr: *const f64, len: usize, ctx: &Arc<CudaContext>) -> Self {
        // SAFETY: upheld by the caller; `Drop` hands the allocation back unfreed.
        Self(Some(unsafe {
            DeviceBuffer::from_raw_parts(ptr as u64, len, ctx.clone())
        }))
    }
}

impl Deref for Borrowed {
    type Target = DeviceBuffer<f64>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().expect("present until dropped")
    }
}

impl DerefMut for Borrowed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().expect("present until dropped")
    }
}

impl Drop for Borrowed {
    fn drop(&mut self) {
        if let Some(buffer) = self.0.take() {
            let _ = buffer.into_raw_parts();
        }
    }
}

/// Loads every member of a PDF set and uploads it to GPU `device`.
///
/// Only collinear `LogBicubic`/`LogChebyshev` sets with a tabulated `alpha_s` are
/// supported. Returns `NULL` if the set can't be laid out for the device or the
/// device can't be initialized. Release the grid with `neopdf_device_grid_free`.
///
/// # Panics
///
/// Panics if `pdf_name` is not valid UTF-8 or the set cannot be loaded.
///
/// # Safety
///
/// `pdf_name` must be a null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn neopdf_device_grid_new(
    pdf_name: *const c_char,
    device: usize,
) -> *mut NeoPDFDeviceGrid {
    let pdf_name = unsafe { CStr::from_ptr(pdf_name) }
        .to_str()
        .expect("Invalid UTF-8 string");
    NeoPDFDeviceGrid::new(pdf_name, device)
        .map_or(std::ptr::null_mut(), |grid| Box::into_raw(Box::new(grid)))
}

/// Frees a device grid and its device memory.
///
/// # Safety
///
/// `grid` must be `NULL` or come from `neopdf_device_grid_new`, and not be used afterwards.
#[no_mangle]
pub unsafe extern "C" fn neopdf_device_grid_free(grid: *mut NeoPDFDeviceGrid) {
    if !grid.is_null() {
        drop(unsafe { Box::from_raw(grid) });
    }
}

/// Returns the number of members on the device.
///
/// # Safety
///
/// `grid` must be a valid device grid.
#[no_mangle]
pub const unsafe extern "C" fn neopdf_device_grid_num_members(
    grid: *const NeoPDFDeviceGrid,
) -> usize {
    unsafe { &*grid }.grid.n_members()
}

/// Returns the number of flavors per member.
///
/// # Safety
///
/// `grid` must be a valid device grid.
#[no_mangle]
pub const unsafe extern "C" fn neopdf_device_grid_num_pids(grid: *const NeoPDFDeviceGrid) -> usize {
    unsafe { &*grid }.pids.len()
}

/// Writes the flavor PIDs, in output column order, into `pids`.
///
/// # Safety
///
/// `grid` must be a valid device grid and `pids` must hold `num_pids` values, with
/// `num_pids` equal to `neopdf_device_grid_num_pids(grid)`.
#[no_mangle]
pub unsafe extern "C" fn neopdf_device_grid_pids(
    grid: *const NeoPDFDeviceGrid,
    pids: *mut i32,
    num_pids: usize,
) -> NeopdfCudaResult {
    if grid.is_null() || pids.is_null() {
        return NeopdfCudaResult::ErrorNullPointer;
    }
    let grid = unsafe { &*grid };
    if num_pids != grid.pids.len() {
        return NeopdfCudaResult::ErrorInvalidLength;
    }
    unsafe { slice::from_raw_parts_mut(pids, num_pids) }.copy_from_slice(&grid.pids);
    NeopdfCudaResult::Success
}

/// Returns the length of the batch output for `n_events` events:
/// `n_events * num_members * (num_pids + 1)`.
///
/// # Safety
///
/// `grid` must be a valid device grid.
#[no_mangle]
pub const unsafe extern "C" fn neopdf_device_grid_output_len(
    grid: *const NeoPDFDeviceGrid,
    n_events: usize,
) -> usize {
    unsafe { &*grid }.grid.output_len(n_events)
}

/// Evaluates every member and flavor, plus `alpha_s`, at `n_events` host-resident
/// `(xs[i], q2s[i])` points into the host-resident `out`, returning once `out` is filled.
///
/// `out` is laid out `[n_events][num_members][num_pids + 1]`, the last column holding
/// `alpha_s(Q²)`, and `out_len` must equal `neopdf_device_grid_output_len(grid, n_events)`.
///
/// # Safety
///
/// `grid` must be a valid device grid, `xs`/`q2s` must hold `n_events` values and
/// `out` must hold `out_len` values.
#[no_mangle]
pub unsafe extern "C" fn neopdf_device_grid_xfxq2_batch_host(
    grid: *const NeoPDFDeviceGrid,
    xs: *const f64,
    q2s: *const f64,
    n_events: usize,
    out: *mut f64,
    out_len: usize,
) -> NeopdfCudaResult {
    if grid.is_null() || xs.is_null() || q2s.is_null() || out.is_null() {
        return NeopdfCudaResult::ErrorNullPointer;
    }
    let grid = unsafe { &*grid };
    let (xs, q2s) = unsafe {
        (
            slice::from_raw_parts(xs, n_events),
            slice::from_raw_parts(q2s, n_events),
        )
    };
    let out = unsafe { slice::from_raw_parts_mut(out, out_len) };
    grid.grid
        .xfxq2_batch_host(&grid.stream, xs, q2s, out)
        .into()
}

/// Enqueues the same evaluation as `neopdf_device_grid_xfxq2_batch_host` on
/// device-resident buffers, ordered on the caller's CUDA `stream`.
///
/// Does not synchronize the host: the results are in `out` once `stream` reaches
/// this point.
///
/// `stream` is a `CUstream`/`cudaStream_t` of the same device (`NULL` for the default
/// stream); `out` must hold `neopdf_device_grid_output_len(grid, n_events)` values.
///
/// # Safety
///
/// `grid` must be a valid device grid; `xs`, `q2s` and `out` must be device
/// allocations (e.g. from `cudaMalloc`) on the grid's device, of the sizes above.
#[no_mangle]
pub unsafe extern "C" fn neopdf_device_grid_xfxq2_batch_device(
    grid: *const NeoPDFDeviceGrid,
    xs: *const f64,
    q2s: *const f64,
    n_events: usize,
    out: *mut f64,
    stream: *mut c_void,
) -> NeopdfCudaResult {
    if grid.is_null() || xs.is_null() || q2s.is_null() || out.is_null() {
        return NeopdfCudaResult::ErrorNullPointer;
    }
    let grid = unsafe { &*grid };
    let ctx = grid.stream.context();
    let out_len = grid.grid.output_len(n_events);
    let (xs, q2s, mut out) = unsafe {
        (
            Borrowed::new(xs, n_events, ctx),
            Borrowed::new(q2s, n_events, ctx),
            Borrowed::new(out, out_len, ctx),
        )
    };
    grid.xfxq2_batch_on(stream.cast(), &xs, &q2s, &mut out)
        .into()
}
