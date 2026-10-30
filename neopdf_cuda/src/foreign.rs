//! Device grids driven from other languages (features `capi` and `python`).
//!
//! A [`NeoPDFDeviceGrid`] is built from a set name and owns its stream, so
//! the C and Python interfaces share one implementation. Callers of other
//! languages pass device memory as raw pointers and streams as raw handles,
//! which [`NeoPDFDeviceGrid::xfxq2_batch_raw`] borrows without taking ownership.

use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use cuda_core::sys::{
    cuEventRecord, cuStreamWaitEvent, CUevent_wait_flags_enum_CU_EVENT_WAIT_DEFAULT,
};
use cuda_core::{CudaContext, CudaStream, DeviceBuffer, IntoResult};
use neopdf::pdf::PDF;
use thiserror::Error;

pub use cuda_core::sys::CUstream;

use crate::device::{DeviceError, DeviceGrid};
use crate::grid_prep::{DeviceGridLayout, GridPrepError};

/// Errors detailing why a [`NeoPDFDeviceGrid`] could not be created.
#[derive(Debug, Error)]
pub enum NeoPDFDeviceGridError {
    /// The set can't be laid out for the device.
    #[error(transparent)]
    Layout(#[from] GridPrepError),
    /// The device could not be initialized or the upload failed.
    #[error(transparent)]
    Device(#[from] DeviceError),
}

/// Opaque pointer to every member of a PDF set, resident on a GPU.
pub struct NeoPDFDeviceGrid {
    grid: DeviceGrid,
    stream: Arc<CudaStream>,
    pids: Vec<i32>,
}

impl NeoPDFDeviceGrid {
    /// Loads every member of `pdf_name` and uploads it to GPU `device`.
    ///
    /// # Errors
    ///
    /// Returns an error if the set can't be laid out for the device, or the device
    /// can't be initialized.
    ///
    /// # Panics
    ///
    /// Panics if the set cannot be loaded.
    pub fn new(pdf_name: &str, device: usize) -> Result<Self, NeoPDFDeviceGridError> {
        let layout = DeviceGridLayout::build(&PDF::load_pdfs(pdf_name))?;
        let stream = CudaContext::new(device)
            .and_then(|ctx| ctx.new_stream())
            .map_err(DeviceError::from)?;
        let grid = DeviceGrid::upload(&stream, &layout)?;
        Ok(Self {
            grid,
            stream,
            pids: layout.pids,
        })
    }

    /// Number of members on the device.
    #[must_use]
    pub const fn n_members(&self) -> usize {
        self.grid.n_members()
    }

    /// Flavor PIDs, in output column order.
    #[must_use]
    pub const fn pids(&self) -> &[i32] {
        self.pids.as_slice()
    }

    /// Length of the batch output for `n_events` events; see [`DeviceGrid::output_len`].
    #[must_use]
    pub const fn output_len(&self, n_events: usize) -> usize {
        self.grid.output_len(n_events)
    }

    /// Evaluates host-resident `xs`/`q2s` into the host-resident `out`; see
    /// [`DeviceGrid::xfxq2_batch_host`].
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer lengths are inconsistent, or a device
    /// allocation, copy or launch fails.
    pub fn xfxq2_batch_host(
        &self,
        xs: &[f64],
        q2s: &[f64],
        out: &mut [f64],
    ) -> Result<(), DeviceError> {
        self.grid.xfxq2_batch_host(&self.stream, xs, q2s, out)
    }

    /// Enqueues the batch evaluation of `n_events` device-resident points into
    /// `out` on the caller's `stream`, without synchronizing the host.
    ///
    /// The kernel runs on this grid's own stream, fenced by events so it starts
    /// after, and finishes before, whatever the caller enqueues on `stream` around
    /// this call.
    ///
    /// # Errors
    ///
    /// Returns an error if the batch is too large for one launch, or a driver call
    /// fails.
    ///
    /// # Safety
    ///
    /// `xs` and `q2s` must be device allocations on the grid's device holding
    /// `n_events` values, `out` one holding [`Self::output_len`] values, and
    /// `stream` a stream of that device (or null for the default stream).
    pub unsafe fn xfxq2_batch_raw(
        &self,
        xs: *const f64,
        q2s: *const f64,
        n_events: usize,
        out: *mut f64,
        stream: CUstream,
    ) -> Result<(), DeviceError> {
        let ctx = self.stream.context();
        // SAFETY: upheld by the caller.
        let (xs, q2s, mut out) = unsafe {
            (
                Borrowed::new(xs, n_events, ctx),
                Borrowed::new(q2s, n_events, ctx),
                Borrowed::new(out, self.output_len(n_events), ctx),
            )
        };
        let ready = ctx.new_event(None)?;
        // SAFETY: `stream` is a valid stream of this context, as required by the caller.
        unsafe { cuEventRecord(ready.cu_event(), stream).result()? };
        self.stream.wait(&ready)?;
        self.grid
            .xfxq2_batch_device(&self.stream, &xs, &q2s, &mut out)?;
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
