import numpy as np

from neopdf.gridpdf import SubGrid, GridArray


class TestGridPDF:
    def test_subgrid(self, xq2_points):
        xmin, xmax, q2min, q2max = (1e-5, 1.0, 1.65, 1.0e8)
        xs, q2s = xq2_points(xmin, xmax, q2min, q2max)
        kts = [0.5, 1.0]
        xis = [0.0]  # dummy values
        deltas = [0.0]  # dummy values
        nucleons = [1.0, 2.0]
        alphas = [0.118, 0.120]
        grid = np.random.rand(
            len(nucleons),
            len(alphas),
            len(xis),
            len(deltas),
            len(kts),
            len(xs),
            len(q2s),
            1,
        )

        subgrid = SubGrid(xs, q2s, kts, xis, deltas, nucleons, alphas, grid)

        assert subgrid.alphas_range() == (0.118, 0.120)
        assert subgrid.x_range() == (xs[0], xs[-1])
        assert subgrid.q2_range() == (q2s[0], q2s[-1])
        assert subgrid.grid_shape() == [
            len(nucleons),
            len(alphas),
            len(xis),
            len(deltas),
            len(kts),
            len(xs),
            len(q2s),
            1,
        ]

    def test_gridarray(self, xq2_points):
        xmin, xmax, q2min, q2max = (1e-5, 1.0, 1.65, 1.0e8)
        xs, q2s = xq2_points(xmin, xmax, q2min, q2max)
        kts = [0.5, 1.0]
        xis = [0.0]  # dummy values
        deltas = [0.0]  # dummy values
        nucleons = [1.0, 2.0]
        alphas = [0.118, 0.120]
        grid = np.random.rand(
            len(nucleons),
            len(alphas),
            len(xis),
            len(deltas),
            len(kts),
            len(xs),
            len(q2s),
            1,
        )

        subgrid1 = SubGrid(xs, q2s, kts, xis, deltas, nucleons, alphas, grid)
        subgrid2 = SubGrid(xs, q2s, kts, xis, deltas, nucleons, alphas, grid)

        pids = [21, -2, -1, 1, 2]
        grid_array = GridArray(pids, [subgrid1, subgrid2])

        assert grid_array.pids() == pids
        assert len(grid_array.subgrids()) == 2
