from __future__ import annotations

from typing import Any

import neopdf
import pytest

SETNAME = "NNPDF40_nnlo_as_01180"


@pytest.fixture
def manage_data() -> Any:
    pdf_format = neopdf.manage.PdfSetFormat.Lhapdf  # type: ignore[attr-defined]
    return neopdf.manage.ManageData(SETNAME, pdf_format)  # type: ignore[attr-defined]


def test_manage_data(manage_data: Any) -> None:
    assert manage_data.is_pdf_installed()
    assert manage_data.set_name() == SETNAME
    assert manage_data.set_path() == f"{manage_data.data_path()}/{SETNAME}"


def test_manage_data_status(manage_data: Any) -> None:
    assert isinstance(manage_data.is_pdf_installed(), bool)


def test_manage_data_paths(manage_data: Any) -> None:
    d_path: str = manage_data.data_path()
    s_path: str = manage_data.set_path()
    assert isinstance(d_path, str)
    assert isinstance(s_path, str)
    assert SETNAME in s_path
