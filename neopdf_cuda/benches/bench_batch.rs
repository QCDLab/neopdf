//! Batch throughput: the persistent-`DeviceGrid` kernel against a `rayon`-parallel
//! CPU baseline over every member. Needs a CUDA GPU:
//! `cargo oxide test -- --features cuda --release --bench bench_batch -- --bench`.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use cuda_core::{CudaContext, DeviceBuffer};
use neopdf::pdf::PDF;
use neopdf_cuda::device::DeviceGrid;
use neopdf_cuda::grid_prep::DeviceGridLayout;
use rayon::prelude::*;

/// `n` log-uniform `(x, Q^2)` points inside the grid, decorrelated between axes.
#[allow(clippy::cast_precision_loss)]
fn events(n: usize) -> (Vec<f64>, Vec<f64>) {
    (0..n)
        .map(|i| {
            let (u, v) = (i as f64 / n as f64, ((i * 7919) % n) as f64 / n as f64);
            (
                10f64.powf(5.9f64.mul_add(u, -6.0)),
                10f64.powf(7.0f64.mul_add(v, 0.5)),
            )
        })
        .unzip()
}

fn xfxq2_batch(c: &mut Criterion) {
    let members = PDF::load_pdfs("NNPDF40_nnlo_as_01180");
    let layout = DeviceGridLayout::build(&members).unwrap();
    let ctx = CudaContext::new(0).unwrap();
    let stream = ctx.default_stream();
    let grid = DeviceGrid::upload(&stream, &layout).unwrap();
    let n_columns = layout.n_flavors() + 1;

    let mut group = c.benchmark_group("xfxq2_batch");
    group.sample_size(10);
    // 100k events x 101 members x 12 columns is already ~1 GB of output.
    for n_events in [1_000, 10_000, 100_000] {
        let (xs, q2s) = events(n_events);
        group.throughput(Throughput::Elements((n_events * members.len()) as u64));

        let mut out = vec![0.0; grid.output_len(n_events)];
        group.bench_function(BenchmarkId::new("cpu_rayon", n_events), |b| {
            b.iter(|| {
                out.par_chunks_mut(members.len() * n_columns)
                    .zip(xs.par_iter().zip(&q2s))
                    .for_each(|(event, (&x, &q2))| {
                        for (row, member) in event.chunks_mut(n_columns).zip(&members) {
                            member.xfxq2_allpids(&layout.pids, &[x, q2], &mut row[..n_columns - 1]);
                            row[n_columns - 1] = member.alphas_q2(q2);
                        }
                    });
            });
        });

        let xs_dev = DeviceBuffer::from_host(&stream, &xs).unwrap();
        let q2s_dev = DeviceBuffer::from_host(&stream, &q2s).unwrap();
        let mut out_dev = DeviceBuffer::zeroed(&stream, grid.output_len(n_events)).unwrap();
        group.bench_function(BenchmarkId::new("gpu_device", n_events), |b| {
            b.iter(|| {
                grid.xfxq2_batch_device(&stream, &xs_dev, &q2s_dev, &mut out_dev)
                    .unwrap();
                stream.synchronize().unwrap();
            });
        });
        group.bench_function(BenchmarkId::new("gpu_host", n_events), |b| {
            b.iter(|| grid.xfxq2_batch_host(&stream, &xs, &q2s, &mut out).unwrap());
        });
    }
    group.finish();
}

criterion_group!(benches, xfxq2_batch);
criterion_main!(benches);
