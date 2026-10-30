"""Tests of the `neopdf_cuda` Python interface against NeoPDF's CPU evaluation."""

import numpy as np
import pytest
from neopdf_cuda import DeviceGrid

SET = "NNPDF40_nnlo_as_01180"
XS, Q2S = (
    a.ravel()
    for a in np.meshgrid(np.geomspace(1e-6, 0.9, 9), np.geomspace(2.0, 1e6, 7))
)


@pytest.fixture(scope="module")
def grid() -> DeviceGrid:
    return DeviceGrid(SET)


def test_output_shape(grid: DeviceGrid) -> None:
    out = grid.xfxQ2_batch(XS, Q2S)
    assert out.shape == (XS.size, grid.num_members(), len(grid.pids()) + 1)
    assert out.size == grid.output_len(XS.size)


def test_host_matches_cpu(grid: DeviceGrid) -> None:
    neopdf = pytest.importorskip("neopdf.pdf")
    members = neopdf.PDF.mkPDFs(SET)
    pids = grid.pids()
    want = np.array(
        [
            [[*pdf.xfxQ2_allpids(pids, x, q2), pdf.alphasQ2(q2)] for pdf in members]
            for x, q2 in zip(XS, Q2S)
        ]
    )
    np.testing.assert_allclose(grid.xfxQ2_batch(XS, Q2S), want, rtol=1e-12, atol=1e-15)


def test_device_matches_host(grid: DeviceGrid) -> None:
    cp = pytest.importorskip("cupy")
    out = cp.empty((XS.size, grid.num_members(), len(grid.pids()) + 1))
    with cp.cuda.Stream(non_blocking=True) as stream:
        grid.xfxQ2_batch_device(cp.asarray(XS), cp.asarray(Q2S), out, stream.ptr)
        got = out.get(stream=stream)
    np.testing.assert_array_equal(got, grid.xfxQ2_batch(XS, Q2S))


def test_rejects_inconsistent_lengths(grid: DeviceGrid) -> None:
    with pytest.raises(ValueError):
        grid.xfxQ2_batch(XS, Q2S[:-1])


def test_rejects_host_arrays_on_the_device_path(grid: DeviceGrid) -> None:
    with pytest.raises(TypeError):
        grid.xfxQ2_batch_device(XS, Q2S, np.empty(grid.output_len(XS.size)))


def test_rejects_invalid_device_arrays(grid: DeviceGrid) -> None:
    cp = pytest.importorskip("cupy")
    xs, q2s = cp.asarray(XS), cp.asarray(Q2S)
    out = cp.empty(grid.output_len(XS.size))
    with pytest.raises(TypeError):
        grid.xfxQ2_batch_device(xs.astype(cp.float32), q2s, out)
    with pytest.raises(ValueError):
        grid.xfxQ2_batch_device(xs[::2], q2s[::2], out[: grid.output_len(xs[::2].size)])
    with pytest.raises(ValueError):
        grid.xfxQ2_batch_device(xs, q2s, out[:-1])


def test_rejects_unsupported_sets() -> None:
    with pytest.raises(ValueError):
        DeviceGrid("MAP22_grids_FF_Km_N3LL.neopdf.lz4")
