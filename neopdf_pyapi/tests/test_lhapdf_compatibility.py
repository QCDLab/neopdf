import numpy as np
import pytest
import lhapdf
import neopdf

PDF_SET = "CT18NLO"
MEMBER = 7


@pytest.fixture(scope="module")
def neo_ps():
    return neopdf.getPDFSet(PDF_SET)


@pytest.fixture(scope="module")
def lha_ps():
    return lhapdf.getPDFSet(PDF_SET)


@pytest.fixture(scope="module")
def neo_p():
    return neopdf.mkPDF(PDF_SET, MEMBER)


@pytest.fixture(scope="module")
def lha_p():
    return lhapdf.mkPDF(PDF_SET, MEMBER)


class TestModuleLevelFunctions:
    def test_verbosity_roundtrip(self):
        neopdf.setVerbosity(3)
        assert neopdf.verbosity() == 3
        neopdf.setVerbosity(0)
        assert neopdf.verbosity() == 0

    def test_get_pdf_set_returns_pdfset(self):
        ps = neopdf.getPDFSet(PDF_SET)
        assert isinstance(ps, neopdf.PDFSet)

    def test_mk_pdf_returns_pdf(self):
        p = neopdf.mkPDF(PDF_SET, 0)
        assert isinstance(p, neopdf.pdf.PDF)

    def test_mk_pdf_default_member(self):
        p = neopdf.mkPDF(PDF_SET)
        assert p.memberID == 0

    def test_mk_pdfs_length_matches_lhapdf(self):
        neo = neopdf.mkPDFs(PDF_SET)
        lha = lhapdf.mkPDFs(PDF_SET)
        assert len(neo) == len(lha)

    def test_available_pdf_sets_is_list_of_str(self):
        sets = neopdf.availablePDFSets()
        assert isinstance(sets, list)
        assert all(isinstance(s, str) for s in sets)

    def test_available_pdf_sets_contains_test_set(self):
        assert PDF_SET in neopdf.availablePDFSets()

    def test_paths_returns_list_of_str(self):
        ps = neopdf.paths()
        assert isinstance(ps, list)
        assert len(ps) >= 1
        assert all(isinstance(p, str) for p in ps)

    def test_set_paths_no_op(self):
        neopdf.setPaths(["/tmp/fake"])

    def test_paths_append_no_op(self):
        neopdf.pathsAppend("/tmp/fake")

    def test_paths_prepend_no_op(self):
        neopdf.pathsPrepend("/tmp/fake")


class TestPDFSetClass:
    def test_name(self, neo_ps, lha_ps):
        assert neo_ps.name == lha_ps.name

    def test_size(self, neo_ps, lha_ps):
        assert neo_ps.size == lha_ps.size

    def test_lhapdf_id(self, neo_ps, lha_ps):
        assert neo_ps.lhapdfID == lha_ps.lhapdfID

    def test_error_type(self, neo_ps, lha_ps):
        assert neo_ps.errorType == lha_ps.errorType

    def test_description_non_empty(self, neo_ps):
        assert len(neo_ps.description) > 0

    def test_len(self, neo_ps, lha_ps):
        assert len(neo_ps) == len(lha_ps)

    def test_mkpdf_member_id(self, neo_ps, lha_ps):
        neo_p = neo_ps.mkPDF(MEMBER)
        lha_p = lha_ps.mkPDF(MEMBER)
        assert neo_p.memberID == lha_p.memberID

    def test_mkpdf_lhapdf_id(self, neo_ps, lha_ps):
        neo_p = neo_ps.mkPDF(MEMBER)
        lha_p = lha_ps.mkPDF(MEMBER)
        assert neo_p.lhapdfID == lha_p.lhapdfID

    def test_mkpdfs_count(self, neo_ps, lha_ps):
        neo_members = neo_ps.mkPDFs()
        lha_members = lha_ps.mkPDFs()
        assert len(neo_members) == len(lha_members)

    def test_mkpdfs_member_ids(self, neo_ps):
        members = neo_ps.mkPDFs()
        for i, m in enumerate(members):
            assert m.memberID == i


@pytest.mark.parametrize(
    "x,q,pid",
    [(1e-4, 10.0, 21), (0.1, 100.0, 1), (0.5, 1000.0, -5)],
)
class TestPDFXfxQ:
    def test_xfxq_equals_xfxq2(self, x, q, pid, neo_p):
        """xfxQ(id, x, Q) must equal xfxQ2(id, x, Q²)."""
        assert neo_p.xfxQ(pid, x, q) == neo_p.xfxQ2(pid, x, q * q)

    def test_xfxq_matches_lhapdf(self, x, q, pid, neo_p, lha_p):
        np.testing.assert_equal(neo_p.xfxQ(pid, x, q), lha_p.xfxQ(pid, x, q))


class TestPDFAlphaS:
    @pytest.mark.parametrize("q", [2.0, 10.0, 91.2, 1000.0])
    def test_alphasq_equals_alphasq2(self, q, neo_p):
        assert neo_p.alphasQ(q) == neo_p.alphasQ2(q * q)

    @pytest.mark.parametrize("q", [2.0, 10.0, 91.2, 1000.0])
    def test_alphasq_matches_lhapdf(self, q, neo_p, lha_p):
        np.testing.assert_equal(neo_p.alphasQ(q), lha_p.alphasQ(q))


class TestPDFIdentity:
    def test_member_id(self, neo_p, lha_p):
        assert neo_p.memberID == lha_p.memberID

    def test_lhapdf_id(self, neo_p, lha_p):
        assert neo_p.lhapdfID == lha_p.lhapdfID

    def test_order_qcd(self, neo_p, lha_p):
        assert neo_p.orderQCD == lha_p.orderQCD

    def test_quark_mass_charm(self, neo_p, lha_p):
        np.testing.assert_allclose(neo_p.quarkMass(4), lha_p.quarkMass(4))

    def test_quark_mass_bottom(self, neo_p, lha_p):
        np.testing.assert_allclose(neo_p.quarkMass(5), lha_p.quarkMass(5))

    def test_quark_threshold_equals_mass(self, neo_p):
        assert neo_p.quarkThreshold(4) == neo_p.quarkMass(4)
        assert neo_p.quarkThreshold(5) == neo_p.quarkMass(5)

    def test_x_max(self, neo_p, lha_p):
        np.testing.assert_allclose(neo_p.xMax, lha_p.xMax)

    def test_q2_min(self, neo_p, lha_p):
        np.testing.assert_allclose(neo_p.q2Min, lha_p.q2Min)

    def test_q2_max(self, neo_p, lha_p):
        np.testing.assert_allclose(neo_p.q2Max, lha_p.q2Max)


class TestPDFFlavors:
    def test_flavors_match_lhapdf(self, neo_p, lha_p):
        assert neo_p.flavors() == lha_p.flavors()

    def test_has_flavor_gluon(self, neo_p, lha_p):
        assert neo_p.hasFlavor(21) == lha_p.hasFlavor(21)

    def test_has_flavor_unknown(self, neo_p, lha_p):
        assert neo_p.hasFlavor(99) == lha_p.hasFlavor(99)

    @pytest.mark.parametrize("pid", [-5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 21])
    def test_has_flavor_all_standard(self, pid, neo_p):
        assert neo_p.hasFlavor(pid) is True


class TestPDFRangeChecks:
    @pytest.mark.parametrize(
        "x,q2,expected",
        [
            (1e-4, 100.0, True),
            (0.5, 1e4, True),
            (2.0, 100.0, False),
            (1e-4, 1e13, False),
        ],
    )
    def test_in_range_xq2_matches_lhapdf(self, x, q2, expected, neo_p, lha_p):
        assert neo_p.inRangeXQ2(x, q2) == lha_p.inRangeXQ2(x, q2)

    def test_in_range_xq_matches_lhapdf(self, neo_p, lha_p):
        for x, q in [(1e-4, 10.0), (0.5, 100.0), (2.0, 10.0)]:
            assert neo_p.inRangeXQ(x, q) == lha_p.inRangeXQ(x, q)

    def test_in_range_x_matches_lhapdf(self, neo_p, lha_p):
        for x in [1e-6, 0.1, 0.9, 1.5]:
            assert neo_p.inRangeX(x) == lha_p.inRangeX(x)

    def test_in_range_q2_matches_lhapdf(self, neo_p, lha_p):
        for q2 in [1.0, 100.0, 1e10, 1e13]:
            assert neo_p.inRangeQ2(q2) == lha_p.inRangeQ2(q2)

    def test_in_range_q_matches_lhapdf(self, neo_p, lha_p):
        for q in [1.0, 10.0, 1e5, 1e7]:
            assert neo_p.inRangeQ(q) == lha_p.inRangeQ(q)


class TestPDFSetProperty:
    def test_set_property_not_none(self, neo_p):
        assert neo_p.set is not None

    def test_set_property_is_pdfset(self, neo_p):
        assert isinstance(neo_p.set, neopdf.PDFSet)

    def test_set_property_name(self, neo_p):
        assert neo_p.set.name == PDF_SET

    def test_set_property_none_when_loaded_by_lhaid(self):
        from neopdf.pdf import PDF

        p = PDF.mkPDF_lhaid(14400)
        assert p.set is None


def _compute_xfxQ2_central(backend, pdfname, pid, x, q2):
    pdf = backend.mkPDF(pdfname, 0)
    return pdf.xfxQ2(pid, x, q2)


def _collect_all_member_values(backend, pdfname, pid, x, q2):
    pdfs = backend.mkPDFs(pdfname)
    return [p.xfxQ2(pid, x, q2) for p in pdfs]


def _inspect_set(backend, pdfname):
    ps = backend.getPDFSet(pdfname)
    return {
        "name": ps.name,
        "size": ps.size,
        "lhapdfID": ps.lhapdfID,
        "errorType": ps.errorType,
    }


def _check_kinematics(backend, pdfname, x, q):
    pdf = backend.mkPDF(pdfname, 0)
    return pdf.inRangeXQ(x, q)


@pytest.mark.parametrize(
    "pid,x,q2",
    [(21, 1e-4, 100.0), (1, 0.1, 1e4), (-3, 0.3, 1e6)],
)
class TestBackendSwap:
    def test_central_member_xfxq2(self, pid, x, q2):
        lha_val = _compute_xfxQ2_central(lhapdf, PDF_SET, pid, x, q2)
        neo_val = _compute_xfxQ2_central(neopdf, PDF_SET, pid, x, q2)
        np.testing.assert_equal(neo_val, lha_val)

    def test_all_member_values(self, pid, x, q2):
        lha_vals = _collect_all_member_values(lhapdf, PDF_SET, pid, x, q2)
        neo_vals = _collect_all_member_values(neopdf, PDF_SET, pid, x, q2)
        np.testing.assert_array_equal(neo_vals, lha_vals)


class TestBackendSwapMetadata:
    def test_set_metadata(self):
        lha_meta = _inspect_set(lhapdf, PDF_SET)
        neo_meta = _inspect_set(neopdf, PDF_SET)
        assert neo_meta == lha_meta

    def test_kinematics_in_range(self):
        for x, q in [(1e-3, 10.0), (0.5, 300.0), (2.0, 10.0)]:
            assert _check_kinematics(
                neopdf,
                PDF_SET,
                x,
                q,
            ) == _check_kinematics(lhapdf, PDF_SET, x, q)
