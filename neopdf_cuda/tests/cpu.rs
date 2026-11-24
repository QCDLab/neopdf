//! The flattened layout, evaluated on the host, against `neopdf`'s own CPU path.

mod common;

use neopdf::pdf::PDF;
use neopdf_cuda::grid_prep::DeviceGridLayout;

/// Checks the host evaluation against `xfxq2_allpids`/`alphas_q2` for every member
/// at every point of [`common::points`]. The host port keeps the CPU's operation
/// order, so the two must agree bit for bit.
fn assert_matches_cpu(members: &[PDF]) {
    let layout = DeviceGridLayout::build(members).unwrap();
    let (xs, q2s) = common::points(&members[0]);
    let out = common::eval_host(&layout, &xs, &q2s);

    let n_flavors = layout.n_flavors();
    let mut expected = vec![0.0; n_flavors + 1];
    let queries = xs
        .iter()
        .zip(&q2s)
        .flat_map(|p| members.iter().map(move |m| (p, m)));
    for (row, ((&x, &q2), member)) in out.chunks(n_flavors + 1).zip(queries) {
        member.xfxq2_allpids(&layout.pids, &[x, q2], &mut expected[..n_flavors]);
        expected[n_flavors] = member.alphas_q2(q2);
        #[allow(clippy::float_cmp)]
        let exact = row == expected;
        assert!(
            exact,
            "x = {x}, Q² = {q2}: got {row:?}, expected {expected:?}"
        );
    }
}

#[test]
fn hermite_matches_cpu_across_members_and_subgrids() {
    assert_matches_cpu(&PDF::load_pdfs(common::HERMITE_SET)[..10]);
}

#[test]
fn chebyshev_matches_cpu_across_members_and_subgrids() {
    let (_dir, members) = common::chebyshev_set();
    assert_matches_cpu(&members);
}
