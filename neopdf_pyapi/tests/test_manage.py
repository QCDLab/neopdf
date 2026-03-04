import neopdf
import pytest

SETNAME = "NNPDF40_nnlo_as_01180"


@pytest.fixture
def manage_data(tmp_path):
    pdf_format = neopdf.manage.PdfSetFormat.Lhapdf
    return neopdf.manage.ManageData(SETNAME, pdf_format)


def test_manage_data(manage_data):
    assert manage_data.is_pdf_installed()
    set_name = manage_data.set_name()
    assert set_name == SETNAME
    set_path = manage_data.set_path()
    data_path = manage_data.data_path()
    assert set_path == f"{data_path}/{SETNAME}"


def test_manage_data_status(manage_data):
    status = manage_data.is_pdf_installed()
    assert isinstance(status, bool)


def test_manage_data_paths(manage_data):
    d_path = manage_data.data_path()
    s_path = manage_data.set_path()
    assert isinstance(d_path, str)
    assert isinstance(s_path, str)
    assert SETNAME in s_path
