use neopdf::pdf::PDF;
use neopdf::precision::{Binary64, InterpolationError};
use neopdf::precision_pdf::PrecisionError;

/// The binary64 instance of the precision-generic path reproduces `PDF::xfxq2` and
/// `PDF::alphas_q2` inside the grid, across every subgrid and at the flavour thresholds.
#[test]
fn test_precision_view_reproduces_xfxq2_and_alphas() {
    let pdf = PDF::load("NNPDF40_nnlo_as_01180", 0);
    let view = pdf.with_arithmetic(Binary64).unwrap();
    let ranges = pdf.param_ranges();
    let (x_min, x_max) = (ranges.x.min, ranges.x.max);
    let (q2_min, q2_max) = (ranges.q2.min, ranges.q2.max);

    let mut points = Vec::new();
    for i in 0..=24 {
        let x = x_min * (x_max / x_min).powf(i as f64 / 24.0);
        for j in 0..=30 {
            points.push((x, q2_min * (q2_max / q2_min).powf(j as f64 / 30.0)));
        }
    }
    // Subgrid boundaries (flavour thresholds) and the points next to them.
    for subgrid in pdf.subgrids() {
        for q2 in [subgrid.q2_range.min, subgrid.q2_range.max] {
            for x in [1e-7, 1e-3, 0.2, 0.7] {
                points.push((x, q2));
                points.push((x, q2 * (1.0 + 1e-9)));
                points.push((x, q2 * (1.0 - 1e-9)));
            }
        }
    }

    for &(x, q2) in &points {
        if !(q2_min..=q2_max).contains(&q2) {
            continue;
        }
        for pid in [-5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 21] {
            let expected = pdf.xfxq2(pid, &[x, q2]);
            let got = view.xfxq2(pid, &x, &q2).unwrap();
            assert!(
                (got - expected).abs() <= 1e-12 * expected.abs().max(1e-3),
                "pid {pid}, x {x}, Q2 {q2}: {got} vs {expected}"
            );
        }
        let (expected, got) = (pdf.alphas_q2(q2), view.alphas_q2(&q2).unwrap());
        assert!(
            (got - expected).abs() <= 1e-14,
            "alpha_s(Q2 = {q2}): {got} vs {expected}"
        );
    }

    // Outside the grid the generic path reports instead of extrapolating.
    assert_eq!(
        view.xfxq2(21, &(x_min / 2.0), &100.0),
        Err(PrecisionError::Interpolation(
            InterpolationError::OutOfRange
        ))
    );
}
