//! The C-language interface to device grids (feature `capi`).
//!
//! Mirrors `neopdf_capi`'s conventions: opaque, heap-allocated
//! handles released with a `_free` function, and status codes
//! for fallible calls. A grid is built from a set name, so this
//! library is self-contained and can be used next to `neopdf_capi`.

use std::ffi::{c_char, c_void, CStr};
use std::slice;

use crate::device::DeviceError;
pub use crate::foreign::NeoPDFDeviceGrid;

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
    unsafe { &*grid }.n_members()
}

/// Returns the number of flavors per member.
///
/// # Safety
///
/// `grid` must be a valid device grid.
#[no_mangle]
pub const unsafe extern "C" fn neopdf_device_grid_num_pids(grid: *const NeoPDFDeviceGrid) -> usize {
    unsafe { &*grid }.pids().len()
}

/// Writes the flavor PIDs, in output column order, into `pids`.
///
/// # Safety
///
/// `grid` must be a valid device grid and `pids` must hold `num_pids` values, with
/// `num_pids` equal to `neopdf_device_grid_num_pids(grid)`.
#[no_mangle]
pub const unsafe extern "C" fn neopdf_device_grid_pids(
    grid: *const NeoPDFDeviceGrid,
    pids: *mut i32,
    num_pids: usize,
) -> NeopdfCudaResult {
    if grid.is_null() || pids.is_null() {
        return NeopdfCudaResult::ErrorNullPointer;
    }
    let grid = unsafe { &*grid };
    if num_pids != grid.pids().len() {
        return NeopdfCudaResult::ErrorInvalidLength;
    }
    unsafe { slice::from_raw_parts_mut(pids, num_pids) }.copy_from_slice(grid.pids());
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
    unsafe { &*grid }.output_len(n_events)
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
    grid.xfxq2_batch_host(xs, q2s, out).into()
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
    unsafe { grid.xfxq2_batch_raw(xs, q2s, n_events, out, stream.cast()) }.into()
}
