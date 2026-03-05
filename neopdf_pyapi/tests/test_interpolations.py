import numpy as np
import pytest
import matplotlib.pyplot as plt

from neopdf.pdf import PDF as NEOPDF
from neopdf.writer import compress
from neopdf.gridpdf import GridArray, SubGrid
from neopdf.metadata import InterpolatorType, SetType, MetaData, PhysicsParameters


def f_ubar_abmp16_nnlo(x):
    """Analytic u-bar parametrization from ABMP16 NNLO."""
    return (
        0.0703
        * x ** (-0.415 * (1 + 4.44 * x) * (1 + 0.0373 * np.log(x)))
        * (1 - x) ** 7.75
    )


def f_gluon_herapdf20_nlo(x):
    """Analytic gluon parametrization from HERAPDF 2.0 NLO."""
    return (
        4.34 * x ** (-0.015) * (1 - x) ** 9.11 - 1.048 * x ** (-0.167) * (1 - x) ** 25.0
    )


def remove_near_nodes(x_samples, nodes, rel_tol=1e-1, abs_tol=None):
    """Remove points from x_samples that are too close to grid
    nodes to avoid trivial matches.
    """
    x = np.asarray(x_samples)
    nodes = np.sort(np.asarray(nodes))
    diffs = np.diff(nodes)
    valid_diffs = diffs[diffs > 0]
    min_spacing = np.min(valid_diffs) if valid_diffs.size > 0 else np.min(np.abs(nodes))
    if abs_tol is None:
        abs_tol = max(min_spacing * rel_tol, np.finfo(float).eps)
    keep_mask = np.ones_like(x, dtype=bool)
    for n in nodes:
        keep_mask &= np.abs(x - n) > abs_tol
    return x[keep_mask]


def create_cheby_grid(n_points: int, x_min: float, x_max: float) -> np.ndarray:
    """Create a Chebyshev grid with logarithmic spacing."""
    u_min = np.log(x_min)
    u_max = np.log(x_max)
    grid_points = []
    for j in range(n_points):
        t_j = np.cos(np.pi * (n_points - 1 - j) / (n_points - 1))
        u_j = u_min + (u_max - u_min) * (t_j + 1.0) / 2.0
        grid_points.append(np.exp(u_j))
    return np.array(grid_points)


class TestInterpolations:
    @pytest.fixture(scope="class")
    def interpolation_sets(self, tmp_path_factory):
        """Fixture to generate toy NeoPDF sets with different interpolation
        types.
        """
        tmp_dir = tmp_path_factory.mktemp("interp_sets")

        N_NODES = 100
        N_CHEB_NODES = 50
        N_CHEB_NODES_B = 25
        q2_values = np.logspace(1, 6, 40)
        x_values = np.logspace(-6, 0, N_NODES)

        self._generate_pdf(
            tmp_dir,
            "logbilinear",
            InterpolatorType.LogBilinear,
            [x_values],
            q2_values,
        )
        self._generate_pdf(
            tmp_dir,
            "logbicubic",
            InterpolatorType.LogBicubic,
            [x_values],
            q2_values,
        )

        x_cheb = [
            create_cheby_grid(N_CHEB_NODES, 1e-6, 0.2),
            create_cheby_grid(N_CHEB_NODES, 0.2, 1.0),
        ]
        self._generate_pdf(
            tmp_dir,
            f"logchebyshev_{N_CHEB_NODES}",
            InterpolatorType.LogChebyshev,
            x_cheb,
            q2_values,
        )

        x_cheb_b = [
            create_cheby_grid(N_CHEB_NODES_B, 1e-6, 0.2),
            create_cheby_grid(N_CHEB_NODES_B, 0.2, 1.0),
        ]
        self._generate_pdf(
            tmp_dir,
            f"logchebyshev_{N_CHEB_NODES_B}",
            InterpolatorType.LogChebyshev,
            x_cheb_b,
            q2_values,
        )

        return {
            "path": tmp_dir,
            "x_nodes": x_values,
            "N_CHEB_NODES": N_CHEB_NODES,
            "N_CHEB_NODES_B": N_CHEB_NODES_B,
        }

    def _generate_pdf(self, path, name, interp_type, x_vals_list, q2_vals):
        global_x_min = min(xv.min() for xv in x_vals_list)
        global_x_max = max(xv.max() for xv in x_vals_list)

        meta = self._get_metadata(interp_type, global_x_min, global_x_max, q2_vals)
        subgrids = [self._create_subgrid(xv, q2_vals) for xv in x_vals_list]
        grid_member = GridArray(pids=[-2, 21], subgrids=subgrids)
        compress(
            grids=[grid_member],
            metadata=meta,
            path=str(
                path / f"interp_test_{name}.neopdf.lz4",
            ),
        )

    def _get_metadata(self, interp_type, x_min, x_max, q2_vals):
        phys = PhysicsParameters(
            flavor_scheme="fixed",
            order_qcd=2,
            alphas_order_qcd=2,
            m_z=91.1876,
        )
        _as_q = np.geomspace(np.sqrt(q2_vals.min()), np.sqrt(q2_vals.max()), 6)

        return MetaData(
            set_desc="Toy NeoPDF set",
            set_index=123456,
            num_members=1,
            x_min=x_min,
            x_max=x_max,
            q_min=np.sqrt(q2_vals.min()),
            q_max=np.sqrt(q2_vals.max()),
            xsi_min=0.0,
            xsi_max=0.0,
            delta_min=0.0,
            delta_max=0.0,
            flavors=[-2, 21],
            format="neopdf",
            alphas_q_values=_as_q,
            alphas_vals=np.random.uniform(0.1, 0.2, 6),
            polarised=False,
            set_type=SetType.SpaceLike,
            interpolator_type=interp_type,
            phys_params=phys,
        )

    def _create_subgrid(self, x_vals, q2_vals):
        pids = [-2, 21]
        xq2_flavors = []
        for pid in pids:
            xf_func = f_gluon_herapdf20_nlo if pid == 21 else f_ubar_abmp16_nnlo
            xq2_array = np.zeros((x_vals.size, q2_vals.size))
            for i, x in enumerate(x_vals):
                xq2_array[i, :] = xf_func(x)
            xq2_flavors.append(xq2_array)

        grid = np.array(xq2_flavors).reshape(
            1,
            1,
            1,
            1,
            1,
            len(pids),
            x_vals.size,
            q2_vals.size,
        )
        return SubGrid(
            xs=x_vals,
            q2s=q2_vals,
            kts=[0.0],
            xsis=[0.0],
            deltas=[0.0],
            nucleons=[1],
            alphas=[0.118],
            grid=grid,
        )

    @pytest.mark.mpl_image_compare(baseline_dir="baseline", tolerance=10)
    @pytest.mark.parametrize("pid, label", [(21, "gluon"), (-2, "ubar")])
    def test_interpolation_visual(self, interpolation_sets, pid, label):
        """Generates error plots and checks numerical consistency."""
        base_path = interpolation_sets["path"]

        pdf_lin = NEOPDF(str(base_path / "interp_test_logbilinear.neopdf.lz4"))
        pdf_cub = NEOPDF(str(base_path / "interp_test_logbicubic.neopdf.lz4"))
        pdf_cheb = NEOPDF(
            str(
                base_path
                / f"interp_test_logchebyshev_{interpolation_sets['N_CHEB_NODES']}.neopdf.lz4"
            )
        )
        pdf_cheb_b = NEOPDF(
            str(
                base_path
                / f"interp_test_logchebyshev_{interpolation_sets['N_CHEB_NODES_B']}.neopdf.lz4"
            )
        )

        x_tests = np.logspace(-6, 0, 250)
        x_clean = remove_near_nodes(x_tests, interpolation_sets["x_nodes"])

        xf_ref_func = f_gluon_herapdf20_nlo if pid == 21 else f_ubar_abmp16_nnlo
        ref = xf_ref_func(x_clean)
        q2_test = 1e2

        res_lin = np.array([pdf_lin.xfxQ2(pid, x, q2_test) for x in x_clean])
        res_cub = np.array([pdf_cub.xfxQ2(pid, x, q2_test) for x in x_clean])
        res_cheb = np.array([pdf_cheb.xfxQ2(pid, x, q2_test) for x in x_clean])
        res_cheb_b = np.array([pdf_cheb_b.xfxQ2(pid, x, q2_test) for x in x_clean])

        # Plotting
        fig, ax = plt.subplots(figsize=(5.6, 3.9))
        ax.loglog(
            x_clean,
            1e-3 * np.abs(ref),
            color="black",
            linewidth=2,
            label=f"(x 1e-3) - {label.upper()}",
        )
        ax.loglog(
            x_clean,
            np.abs(res_lin / ref - 1),
            label="Linear (100)",
            color="C2",
            alpha=0.7,
        )
        ax.loglog(
            x_clean,
            np.abs(res_cub / ref - 1),
            label="Cubic (100)",
            color="C0",
            alpha=0.7,
        )
        ax.loglog(
            x_clean,
            np.abs(res_cheb_b / ref - 1),
            label="Cheb (25+25)",
            color="C4",
            alpha=0.7,
        )
        ax.loglog(
            x_clean,
            np.abs(res_cheb / ref - 1),
            label="Cheb (50+50)",
            color="C1",
            alpha=0.7,
        )

        ax.set_xlabel("x")
        ax.set_ylabel(r"|f_interp / f_ref - 1|")
        ax.set_title(f"Interpolation Error - {label.upper()}")
        ax.set_xlim(1e-6, 1)
        ax.set_ylim(1e-16, 1)
        shift_ylegend = 0.20 if label == "ubar" else 0.35
        ax.legend(
            fontsize=10,
            ncols=2,
            loc="center",
            frameon=False,
            bbox_to_anchor=(0.52, shift_ylegend),
        )

        return fig
