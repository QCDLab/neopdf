//! Device-resident grids and the batch evaluation kernel.
//!
//! [`DeviceGrid::upload`] copies a [`DeviceGridLayout`] to the GPU once, so it can be
//! reused across a whole integration run. [`DeviceGrid::xfxq2_batch_device`] is the
//! primitive a GPU-native caller wants: inputs and output already live on the
//! device. The launch is then enqueued on the caller's stream and nothing is
//! synchronized. [`DeviceGrid::xfxq2_batch_host`] wraps it with the host transfers.
//!
//! The kernel runs one thread per row of the `[n_events, n_members, n_flavors + 1]`
//! output, each evaluating [`GridView::eval_row`]: the logarithms and the knot search
//! of an `(event, member)` are done once for all its flavors and `alpha_s`.

use std::mem::{size_of, ManuallyDrop};
use std::sync::Arc;

use cuda_core::memory::memcpy_dtoh_async;
use cuda_core::sys::{
    cuPointerGetAttribute, CUdeviceptr, CUmemorytype_enum_CU_MEMORYTYPE_HOST,
    CUpointer_attribute_enum_CU_POINTER_ATTRIBUTE_MEMORY_TYPE,
};
use cuda_core::{
    CudaEvent, CudaStream, DeviceBuffer, DeviceCopy, DriverError, IntoResult, LaunchConfig1D,
    LaunchContractError,
};
use cuda_device::{kernel, launch_bounds, launch_contract, thread, DisjointSlice};
use cuda_host::{cuda_module, EmbeddedModuleError};
use thiserror::Error;

use crate::eval::{Chebyshev, GridView, Hermite};
use crate::grid_prep::{DeviceGridLayout, Interpolation};

/// Threads per block; must match the kernel's `launch_contract`.
const BLOCK_SIZE: u32 = 256;

/// Output bytes per chunk of [`DeviceGrid::xfxq2_batch_host`] into page-locked memory:
/// large enough to keep launches and copies efficient, small enough to overlap.
const CHUNK_BYTES: usize = 64 << 20;

// `#[cuda_module]` generates the kernel's entry point and expects its items in scope.
#[cuda_module]
#[allow(clippy::wildcard_imports, clippy::no_mangle_with_rust_abi)]
mod kernels {
    use super::*;

    /// Fills `out` with the `[n_events, n_members, n_flavors + 1]` batch output.
    ///
    /// The fast path is a uniform runtime flag rather than a type parameter:
    /// `cuda-oxide` only links a library's embedded artifact into downstream
    /// binaries for non-generic kernels, and every thread takes the same branch.
    #[kernel]
    #[launch_bounds(256)]
    #[launch_contract(domain = 1, block = (256, 1, 1))]
    #[allow(clippy::too_many_arguments)]
    pub fn xfxq2_batch(
        chebyshev: bool,
        n_members: usize,
        n_flavors: usize,
        clip_floor: f64,
        log_xs: &[f64],
        x_nodes: &[f64],
        log_q2s: &[f64],
        q2_nodes: &[f64],
        q2_offsets: &[usize],
        values: &[f64],
        value_offsets: &[usize],
        alphas_log_q2s: &[f64],
        alphas: &[f64],
        alphas_offsets: &[usize],
        xs: &[f64],
        q2s: &[f64],
        mut out: DisjointSlice<f64>,
    ) {
        let grid = GridView {
            n_members,
            n_flavors,
            clip_floor,
            log_xs,
            x_nodes,
            log_q2s,
            q2_nodes,
            q2_offsets,
            values,
            value_offsets,
            alphas_log_q2s,
            alphas,
            alphas_offsets,
        };
        let row = thread::index_1d().get();
        let n_columns = grid.n_columns();
        if (row + 1) * n_columns <= out.len() {
            // SAFETY: thread `row` alone writes the entries `[row, row + 1) * n_columns`,
            // which the check above keeps inside `out`.
            let entries = unsafe {
                core::slice::from_raw_parts_mut(out.as_mut_ptr().add(row * n_columns), n_columns)
            };
            if chebyshev {
                grid.eval_row::<Chebyshev>(row, xs, q2s, entries);
            } else {
                grid.eval_row::<Hermite>(row, xs, q2s, entries);
            }
        }
    }
}

/// Errors detailing why a [`DeviceGrid`] operation failed.
#[derive(Debug, Error)]
pub enum DeviceError {
    /// A CUDA driver call (allocation, copy, launch) failed.
    #[error(transparent)]
    Driver(#[from] DriverError),
    /// The embedded kernel module could not be loaded.
    #[error(transparent)]
    Module(#[from] EmbeddedModuleError),
    /// The launch configuration was rejected by the kernel's launch contract.
    #[error(transparent)]
    Launch(#[from] LaunchContractError),
    /// The batch buffers don't have consistent lengths.
    #[error("{xs} x values and {q2s} Q^2 values need an output of length {expected}, found {out}")]
    Shape {
        /// Length of the x buffer.
        xs: usize,
        /// Length of the Q^2 buffer.
        q2s: usize,
        /// Length of the output buffer.
        out: usize,
        /// The output length the inputs require.
        expected: usize,
    },
    /// The batch needs more blocks than a single launch can address.
    #[error("a batch of {0} outputs exceeds a single launch")]
    TooLarge(usize),
}

/// A [`DeviceGridLayout`] resident on the GPU, with its loaded kernel module.
pub struct DeviceGrid {
    module: kernels::LoadedModule,
    interpolation: Interpolation,
    n_members: usize,
    n_flavors: usize,
    clip_floor: f64,
    log_xs: DeviceBuffer<f64>,
    x_nodes: DeviceBuffer<f64>,
    log_q2s: DeviceBuffer<f64>,
    q2_nodes: DeviceBuffer<f64>,
    q2_offsets: DeviceBuffer<usize>,
    values: DeviceBuffer<f64>,
    value_offsets: DeviceBuffer<usize>,
    alphas_log_q2s: DeviceBuffer<f64>,
    alphas: DeviceBuffer<f64>,
    alphas_offsets: DeviceBuffer<usize>,
    /// Copies chunks back to the host while the next ones are computed.
    copy_stream: Arc<CudaStream>,
}

impl DeviceGrid {
    /// Loads the kernel module into `stream`'s context and copies `layout` to the device.
    ///
    /// # Errors
    ///
    /// Returns an error if the module fails to load or a device allocation or
    /// copy fails.
    pub fn upload(stream: &CudaStream, layout: &DeviceGridLayout) -> Result<Self, DeviceError> {
        // SAFETY: `kernels` is the only `cuda_module` of this package, so the embedded
        // artifact selected by package name is the one compiled from it.
        let module = unsafe { kernels::load(stream.context())? };
        let upload = |data: &[f64]| DeviceBuffer::from_host(stream, data);
        let upload_offsets = |data: &[usize]| DeviceBuffer::from_host(stream, data);
        Ok(Self {
            module,
            interpolation: layout.interpolation,
            n_members: layout.n_members,
            n_flavors: layout.n_flavors(),
            clip_floor: layout.clip_floor,
            log_xs: upload(&layout.log_xs)?,
            x_nodes: upload(&layout.x_nodes)?,
            log_q2s: upload(&layout.log_q2s)?,
            q2_nodes: upload(&layout.q2_nodes)?,
            q2_offsets: upload_offsets(&layout.q2_offsets)?,
            values: upload(&layout.values)?,
            value_offsets: upload_offsets(&layout.value_offsets)?,
            alphas_log_q2s: upload(&layout.alphas_log_q2s)?,
            alphas: upload(&layout.alphas)?,
            alphas_offsets: upload_offsets(&layout.alphas_offsets)?,
            copy_stream: stream.context().new_stream()?,
        })
    }

    /// Number of members on the device.
    #[must_use]
    pub const fn n_members(&self) -> usize {
        self.n_members
    }

    /// Number of flavors per member (the output has one more column, `alpha_s`).
    #[must_use]
    pub const fn n_flavors(&self) -> usize {
        self.n_flavors
    }

    /// Length of the batch output for `n_events` events: `n_events * n_members * (n_flavors + 1)`.
    #[must_use]
    pub const fn output_len(&self, n_events: usize) -> usize {
        n_events * self.n_members * (self.n_flavors + 1)
    }

    /// Enqueues the batch evaluation of device-resident `xs`/`q2s` into `out` on
    /// `stream`, without synchronizing.
    ///
    /// `out` is laid out `[n_events, n_members, n_flavors + 1]`, the last column
    /// holding `alpha_s(Q^2)`; see [`Self::output_len`].
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer lengths are inconsistent, the batch is too
    /// large for one launch, or the launch is rejected.
    pub fn xfxq2_batch_device(
        &self,
        stream: &CudaStream,
        xs: &DeviceBuffer<f64>,
        q2s: &DeviceBuffer<f64>,
        out: &mut DeviceBuffer<f64>,
    ) -> Result<(), DeviceError> {
        let expected = self.output_len(xs.len());
        if xs.len() != q2s.len() || out.len() != expected {
            return Err(DeviceError::Shape {
                xs: xs.len(),
                q2s: q2s.len(),
                out: out.len(),
                expected,
            });
        }
        if expected == 0 {
            return Ok(());
        }
        // one thread per `(event, member)` row
        let n_rows = expected / (self.n_flavors + 1);
        let n_blocks = u32::try_from(n_rows.div_ceil(BLOCK_SIZE as usize))
            .map_err(|_| DeviceError::TooLarge(expected))?;
        let prepared = self
            .module
            .prepare_xfxq2_batch(LaunchConfig1D::new(n_blocks, BLOCK_SIZE, 0))?;
        self.module.xfxq2_batch(
            stream,
            &prepared,
            self.interpolation == Interpolation::Chebyshev,
            self.n_members,
            self.n_flavors,
            self.clip_floor,
            &self.log_xs,
            &self.x_nodes,
            &self.log_q2s,
            &self.q2_nodes,
            &self.q2_offsets,
            &self.values,
            &self.value_offsets,
            &self.alphas_log_q2s,
            &self.alphas,
            &self.alphas_offsets,
            xs,
            q2s,
            out,
        )?;
        Ok(())
    }

    /// Evaluates host-resident `xs`/`q2s` into the host-resident `out`, returning
    /// once `out` is filled.
    ///
    /// See [`Self::xfxq2_batch_device`] for the output layout.
    ///
    /// The copy of `out` back to the host dominates for large batches. Into
    /// page-locked memory, a [`cuda_core::PinnedHostBuffer`] passed as `out`, it runs
    /// several times faster than into a pageable `Vec`, and a batch of more than 64 MiB
    /// of output is computed in chunks, each copied back while the next one is computed
    /// (on two chunk buffers rather than the whole output on the device). Allocating
    /// pinned memory is slow, so allocate it once and reuse it across batches.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer lengths are inconsistent, or a device
    /// allocation, copy or launch fails.
    pub fn xfxq2_batch_host(
        &self,
        stream: &CudaStream,
        xs: &[f64],
        q2s: &[f64],
        out: &mut [f64],
    ) -> Result<(), DeviceError> {
        let row_len = self.n_members * (self.n_flavors + 1);
        let chunk_events = (CHUNK_BYTES / (row_len * size_of::<f64>())).max(1);
        if xs.len() <= chunk_events || !is_page_locked(stream, out.as_ptr()) {
            let xs = DeviceBuffer::from_host(stream, xs)?;
            let q2s = DeviceBuffer::from_host(stream, q2s)?;
            let mut out_dev = DeviceBuffer::zeroed(stream, out.len())?;
            self.xfxq2_batch_device(stream, &xs, &q2s, &mut out_dev)?;
            return Ok(out_dev.copy_to_host(stream, out)?);
        }
        let expected = self.output_len(xs.len());
        if xs.len() != q2s.len() || out.len() != expected {
            return Err(DeviceError::Shape {
                xs: xs.len(),
                q2s: q2s.len(),
                out: out.len(),
                expected,
            });
        }

        let xs = DeviceBuffer::from_host(stream, xs)?;
        let q2s = DeviceBuffer::from_host(stream, q2s)?;
        let chunks = [
            DeviceBuffer::<f64>::zeroed(stream, chunk_events * row_len)?,
            DeviceBuffer::<f64>::zeroed(stream, chunk_events * row_len)?,
        ];
        let mut enqueue = || -> Result<(), DeviceError> {
            // the last copy out of each chunk buffer, before it is overwritten
            let mut copied: [Option<CudaEvent>; 2] = [None, None];
            for (index, start) in (0..xs.len()).step_by(chunk_events).enumerate() {
                let n_events = chunk_events.min(xs.len() - start);
                let len = n_events * row_len;
                let slot = index % 2;
                if let Some(copied) = &copied[slot] {
                    stream.wait(copied)?;
                }
                // SAFETY: each view lies inside its buffer, which outlives it.
                let (chunk_xs, chunk_q2s, mut chunk_out) = unsafe {
                    (
                        view(&xs, start, n_events),
                        view(&q2s, start, n_events),
                        view(&chunks[slot], 0, len),
                    )
                };
                self.xfxq2_batch_device(stream, &chunk_xs, &chunk_q2s, &mut chunk_out)?;
                self.copy_stream.wait(&stream.record_event(None)?)?;
                let host = &mut out[start * row_len..][..len];
                // SAFETY: `host` is page-locked memory of `len` values, and the copy is
                // complete before this function returns, through the synchronization
                // below.
                unsafe {
                    memcpy_dtoh_async(
                        host.as_mut_ptr(),
                        chunks[slot].cu_deviceptr(),
                        len * size_of::<f64>(),
                        self.copy_stream.cu_stream(),
                    )?;
                }
                copied[slot] = Some(self.copy_stream.record_event(None)?);
            }
            Ok(())
        };
        // Wait for every enqueued copy, even after an error, as they write into `out`.
        let enqueued = enqueue();
        let copied = self.copy_stream.synchronize();
        enqueued?;
        Ok(copied?)
    }
}

/// Whether `ptr` is page-locked host memory, so that a copy into it is asynchronous.
fn is_page_locked(stream: &CudaStream, ptr: *const f64) -> bool {
    let mut memory_type = 0u32;
    stream.context().bind_to_thread().is_ok()
        // SAFETY: the attribute is written as an `unsigned int`; the pointer is only
        // looked up, never dereferenced.
        && unsafe {
            cuPointerGetAttribute(
                (&raw mut memory_type).cast(),
                CUpointer_attribute_enum_CU_POINTER_ATTRIBUTE_MEMORY_TYPE,
                ptr as CUdeviceptr,
            )
        }
        .result()
        .is_ok()
        && memory_type == CUmemorytype_enum_CU_MEMORYTYPE_HOST
}

/// The `len` values of `buffer` from `offset` on, as a buffer that is never freed.
///
/// # Safety
///
/// `offset + len` must not exceed `buffer.len()`, and the view must not outlive
/// `buffer`.
unsafe fn view<T: DeviceCopy>(
    buffer: &DeviceBuffer<T>,
    offset: usize,
    len: usize,
) -> ManuallyDrop<DeviceBuffer<T>> {
    let ptr = buffer.cu_deviceptr() + (offset * size_of::<T>()) as CUdeviceptr;
    ManuallyDrop::new(unsafe { DeviceBuffer::from_raw_parts(ptr, len, buffer.context().clone()) })
}
