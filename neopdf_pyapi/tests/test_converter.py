from __future__ import annotations

import os
import neopdf
import pytest

from pathlib import Path


@pytest.fixture
def create_dummy_files(tmp_path: Path) -> str:
    dummy_content = """
    ---
    Some contents
    ---
    """
    lhapdf_dir = tmp_path / "LHAPDF"
    lhapdf_dir.mkdir()
    base_name = "NNPDF40_nnlo_as_01180"
    for filename in [f"{base_name}.info", f"{base_name}_0001.dat"]:
        (lhapdf_dir / filename).write_text(dummy_content)
    return str(tmp_path)


def test_convert_lhapdf(create_dummy_files: str) -> None:
    os.environ["LHAPDF_DATA_PATH"] = create_dummy_files
    output_path = os.path.join(create_dummy_files, "test.neopdf.lz4")
    neopdf.converter.convert_lhapdf("NNPDF40_nnlo_as_01180", output_path)  # type: ignore[attr-defined]
    assert os.path.exists(output_path)
