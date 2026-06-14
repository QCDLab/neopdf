from __future__ import annotations

import lhapdf  # type: ignore[import-untyped]
import numpy as np
import pytest

from neopdf.pdf import PDF as NeoPDF  # type: ignore[import-untyped]
from pydantic import BaseModel, Field
from typing import Any, Iterator, Protocol


class XQ2Range(BaseModel):
    xmin: float = Field(gt=0)
    xmax: float = Field(gt=0, le=1)
    q2min: float = Field(gt=0)
    q2max: float = Field(gt=0)


class PDFFactory(Protocol):
    """Factory that loads (and caches) a single PDF member by set name."""

    def __call__(self, pdfname: str) -> Any: ...


class PDFsFactory(Protocol):
    """Factory that loads (and caches) all PDF members by set name."""

    def __call__(self, pdfname: str) -> list[Any]: ...


class LazyPDFsFactory(Protocol):
    """Factory that lazily streams PDF members from a compressed archive."""

    def __call__(self, pdfname: str) -> Iterator[Any]: ...


class XQ2PointsFactory(Protocol):
    """Factory that produces log-spaced (x, Q²) grid points from a range."""

    def __call__(self, r: XQ2Range) -> tuple[np.ndarray, np.ndarray]: ...


@pytest.fixture(scope="session")
def neo_pdf() -> PDFFactory:
    cached: dict[str, Any] = {}

    def _init(pdfname: str) -> Any:
        if pdfname not in cached:
            cached[pdfname] = NeoPDF(pdfname)
        return cached[pdfname]

    return _init


@pytest.fixture(scope="session")
def neo_pdfs() -> PDFsFactory:
    cached: dict[str, list[Any]] = {}

    def _init(pdfname: str) -> list[Any]:
        if pdfname not in cached:
            cached[pdfname] = NeoPDF.mkPDFs(pdfname)
        return cached[pdfname]

    return _init


@pytest.fixture(scope="session")
def neo_pdfs_lazy() -> LazyPDFsFactory:
    cached: dict[str, Iterator[Any]] = {}

    def _init(pdfname: str) -> Iterator[Any]:
        if pdfname not in cached:
            cached[pdfname] = NeoPDF.mkPDFs_lazy(pdfname)
        return cached[pdfname]

    return _init


@pytest.fixture(scope="session")
def lha_pdf() -> PDFFactory:
    cached: dict[str, Any] = {}

    def _init(pdfname: str) -> Any:
        if pdfname not in cached:
            cached[pdfname] = lhapdf.mkPDF(pdfname)  # type: ignore[attr-defined]
        return cached[pdfname]

    return _init


@pytest.fixture(scope="session")
def lha_pdfs() -> PDFsFactory:
    cached: dict[str, list[Any]] = {}

    def _init(pdfname: str) -> list[Any]:
        if pdfname not in cached:
            cached[pdfname] = lhapdf.mkPDFs(pdfname)  # type: ignore[attr-defined]
        return cached[pdfname]

    return _init


@pytest.fixture(scope="session")
def xq2_points() -> XQ2PointsFactory:
    def _xq2_points(r: XQ2Range) -> tuple[np.ndarray, np.ndarray]:
        xs = np.geomspace(r.xmin, r.xmax, num=200)
        q2s = np.geomspace(r.q2min, r.q2max, num=200)
        return xs, q2s

    return _xq2_points
