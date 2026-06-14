from __future__ import annotations

import os
from pathlib import Path
import numpy as np
import pytest

from conftest import (
    XQ2Range,
    PDFFactory,
    PDFsFactory,
    LazyPDFsFactory,
    XQ2PointsFactory,
)
from itertools import product
from neopdf.pdf import ForcePositive  # type: ignore[import-untyped]


@pytest.mark.parametrize("pdfname", ["NNPDF40_nnlo_as_01180", "MSHT20qed_an3lo"])
class TestPDF:
    def test_mkpdfs(self, neo_pdfs: PDFsFactory, pdfname: str) -> None:
        pdfs = neo_pdfs(pdfname)
        assert len(pdfs) > 0

    def test_pdf_methods(self, neo_pdf: PDFFactory, pdfname: str) -> None:
        neopdf = neo_pdf(pdfname)
        assert len(neopdf.pids()) > 0
        assert len(neopdf.subgrids()) > 0
        assert neopdf.x_min() > 0
        assert neopdf.x_max() > 0
        assert neopdf.q2_min() > 0
        assert neopdf.q2_max() > 0


class TestPDFInterpolations:
    @pytest.mark.parametrize(
        "pdfname",
        [
            "NNPDF40_nnlo_as_01180",
            "MSHT20qed_an3lo",
            "CT18NNLO_as_0118",
            "NNPDFpol20_nnlo_as_01180",
            "nNNPDF30_nlo_as_0118_A56_Z26",
        ],
    )
    @pytest.mark.parametrize("pid", [nf for nf in range(-5, 6) if nf != 0])
    def test_xfxq2(
        self,
        neo_pdf: PDFFactory,
        lha_pdf: PDFFactory,
        xq2_points: XQ2PointsFactory,
        pdfname: str,
        pid: int,
    ) -> None:
        neopdf = neo_pdf(pdfname)
        lhapdf = lha_pdf(pdfname)
        xs, q2s = xq2_points(
            XQ2Range(
                xmin=lhapdf.xMin,
                xmax=lhapdf.xMax,
                q2min=lhapdf.q2Min,
                q2max=lhapdf.q2Max,
            )
        )
        for x, q2 in product(xs, q2s):
            np.testing.assert_equal(neopdf.xfxQ2(pid, x, q2), lhapdf.xfxQ2(pid, x, q2))

    @pytest.mark.parametrize("pdfname", ["NNPDF40_nnlo_as_01180", "MSHT20qed_an3lo"])
    @pytest.mark.parametrize("pid", [21])
    def test_xfxq2s(
        self,
        neo_pdf: PDFFactory,
        lha_pdf: PDFFactory,
        xq2_points: XQ2PointsFactory,
        pdfname: str,
        pid: int,
    ) -> None:
        neopdf = neo_pdf(pdfname)
        lhapdf = lha_pdf(pdfname)
        xs, q2s = xq2_points(
            XQ2Range(
                xmin=lhapdf.xMin,
                xmax=lhapdf.xMax,
                q2min=lhapdf.q2Min,
                q2max=lhapdf.q2Max,
            )
        )
        res = neopdf.xfxQ2s([pid], xs, q2s)
        ref = [lhapdf.xfxQ2(pid, x, q2) for x, q2 in product(xs, q2s)]
        np.testing.assert_equal(res, [ref])

    @pytest.mark.parametrize("pdfname", ["NNPDF40_nnlo_as_01180", "MSHT20qed_an3lo"])
    def test_xfxq2_allpids(
        self,
        neo_pdf: PDFFactory,
        lha_pdf: PDFFactory,
        xq2_points: XQ2PointsFactory,
        pdfname: str,
    ) -> None:
        neopdf = neo_pdf(pdfname)
        lhapdf = lha_pdf(pdfname)
        xs, q2s = xq2_points(
            XQ2Range(
                xmin=lhapdf.xMin,
                xmax=lhapdf.xMax,
                q2min=lhapdf.q2Min,
                q2max=lhapdf.q2Max,
            )
        )
        pids: list[int] = neopdf.pids()
        for x, q2 in product(xs, q2s):
            ref = [lhapdf.xfxQ2(pid, x, q2) for pid in pids]
            np.testing.assert_allclose(neopdf.xfxQ2_allpids(pids, x, q2), ref)
            np.testing.assert_allclose(neopdf.xfxQ2_allpids_ND(pids, [x, q2]), ref)


class TestAlphaSInterpolations:
    @pytest.mark.parametrize("pdfname", ["NNPDF40_nnlo_as_01180", "MSHT20qed_an3lo"])
    def test_alphasQ2(
        self, neo_pdf: PDFFactory, lha_pdf: PDFFactory, pdfname: str
    ) -> None:
        neopdf = neo_pdf(pdfname)
        lhapdf = lha_pdf(pdfname)
        qs: list[float] = neopdf.metadata().alphas_q()
        q2_points = [q * q for q in np.linspace(qs[0], qs[-1], num=300)]
        for q2 in q2_points:
            np.testing.assert_equal(neopdf.alphasQ2(q2), lhapdf.alphasQ2(q2))

    @pytest.mark.parametrize(
        "pdfname",
        ["ABMP16_5_nnlo", "ABMP16als118_5_nnlo", "MSHT20nlo_as_smallrange_nf4"],
    )
    def test_alphasQ2_member(
        self, neo_pdfs: PDFsFactory, lha_pdfs: PDFsFactory, pdfname: str
    ) -> None:
        neopdf = neo_pdfs(pdfname)
        lhapdf = lha_pdfs(pdfname)
        for idx in range(len(neopdf)):
            qs: list[float] = neopdf[idx].metadata().alphas_q()
            q2_points = [q * q for q in np.linspace(qs[0], qs[-1], num=100)]
            for q2 in q2_points:
                np.testing.assert_equal(
                    neopdf[idx].alphasQ2(q2), lhapdf[idx].alphasQ2(q2)
                )


class TestLazyLoader:
    @pytest.mark.parametrize("pdfname", ["NNPDF40_nnlo_as_01180.neopdf.lz4"])
    def test_lazy_loader(self, neo_pdfs_lazy: LazyPDFsFactory, pdfname: str) -> None:
        for pdf in neo_pdfs_lazy(pdfname):
            assert isinstance(pdf.xfxQ2(21, 1e-5, 1e2), float)


class TestLHAID:
    def test_mkpdf_lhaid_base(self, neo_pdf: PDFFactory) -> None:
        from neopdf.pdf import PDF  # type: ignore[import-untyped]

        # NNPDF40_nnlo_as_01180 has base LHAID 331100; member 0 = LHAID 331100
        pdf_by_id = PDF.mkPDF_lhaid(331100)
        pdf_by_name = neo_pdf("NNPDF40_nnlo_as_01180")
        x, q2, pid = 1e-4, 100.0, 21
        np.testing.assert_equal(
            pdf_by_id.xfxQ2(pid, x, q2), pdf_by_name.xfxQ2(pid, x, q2)
        )
        np.testing.assert_equal(pdf_by_id.alphasQ2(q2), pdf_by_name.alphasQ2(q2))

    def test_mkpdf_lhaid_member_offset(self, neo_pdfs: PDFsFactory) -> None:
        from neopdf.pdf import PDF  # type: ignore[import-untyped]

        # ABMP16als118_5_nnlo owns range [42780, 42810); member 10 = LHAID 42790
        pdf_by_id = PDF.mkPDF_lhaid(42790)
        pdf_by_name = neo_pdfs("ABMP16als118_5_nnlo")[10]
        x, q2, pid = 1e-3, 10.0, 1
        np.testing.assert_equal(
            pdf_by_id.xfxQ2(pid, x, q2), pdf_by_name.xfxQ2(pid, x, q2)
        )
        np.testing.assert_equal(pdf_by_id.alphasQ2(q2), pdf_by_name.alphasQ2(q2))


class TestLoadFromFile:
    def test_mkpdf_lhapdf_by_file(self) -> None:
        from neopdf.pdf import PDF  # type: ignore[import-untyped]

        neopdf_data_path = os.environ.get("NEOPDF_DATA_PATH")
        if not neopdf_data_path:
            pytest.skip(
                "NEOPDF_DATA_PATH environment variable not set."
            )  # ty: ignore[too-many-positional-arguments]

        path = (
            Path(neopdf_data_path)
            / "NNPDF40_nnlo_as_01180"
            / "NNPDF40_nnlo_as_01180_0000.dat"
        )
        if not path.exists():
            pytest.skip(
                f"Data file not found at {path}"
            )  # ty: ignore[too-many-positional-arguments]

        xf = PDF.mkPDF_lhapdf_file(str(path)).xfxQ2(21, 1e-9, 1.65 * 1.65)
        np.testing.assert_allclose(xf, 0.14844111, rtol=1e-8)


class TestForcePositive:
    def test_force_positive(self, neo_pdf: PDFFactory) -> None:
        neopdf = neo_pdf("nNNPDF30_nlo_as_0118_A56_Z26")
        assert neopdf.is_force_positive() == ForcePositive.NoClipping
        assert neopdf.xfxQ2(21, 0.9, 1e2) < 0.0

        neopdf.set_force_positive(ForcePositive.ClipNegative)
        assert neopdf.is_force_positive() == ForcePositive.ClipNegative
        assert neopdf.xfxQ2(21, 0.9, 1e2) == 0.0

        neopdf.set_force_positive(ForcePositive.ClipSmall)
        assert neopdf.is_force_positive() == ForcePositive.ClipSmall
        assert neopdf.xfxQ2(21, 0.9, 1e2) == 1e-10
