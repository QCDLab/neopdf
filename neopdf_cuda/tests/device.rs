//! Device output against the host evaluation of the same layout.
//!
//! Needs a CUDA GPU: `cargo oxide test -- --features cuda`.

#![cfg(feature = "cuda")]

mod common;

use cuda_core::CudaContext;
use neopdf::pdf::PDF;
use neopdf_cuda::device::DeviceGrid;
use neopdf_cuda::grid_prep::DeviceGridLayout;

/// Runs the batch kernel over every member at every point of [`common::points`].
/// Device code may contract multiply-adds into FMAs, so agreement is to a few ULPs
/// rather than bit for bit.
fn assert_device_matches_host(members: &[PDF]) {
    let layout = DeviceGridLayout::build(members).unwrap();
    let (xs, q2s) = common::points(&members[0]);
    let expected = common::eval_host(&layout, &xs, &q2s);

    let ctx = CudaContext::new(0).unwrap();
    let stream = ctx.default_stream();
    let grid = DeviceGrid::upload(&stream, &layout).unwrap();
    let mut out = vec![0.0; grid.output_len(xs.len())];
    grid.xfxq2_batch_host(&stream, &xs, &q2s, &mut out).unwrap();

    assert_eq!(out.len(), expected.len());
    for (i, (&got, &want)) in out.iter().zip(&expected).enumerate() {
        assert!(
            (got - want).abs() <= want.abs().mul_add(1e-12, 1e-15),
            "entry {i}: device {got}, host {want}"
        );
    }
}

#[test]
fn hermite_batch_matches_host() {
    assert_device_matches_host(&PDF::load_pdfs(common::HERMITE_SET));
}

#[test]
fn chebyshev_batch_matches_host() {
    let (_dir, members) = common::chebyshev_set();
    assert_device_matches_host(&members);
}

#[test]
fn rejects_mismatched_batch_shapes() {
    let layout = DeviceGridLayout::build(&PDF::load_pdfs(common::HERMITE_SET)[..2]).unwrap();
    let ctx = CudaContext::new(0).unwrap();
    let stream = ctx.default_stream();
    let grid = DeviceGrid::upload(&stream, &layout).unwrap();
    let mut out = vec![0.0; grid.output_len(2)];
    assert!(grid
        .xfxq2_batch_host(&stream, &[0.1, 0.2], &[10.0], &mut out)
        .is_err());
}
