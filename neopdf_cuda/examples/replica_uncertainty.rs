//! Gluon replica uncertainty and `alpha_s`, evaluated for every member at once on the GPU.

use cuda_core::CudaContext;
use neopdf::pdf::PDF;
use neopdf_cuda::device::DeviceGrid;
use neopdf_cuda::grid_prep::DeviceGridLayout;

#[allow(clippy::cast_precision_loss)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Flatten every member of the set into one device-ready layout...
    let members = PDF::load_pdfs("NNPDF40_nnlo_as_01180");
    let layout = DeviceGridLayout::build(&members)?;

    // ...and upload it once: the grid then serves any number of batches.
    let ctx = CudaContext::new(0)?;
    let stream = ctx.default_stream();
    let grid = DeviceGrid::upload(&stream, &layout)?;

    let xs = [1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 0.3, 0.5, 0.8];
    let q2s = [1e4; 8];
    let mut out = vec![0.0; grid.output_len(xs.len())];
    grid.xfxq2_batch_host(&stream, &xs, &q2s, &mut out)?;

    // `out` is laid out [event][member][flavor], plus a last `alpha_s` column.
    let n_columns = layout.n_flavors() + 1;
    let gluon = layout
        .pids
        .iter()
        .position(|&pid| pid == 21)
        .ok_or("no gluon")?;
    let alphas = n_columns - 1;

    println!(
        "{:>10} {:>10} {:>14} {:>14} {:>10}",
        "x", "Q2", "xg", "uncertainty", "alpha_s"
    );
    for (event, (x, q2)) in xs.iter().zip(&q2s).enumerate() {
        let row = &out[event * layout.n_members * n_columns..][..layout.n_members * n_columns];
        let replicas: Vec<f64> = row.chunks(n_columns).skip(1).map(|m| m[gluon]).collect();
        let n = replicas.len() as f64;
        let mean = replicas.iter().sum::<f64>() / n;
        let variance = replicas.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1.0);
        println!(
            "{x:>10.1e} {q2:>10.1e} {:>14.6e} {:>14.6e} {:>10.6}",
            row[gluon],
            variance.sqrt(),
            row[alphas]
        );
    }
    Ok(())
}
