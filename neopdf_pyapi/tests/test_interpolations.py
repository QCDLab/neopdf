from __future__ import annotations

import numpy as np
import pytest

import matplotlib.pyplot as plt
from pathlib import Path
from matplotlib.figure import Figure
from pydantic import BaseModel, Field

from neopdf.gridpdf import GridArray, SubGrid  # type: ignore[import-untyped]
from neopdf.metadata import (  # type: ignore[import-untyped]
    InterpolatorType,
    MetaData,
    PhysicsParameters,
    SetType,
)
from neopdf.pdf import PDF as NEOPDF  # type: ignore[import-untyped]
from neopdf.writer import compress  # type: ignore[import-untyped]


def f_ubar_abmp16_nnlo(x: float | np.ndarray) -> float | np.ndarray:
    """Analytic u-bar parametrization from ABMP16 NNLO."""
    return (
        0.0703
        * x ** (-0.415 * (1 + 4.44 * x) * (1 + 0.0373 * np.log(x)))
        * (1 - x) ** 7.75
    )


def f_gluon_herapdf20_nlo(x: float | np.ndarray) -> float | np.ndarray:
    """Analytic gluon parametrization from HERAPDF 2.0 NLO."""
    return 4.34 * x**-0.015 * (1 - x) ** 9.11 - 1.048 * x**-0.167 * (1 - x) ** 25.0


def remove_near_nodes(
    x_samples: np.ndarray,
    nodes: np.ndarray,
    rel_tol: float = 1e-1,
    abs_tol: float | None = None,
) -> np.ndarray:
    """Remove points from *x_samples* that are too close to grid nodes.

    Prevents trivial (on-node) matches that would inflate accuracy metrics.
    """
    x = np.asarray(x_samples)
    sorted_nodes = np.sort(np.asarray(nodes))
    diffs = np.diff(sorted_nodes)
    valid_diffs = diffs[diffs > 0]
    min_spacing = (
        np.min(valid_diffs) if valid_diffs.size > 0 else np.min(np.abs(sorted_nodes))
    )
    if abs_tol is None:
        abs_tol = max(min_spacing * rel_tol, np.finfo(float).eps)
    keep = np.ones_like(x, dtype=bool)
    for n in sorted_nodes:
        keep &= np.abs(x - n) > abs_tol
    return x[keep]


def create_cheby_grid(n_points: int, x_min: float, x_max: float) -> np.ndarray:
    """Chebyshev nodes mapped to [log(x_min), log(x_max)]."""
    u_min, u_max = np.log(x_min), np.log(x_max)
    ts = np.cos(np.pi * np.arange(n_points - 1, -1, -1) / (n_points - 1))
    return np.exp(u_min + (u_max - u_min) * (ts + 1.0) / 2.0)


class InterpolationSetup(BaseModel):
    """Holds the temporary paths and grid parameters created by the fixture."""

    model_config = {"arbitrary_types_allowed": True}

    path: Path
    x_nodes: np.ndarray = Field(repr=False)
    n_cheb_nodes: int = Field(gt=0)
    n_cheb_nodes_b: int = Field(gt=0)


class TestInterpolations:
    @pytest.fixture(scope="class")
    def interpolation_sets(
        self, tmp_path_factory: pytest.TempPathFactory
    ) -> InterpolationSetup:
        """Generate toy NeoPDF sets with each supported interpolation type."""
        tmp_dir = tmp_path_factory.mktemp("interp_sets")

        N_NODES = 100
        N_CHEB = 50
        N_CHEB_B = 25
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
            create_cheby_grid(N_CHEB, 1e-6, 0.2),
            create_cheby_grid(N_CHEB, 0.2, 1.0),
        ]
        self._generate_pdf(
            tmp_dir,
            f"logchebyshev_{N_CHEB}",
            InterpolatorType.LogChebyshev,
            x_cheb,
            q2_values,
        )

        x_cheb_b = [
            create_cheby_grid(N_CHEB_B, 1e-6, 0.2),
            create_cheby_grid(N_CHEB_B, 0.2, 1.0),
        ]
        self._generate_pdf(
            tmp_dir,
            f"logchebyshev_{N_CHEB_B}",
            InterpolatorType.LogChebyshev,
            x_cheb_b,
            q2_values,
        )
        return InterpolationSetup(
            path=tmp_dir,
            x_nodes=x_values,
            n_cheb_nodes=N_CHEB,
            n_cheb_nodes_b=N_CHEB_B,
        )

    def _generate_pdf(
        self,
        path: Path,
        name: str,
        interp_type: InterpolatorType,
        x_vals_list: list[np.ndarray],
        q2_vals: np.ndarray,
    ) -> None:
        x_min = min(xv.min() for xv in x_vals_list)
        x_max = max(xv.max() for xv in x_vals_list)
        meta = self._build_metadata(interp_type, x_min, x_max, q2_vals)
        subgrids = [self._create_subgrid(xv, q2_vals) for xv in x_vals_list]
        compress(
            grids=[GridArray(pids=[-2, 21], subgrids=subgrids)],
            metadata=meta,
            path=str(path / f"interp_test_{name}.neopdf.lz4"),
        )

    def _build_metadata(
        self,
        interp_type: InterpolatorType,
        x_min: float,
        x_max: float,
        q2_vals: np.ndarray,
    ) -> MetaData:
        phys = PhysicsParameters(
            flavor_scheme="fixed",
            order_qcd=2,
            alphas_order_qcd=2,
            m_z=91.1876,
        )
        q_vals = np.geomspace(np.sqrt(q2_vals.min()), np.sqrt(q2_vals.max()), 6)
        return MetaData(
            set_desc="Toy NeoPDF set",
            set_index=123456,
            num_members=1,
            x_min=x_min,
            x_max=x_max,
            q_min=float(np.sqrt(q2_vals.min())),
            q_max=float(np.sqrt(q2_vals.max())),
            xsi_min=0.0,
            xsi_max=0.0,
            delta_min=0.0,
            delta_max=0.0,
            flavors=[-2, 21],
            format="neopdf",
            alphas_q_values=q_vals,
            alphas_vals=np.random.uniform(0.1, 0.2, 6),
            polarised=False,
            set_type=SetType.SpaceLike,
            interpolator_type=interp_type,
            phys_params=phys,
        )

    def _create_subgrid(self, x_vals: np.ndarray, q2_vals: np.ndarray) -> SubGrid:
        pids = [-2, 21]
        xf_funcs = {21: f_gluon_herapdf20_nlo, -2: f_ubar_abmp16_nnlo}
        xq2_flavors = [
            np.outer(xf_funcs[pid](x_vals), np.ones(q2_vals.size)) for pid in pids
        ]
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
    def test_interpolation_visual(
        self, interpolation_sets: InterpolationSetup, pid: int, label: str
    ) -> Figure:
        """Generate relative-error plots and check numerical consistency."""
        p = interpolation_sets.path
        n_cheb = interpolation_sets.n_cheb_nodes
        n_cheb_b = interpolation_sets.n_cheb_nodes_b

        pdf_lin = NEOPDF(str(p / "interp_test_logbilinear.neopdf.lz4"))
        pdf_cub = NEOPDF(str(p / "interp_test_logbicubic.neopdf.lz4"))
        pdf_cheb = NEOPDF(str(p / f"interp_test_logchebyshev_{n_cheb}.neopdf.lz4"))
        pdf_cheb_b = NEOPDF(str(p / f"interp_test_logchebyshev_{n_cheb_b}.neopdf.lz4"))

        x_tests = np.logspace(-6, 0, 250)
        x_clean = remove_near_nodes(x_tests, interpolation_sets.x_nodes)
        q2_test = 1e2

        xf_ref = f_gluon_herapdf20_nlo if pid == 21 else f_ubar_abmp16_nnlo
        ref = xf_ref(x_clean)

        res_lin = np.array([pdf_lin.xfxQ2(pid, x, q2_test) for x in x_clean])
        res_cub = np.array([pdf_cub.xfxQ2(pid, x, q2_test) for x in x_clean])
        res_cheb = np.array([pdf_cheb.xfxQ2(pid, x, q2_test) for x in x_clean])
        res_cheb_b = np.array([pdf_cheb_b.xfxQ2(pid, x, q2_test) for x in x_clean])

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
        shift = 0.20 if label == "ubar" else 0.35
        ax.legend(
            fontsize=10,
            ncols=2,
            loc="center",
            frameon=False,
            bbox_to_anchor=(0.52, shift),
        )
        return fig
