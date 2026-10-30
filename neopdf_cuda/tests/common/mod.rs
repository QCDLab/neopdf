//! Fixtures shared by the host and device validation tests.

#![allow(dead_code)]

use neopdf::gridpdf::GridArray;
use neopdf::metadata::{InterpolatorType, MetaData};
use neopdf::parser::SubgridData;
use neopdf::pdf::PDF;
use neopdf::writer::GridArrayCollection;
use neopdf_cuda::grid_prep::DeviceGridLayout;
use tempfile::TempDir;

/// A `LogBicubic` set split into two Q² subgrids at the bottom threshold.
pub const HERMITE_SET: &str = "NNPDF40_nnlo_as_01180";

/// `(x, Q²)` points spanning every subgrid, their shared boundary, the grid edges
/// and the out-of-range regions on every side.
pub fn points(pdf: &PDF) -> (Vec<f64>, Vec<f64>) {
    let ranges = pdf.param_ranges();
    let (x, q2) = (ranges.x, ranges.q2);
    let q2_threshold = pdf.subgrid(0).q2_range.max;
    let xs = [x.min * 0.5, x.min, 1e-7, 1e-4, 0.01, 0.3, 0.77, x.max, 1.5];
    let q2s = [
        q2.min * 0.5,
        q2.min,
        2.0,
        10.0,
        q2_threshold,
        q2_threshold * 1.0001,
        1e3,
        1e6,
        q2.max,
        q2.max * 2.0,
    ];
    xs.iter()
        .flat_map(|&x| q2s.iter().map(move |&q2| (x, q2)))
        .unzip()
}

/// The batch output of `layout` at `points`, evaluated on the host.
pub fn eval_host(layout: &DeviceGridLayout, xs: &[f64], q2s: &[f64]) -> Vec<f64> {
    let mut out = vec![0.0; layout.view().output_len(xs.len())];
    layout.eval_batch(xs, q2s, &mut out);
    out
}

/// `n` Chebyshev nodes spanning `[lo, hi]` in log space, with exact endpoints so
/// adjacent subgrids share their boundary knot.
#[allow(clippy::cast_precision_loss)]
fn chebyshev_knots(n: usize, lo: f64, hi: f64) -> Vec<f64> {
    let mut knots: Vec<f64> = (0..n)
        .map(|j| {
            let t = (std::f64::consts::PI * (n - 1 - j) as f64 / (n - 1) as f64).cos();
            (lo.ln() + (hi.ln() - lo.ln()) * (t + 1.0) / 2.0).exp()
        })
        .collect();
    knots[0] = lo;
    knots[n - 1] = hi;
    knots
}

/// No 2-D `LogChebyshev` set is published, so write a synthetic one: three members,
/// two Q² subgrids split at the bottom threshold, and [`HERMITE_SET`]'s `alpha_s`.
///
/// The returned directory holds the set and must outlive the loaded members.
pub fn chebyshev_set() -> (TempDir, Vec<PDF>) {
    let reference = PDF::load(HERMITE_SET, 0);
    let pids = vec![-2, -1, 1, 2, 21];
    let xs = chebyshev_knots(40, 1e-7, 1.0);
    let q2_splits = [(1.65, 24.2064), (24.2064, 1e8)];
    let grids: Vec<GridArray> = (1..=3)
        .map(|member| {
            let subgrids = q2_splits
                .iter()
                .map(|&(lo, hi)| {
                    let q2s = chebyshev_knots(20, lo, hi);
                    let grid_data = pids
                        .iter()
                        .flat_map(|&pid| xs.iter().map(move |&x| (pid, x)))
                        .flat_map(|(pid, x)| {
                            q2s.iter().map(move |&q2: &f64| {
                                f64::from(member * pid) * x.powf(-0.3) * (1.0 - x).powi(3) * q2.ln()
                            })
                        })
                        .collect();
                    SubgridData {
                        nucleons: vec![1.0],
                        alphas: vec![0.118],
                        xis: vec![0.0],
                        deltas: vec![0.0],
                        kts: vec![0.0],
                        xs: xs.clone(),
                        q2s,
                        grid_data,
                    }
                })
                .collect();
            GridArray::new(subgrids, pids.clone())
        })
        .collect();
    let metadata = MetaData {
        interpolator_type: InterpolatorType::LogChebyshev,
        num_members: 3,
        flavors: pids,
        x_min: xs[0],
        x_max: 1.0,
        q_min: q2_splits[0].0.sqrt(),
        q_max: q2_splits[1].1.sqrt(),
        ..reference.metadata().clone()
    };

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("chebyshev.neopdf.lz4");
    GridArrayCollection::compress(&grids.iter().collect::<Vec<_>>(), &metadata, &path).unwrap();
    let members = PDF::load_pdfs(path.to_str().unwrap());
    (dir, members)
}
