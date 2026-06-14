from __future__ import annotations

from pathlib import Path
import pytest
from typing import Any

import neopdf.parser as parser  # type: ignore[import-untyped]
import neopdf.writer as writer  # type: ignore[import-untyped]


@pytest.mark.parametrize("pdf_name", ["NNPDF40_nnlo_as_01180"])
def test_writer(pdf_name: str, tmp_path: Path) -> None:
    lhapdf_set = parser.LhapdfSet(pdf_name)
    members: list[Any] = lhapdf_set.members()
    metadata: Any = lhapdf_set.info()
    grids = [m[1] for m in members]

    output_path = tmp_path / f"{pdf_name}.neopdf.lz4"
    writer.compress(grids, metadata, str(output_path))

    decompressed_members: list[Any] = writer.decompress(str(output_path))
    assert len(decompressed_members) == len(members)

    extracted_metadata: Any = writer.extract_metadata(str(output_path))
    assert extracted_metadata.set_index() == metadata.set_index()
