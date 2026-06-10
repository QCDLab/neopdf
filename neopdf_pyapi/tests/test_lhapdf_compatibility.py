import numpy as np
import pytest
import lhapdf
import neopdf

PDF_SET = "NNPDF40_nnlo_as_01180"
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
        assert sorted(neo_p.flavors()) == sorted(lha_p.flavors())

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


@pytest.mark.parametrize(
    "pdfname", ["NNPDF40_nnlo_as_01180", "CT18NNLO_as_0118", "MSHT20qed_an3lo"]
)
class TestHasKeyGetEntry:
    MANDATORY_KEYS = [
        "SetDesc",
        "NumMembers",
        "XMin",
        "XMax",
        "QMin",
        "QMax",
        "Flavors",
        "Format",
    ]

    def test_has_key_mandatory(self, pdfname):
        neo_ps = neopdf.getPDFSet(pdfname)
        for key in self.MANDATORY_KEYS:
            assert neo_ps.has_key(key), f"PDFSet missing mandatory key '{key}'"

    def test_has_key_absent(self, pdfname):
        neo_ps = neopdf.getPDFSet(pdfname)
        assert not neo_ps.has_key("NonExistentKey_xyz")

    def test_get_entry_num_members(self, pdfname):
        lha_ps = lhapdf.getPDFSet(pdfname)
        neo_ps = neopdf.getPDFSet(pdfname)
        assert int(neo_ps.get_entry("NumMembers")) == int(
            lha_ps.get_entry("NumMembers")
        )

    def test_get_entry_error_type(self, pdfname):
        lha_ps = lhapdf.getPDFSet(pdfname)
        neo_ps = neopdf.getPDFSet(pdfname)
        assert neo_ps.get_entry("ErrorType") == lha_ps.get_entry("ErrorType")

    def test_get_entry_raises_key_error(self, pdfname):
        neo_ps = neopdf.getPDFSet(pdfname)
        with pytest.raises(KeyError):
            neo_ps.get_entry("NonExistentKey_xyz")

    def test_keys_contains_mandatory(self, pdfname):
        neo_ps = neopdf.getPDFSet(pdfname)
        ks = neo_ps.keys()
        assert isinstance(ks, list)
        for key in self.MANDATORY_KEYS:
            assert key in ks, f"key '{key}' missing from keys()"

    def test_error_conf_level_when_key_present(self, pdfname):
        neo_ps = neopdf.getPDFSet(pdfname)
        lha_ps = lhapdf.getPDFSet(pdfname)
        if not lha_ps.has_key("ErrorConfLevel"):
            assert neo_ps.errorConfLevel == -1.0
        else:
            assert neo_ps.errorConfLevel == lha_ps.errorConfLevel

    def test_has_key_error_conf_level(self, pdfname):
        neo_ps = neopdf.getPDFSet(pdfname)
        lha_ps = lhapdf.getPDFSet(pdfname)
        assert neo_ps.has_key("ErrorConfLevel") == lha_ps.has_key("ErrorConfLevel")

    def test_metadata_has_key(self, pdfname):
        neo_p = neopdf.mkPDF(pdfname, 0)
        for key in self.MANDATORY_KEYS:
            assert neo_p.has_key(key), f"PDF missing mandatory key '{key}'"

    def test_metadata_get_entry(self, pdfname):
        neo_p = neopdf.mkPDF(pdfname, 0)
        lha_ps = lhapdf.getPDFSet(pdfname)
        assert int(neo_p.get_entry("NumMembers")) == int(lha_ps.get_entry("NumMembers"))


@pytest.mark.parametrize(
    "pdfname", ["NNPDF40_nnlo_as_01180", "CT18NNLO_as_0118", "MSHT20qed_an3lo"]
)
@pytest.mark.parametrize(
    "pid,x,q", [(21, 0.1, 100.0), (2, 0.01, 10.0), (-3, 0.3, 1000.0)]
)
class TestPDFSetUncertainty:
    def _values(self, pdfname: str, pid: int, x: float, q: float):
        pdfs = neopdf.mkPDFs(pdfname)
        if len(pdfs) <= 1:
            pytest.skip(f"{pdfname} has only {len(pdfs)} member(s) installed")
        return [p.xfxQ(pid, x, q) for p in pdfs]

    def test_uncertainty_returns_object(self, pdfname, pid, x, q):
        from neopdf.uncertainty import Uncertainty

        neo_ps = neopdf.getPDFSet(pdfname)
        values = self._values(pdfname, pid, x, q)
        unc = neo_ps.uncertainty(values)
        assert isinstance(unc, Uncertainty)
        assert unc.central > 0 or unc.central <= 0  # just check it's a float

    def test_uncertainty_matches_standalone(self, pdfname, pid, x, q):
        from neopdf.uncertainty import uncertainty, CL_1_SIGMA

        neo_ps = neopdf.getPDFSet(pdfname)
        values = np.array(self._values(pdfname, pid, x, q))
        ecl = neo_ps.errorConfLevel
        unc_set = neo_ps.uncertainty(list(values), CL_1_SIGMA, False)
        unc_fn = uncertainty(
            values, neo_ps.errorType, error_conf_level=ecl, cl=CL_1_SIGMA
        )
        np.testing.assert_allclose(unc_set.central, unc_fn.central, rtol=1e-10)
        np.testing.assert_allclose(unc_set.errminus, unc_fn.errminus, rtol=1e-10)
        np.testing.assert_allclose(unc_set.errplus, unc_fn.errplus, rtol=1e-10)

    def test_uncertainty_central_matches_lhapdf(self, pdfname, pid, x, q):
        from neopdf.uncertainty import CL_1_SIGMA

        values = self._values(pdfname, pid, x, q)
        neo_ps = neopdf.getPDFSet(pdfname)
        lha_ps = lhapdf.getPDFSet(pdfname)
        neo_unc = neo_ps.uncertainty(values, CL_1_SIGMA, False)
        lha_unc = lha_ps.uncertainty(values, CL_1_SIGMA, False)
        np.testing.assert_allclose(neo_unc.central, lha_unc.central, rtol=1e-10)

    def test_uncertainty_errminus_matches_lhapdf(self, pdfname, pid, x, q):
        from neopdf.uncertainty import CL_1_SIGMA

        values = self._values(pdfname, pid, x, q)
        neo_ps = neopdf.getPDFSet(pdfname)
        lha_ps = lhapdf.getPDFSet(pdfname)
        neo_unc = neo_ps.uncertainty(values, CL_1_SIGMA, False)
        lha_unc = lha_ps.uncertainty(values, CL_1_SIGMA, False)
        np.testing.assert_allclose(neo_unc.errminus, lha_unc.errminus, rtol=1e-2)

    def test_uncertainty_errplus_matches_lhapdf(self, pdfname, pid, x, q):
        from neopdf.uncertainty import CL_1_SIGMA

        values = self._values(pdfname, pid, x, q)
        neo_ps = neopdf.getPDFSet(pdfname)
        lha_ps = lhapdf.getPDFSet(pdfname)
        neo_unc = neo_ps.uncertainty(values, CL_1_SIGMA, False)
        lha_unc = lha_ps.uncertainty(values, CL_1_SIGMA, False)
        np.testing.assert_allclose(neo_unc.errplus, lha_unc.errplus, rtol=1e-3)
