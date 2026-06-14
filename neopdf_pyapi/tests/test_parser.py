from __future__ import annotations

import neopdf.parser as parser  # type: ignore[import-untyped]
import pytest

from typing import Any


@pytest.mark.parametrize("pdf_name", ["NNPDF40_nnlo_as_01180", "MSHT20qed_an3lo"])
def test_lhapdf_set(pdf_name: str) -> None:
    lhapdf_set: Any = parser.LhapdfSet(pdf_name)
    assert lhapdf_set.info() is not None
    assert len(lhapdf_set.members()) > 0
