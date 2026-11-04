//! Repeated device-resident batches on one stream, as a GPU event generator issues them.

use std::time::Instant;

use cuda_core::{CudaContext, DeviceBuffer};
use neopdf::pdf::PDF;
use neopdf_cuda::device::DeviceGrid;
use neopdf_cuda::grid_prep::DeviceGridLayout;

const N_EVENTS: usize = 20_000;
const N_BATCHES: usize = 50;

/// Phase-space points of one batch. A real generator samples these on the device.
#[allow(clippy::cast_precision_loss)]
fn sample(batch: usize) -> (Vec<f64>, Vec<f64>) {
    (0..N_EVENTS)
        .map(|i| {
            let u = ((i * 7919 + batch * 104_729) % N_EVENTS) as f64 / N_EVENTS as f64;
            let v = ((i * 6007 + batch * 15_485_863) % N_EVENTS) as f64 / N_EVENTS as f64;
            (
                10f64.powf(5.0f64.mul_add(u, -6.0)),
                10f64.powf(6.0f64.mul_add(v, 1.0)),
            )
        })
        .unzip()
}

#[allow(clippy::cast_precision_loss)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let members = PDF::load_pdfs("NNPDF40_nnlo_as_01180");
    let layout = DeviceGridLayout::build(&members)?;
    let ctx = CudaContext::new(0)?;
    let stream = ctx.new_stream()?;
    let grid = DeviceGrid::upload(&stream, &layout)?;

    // Device buffers are allocated once and reused by every batch.
    let mut xs_dev = DeviceBuffer::<f64>::zeroed(&stream, N_EVENTS)?;
    let mut q2s_dev = DeviceBuffer::<f64>::zeroed(&stream, N_EVENTS)?;
    let mut out_dev = DeviceBuffer::<f64>::zeroed(&stream, grid.output_len(N_EVENTS))?;

    let batches: Vec<_> = (0..N_BATCHES).map(sample).collect();
    let start = Instant::now();
    for (xs, q2s) in &batches {
        xs_dev.copy_from_host(&stream, xs)?;
        q2s_dev.copy_from_host(&stream, q2s)?;
        // Enqueued on `stream` without synchronizing; kernels consuming `out_dev`
        // (matrix elements, weights, histograms) would follow on the same stream.
        grid.xfxq2_batch_device(&stream, &xs_dev, &q2s_dev, &mut out_dev)?;
    }
    stream.synchronize()?;
    let elapsed = start.elapsed().as_secs_f64();

    // Check the last batch against the host evaluation of the same layout.
    let (xs, q2s) = batches.last().ok_or("no batch")?;
    let got = out_dev.to_host_vec(&stream)?;
    let mut expected = vec![0.0; got.len()];
    layout.eval_batch(xs, q2s, &mut expected);
    let max_deviation = got
        .iter()
        .zip(&expected)
        .map(|(g, e)| (g - e).abs() / e.abs().max(1e-300))
        .fold(0.0, f64::max);

    let pairs = (N_BATCHES * N_EVENTS * layout.n_members) as f64;
    println!(
        "{N_BATCHES} batches of {N_EVENTS} events x {} members",
        layout.n_members
    );
    println!(
        "throughput: {:.1} M (event, member) pairs/s",
        pairs / elapsed / 1e6
    );
    println!("max relative deviation from the host evaluation: {max_deviation:.1e}");
    Ok(())
}
