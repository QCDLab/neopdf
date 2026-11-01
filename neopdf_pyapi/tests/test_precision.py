from __future__ import annotations

import math
from decimal import Decimal, localcontext
from itertools import product
from typing import Any

import pytest

from conftest import PDFFactory, XQ2PointsFactory, XQ2Range

PDFNAME = "NNPDF40_nnlo_as_01180"
PIDS = [-5, -3, -1, 0, 1, 2, 4, 21]
# Every `Decimal` operation runs in Python, so its test keeps
# one point in `DECIMAL_STRIDE`.
DECIMAL_STRIDE = 20


def xq2_range(pdf: Any) -> XQ2Range:
    return XQ2Range(
        xmin=pdf.x_min(),
        xmax=pdf.x_max(),
        q2min=pdf.q2_min(),
        q2max=pdf.q2_max(),
    )


class TestPrecisionPDF:
    def test_float_reproduces_xfxq2_and_alphasq2(
        self, neo_pdf: PDFFactory, xq2_points: XQ2PointsFactory
    ) -> None:
        pdf = neo_pdf(PDFNAME)
        view = pdf.with_arithmetic(float, math.log)
        xs, q2s = xq2_points(xq2_range(pdf))
        for pid, x, q2 in product(PIDS, xs, q2s):
            assert view.xfxQ2(pid, x, q2) == pytest.approx(
                pdf.xfxQ2(pid, x, q2), rel=1e-13, abs=1e-300
            )
        for q2 in q2s:
            assert view.alphasQ2(q2) == pytest.approx(pdf.alphasQ2(q2), rel=1e-15)

    def test_decimal_evaluates_in_its_own_arithmetic(
        self, neo_pdf: PDFFactory, xq2_points: XQ2PointsFactory
    ) -> None:
        pdf = neo_pdf(PDFNAME)
        xs, q2s = xq2_points(xq2_range(pdf))
        xs, q2s = xs[::DECIMAL_STRIDE], q2s[1::DECIMAL_STRIDE]
        with localcontext() as ctx:
            ctx.prec = 50
            view = pdf.with_arithmetic(Decimal, Decimal.ln, Decimal.is_finite)
            for pid, x, q2 in product(PIDS, xs, q2s):
                value = view.xfxQ2(pid, Decimal(x), Decimal(q2))
                assert isinstance(value, Decimal)
                assert float(value) == pytest.approx(
                    pdf.xfxQ2(pid, x, q2), rel=1e-12, abs=1e-300
                )
            for q2 in q2s:
                alphas = view.alphasQ2(Decimal(q2))
                assert isinstance(alphas, Decimal)
                assert float(alphas) == pytest.approx(pdf.alphasQ2(q2), rel=1e-15)

    def test_missing_flavours_are_zero(self, neo_pdf: PDFFactory) -> None:
        view = neo_pdf(PDFNAME).with_arithmetic(float, math.log)
        assert view.xfxQ2(6, 0.1, 100.0) == 0.0

    @pytest.mark.parametrize(
        "x, q2, match",
        [
            (1e-12, 100.0, "outside"),
            (0.1, 1.0, "outside"),
            (0.1, math.nan, "nonfinite"),
        ],
    )
    def test_never_extrapolates(
        self, neo_pdf: PDFFactory, x: float, q2: float, match: str
    ) -> None:
        view = neo_pdf(PDFNAME).with_arithmetic(float, math.log)
        with pytest.raises(ValueError, match=match):
            view.xfxQ2(21, x, q2)
        with pytest.raises(ValueError, match="outside"):
            view.alphasQ2(1.0)

    def test_python_exceptions_propagate(self, neo_pdf: PDFFactory) -> None:
        pdf = neo_pdf(PDFNAME)

        def failing_ln(_: float) -> float:
            raise ZeroDivisionError("from ln")

        with pytest.raises(ZeroDivisionError, match="from ln"):
            pdf.with_arithmetic(float, failing_ln)

        # A failed evaluation leaves the view usable.
        view = pdf.with_arithmetic(float, math.log)
        with pytest.raises(TypeError):
            view.xfxQ2(21, "0.1", 100.0)
        assert view.xfxQ2(21, 0.1, 100.0) == pytest.approx(pdf.xfxQ2(21, 0.1, 100.0))

    def test_rejects_unsupported_sets(self, neo_pdf: PDFFactory) -> None:
        pdf = neo_pdf("MAP22_grids_FF_Km_N3LL.neopdf.lz4")
        with pytest.raises(ValueError, match="not supported"):
            pdf.with_arithmetic(float, math.log)
