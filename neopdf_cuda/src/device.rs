//! Device-resident grids and the batch evaluation kernel.
//!
//! [`DeviceGrid::upload`] copies a [`DeviceGridLayout`] to the GPU once, so it can be
//! reused across a whole integration run. [`DeviceGrid::xfxq2_batch_device`] is the
//! primitive a GPU-native caller wants: inputs and output already live on the
//! device, the launch is enqueued on the caller's stream and nothing is
//! synchronized. [`DeviceGrid::xfxq2_batch_host`] wraps it with the host transfers.
//!
//! The kernel runs one thread per entry of the `[n_events, n_members, n_flavors + 1]`
//! output, each evaluating [`GridView::eval`]: adjacent threads read adjacent
//! flavors of the same grid cell, which keeps the coefficient loads coalesced.

use cuda_core::{CudaStream, DeviceBuffer, DriverError, LaunchConfig1D, LaunchContractError};
use cuda_device::{kernel, launch_bounds, launch_contract, thread, DisjointSlice};
use cuda_host::{cuda_module, EmbeddedModuleError};
use thiserror::Error;

use crate::eval::{Chebyshev, GridView, Hermite};
use crate::grid_prep::{DeviceGridLayout, Interpolation};

/// Threads per block; must match the kernel's `launch_contract`.
const BLOCK_SIZE: u32 = 256;

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
        let index = thread::index_1d();
        let i = index.get();
        if let Some(entry) = out.get_mut(index) {
            *entry = if chebyshev {
                grid.eval::<Chebyshev>(i, xs, q2s)
            } else {
                grid.eval::<Hermite>(i, xs, q2s)
            };
        }
    }
}

/// Why a [`DeviceGrid`] operation failed.
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
    #[error("{xs} x values and {q2s} Q² values need an output of length {expected}, found {out}")]
    Shape {
        /// Length of the x buffer.
        xs: usize,
        /// Length of the Q² buffer.
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
    /// holding `alpha_s(Q²)`; see [`Self::output_len`].
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
        let n_blocks = u32::try_from(expected.div_ceil(BLOCK_SIZE as usize))
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
        let xs = DeviceBuffer::from_host(stream, xs)?;
        let q2s = DeviceBuffer::from_host(stream, q2s)?;
        let mut out_dev = DeviceBuffer::zeroed(stream, out.len())?;
        self.xfxq2_batch_device(stream, &xs, &q2s, &mut out_dev)?;
        Ok(out_dev.copy_to_host(stream, out)?)
    }
}
