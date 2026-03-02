use criterion::{criterion_group, criterion_main, Criterion};

use neopdf::pdf::PDF;

fn xfxq2(c: &mut Criterion) {
    let pdf = PDF::load("NNPDF40_nnlo_as_01180", 0);

    c.bench_function("xfxq2", |b| {
        b.iter(|| pdf.xfxq2(std::hint::black_box(21), std::hint::black_box(&[1e-3, 4.0])))
    });
}

fn xfxq2_cheby(c: &mut Criterion) {
    let pdf = PDF::load("MAP22_grids_FF_Km_N3LL.neopdf.lz4", 0);

    c.bench_function("xfxq2_cheby", |b| {
        b.iter(|| {
            pdf.xfxq2(
                std::hint::black_box(2),
                std::hint::black_box(&[1e-2, 5e-1, 10.0]),
            )
        })
    });
}

fn xfxq2_cheby_batch(c: &mut Criterion) {
    let pdf = PDF::load("MAP22_grids_FF_Km_N3LL.neopdf.lz4", 0);

    c.bench_function("xfxq2_cheby_batch", |b| {
        b.iter(|| {
            pdf.xfxq2_cheby_batch(
                std::hint::black_box(2),
                std::hint::black_box(&[&[1e-2, 5e-1, 10.0]]),
            )
        })
    });
}

fn xfxq2s(c: &mut Criterion) {
    let pdf = PDF::load("NNPDF40_nnlo_as_01180", 0);

    let ids: Vec<i32> = (-4..=4).filter(|&x| x != 0).collect();
    let xs = [1e-5, 1e-3, 1e-3, 1.0];
    let q2s = [5.0, 10.0, 100.0];

    let flatten_points: Vec<Vec<f64>> = xs
        .iter()
        .flat_map(|&x| q2s.iter().map(move |&q2| vec![x, q2]))
        .collect();
    let points_interp: Vec<&[f64]> = flatten_points.iter().map(Vec::as_slice).collect();
    let slice_points: &[&[f64]] = &points_interp;

    c.bench_function("xfxq2s", |b| {
        b.iter(|| {
            pdf.xfxq2s(
                std::hint::black_box(ids.clone()),
                std::hint::black_box(slice_points),
            )
        })
    });
}

fn xfxq2_members(c: &mut Criterion) {
    let pdfs = PDF::load_pdfs("NNPDF40_nnlo_as_01180");

    c.bench_function("xfxq2_members", |b| {
        b.iter(|| {
            pdfs.iter()
                .map(|pdf| pdf.xfxq2(std::hint::black_box(21), std::hint::black_box(&[1e-3, 4.0])))
        })
    });
}

fn xfxq2_allpids_cheby(c: &mut Criterion) {
    let pdf = PDF::load("MAP22_grids_FF_Km_N3LL.neopdf.lz4", 0);
    let ids: Vec<i32> = (-4..=4).filter(|&x| x != 0).collect();
    let point = [1e-2, 5e-1, 10.0];
    let mut out = vec![0.0; ids.len()];

    let mut group = c.benchmark_group("xfxq2_allpids_cheby");

    group.bench_function("fast_path", |b| {
        b.iter(|| {
            pdf.xfxq2_allpids(
                std::hint::black_box(&ids),
                std::hint::black_box(&point),
                std::hint::black_box(&mut out),
            )
        })
    });

    group.bench_function("slow_path_loop", |b| {
        b.iter(|| {
            for (i, &pid) in ids.iter().enumerate() {
                out[i] = pdf.xfxq2(std::hint::black_box(pid), std::hint::black_box(&point));
            }
        })
    });

    group.finish();
}

fn xfxq2s_cheby(c: &mut Criterion) {
    let pdf = PDF::load("MAP22_grids_FF_Km_N3LL.neopdf.lz4", 0);
    let ids: Vec<i32> = (-4..=4).filter(|&x| x != 0).collect();
    let pts = [
        [1e-4, 1e-2, 2.0],
        [1e-3, 1e-2, 2.0],
        [1e-2, 1e-2, 2.0],
        [1e-1, 1e-2, 2.0],
        [1e-4, 1e-1, 10.0],
        [1e-3, 1e-1, 10.0],
        [1e-2, 1e-1, 10.0],
        [1e-1, 1e-1, 10.0],
        [1e-4, 0.5, 100.0],
        [1e-3, 0.5, 100.0],
        [1e-2, 0.5, 100.0],
        [1e-1, 0.5, 100.0],
        [1e-4, 0.9, 1000.0],
        [1e-3, 0.9, 1000.0],
        [1e-2, 0.9, 1000.0],
        [1e-1, 0.9, 1000.0],
    ];
    let pts_slices: Vec<&[f64]> = pts.iter().map(|p| &p[..]).collect();

    let mut group = c.benchmark_group("xfxq2_matrix_cheby");

    group.bench_function("flavor_batched_xfxq2s", |b| {
        b.iter(|| {
            pdf.xfxq2s(
                std::hint::black_box(ids.clone()),
                std::hint::black_box(&pts_slices),
            )
        })
    });

    group.bench_function("point_batched_loop", |b| {
        b.iter(|| {
            for &pid in &ids {
                let _ = pdf.xfxq2_cheby_batch(
                    std::hint::black_box(pid),
                    std::hint::black_box(&pts_slices),
                );
            }
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    xfxq2,
    xfxq2s,
    xfxq2_members,
    xfxq2_cheby,
    xfxq2_cheby_batch,
    xfxq2_allpids_cheby,
    xfxq2s_cheby
);
criterion_main!(benches);
