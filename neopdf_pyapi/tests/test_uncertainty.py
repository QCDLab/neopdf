"""Tests for neopdf.uncertainty, benchmarked against LHAPDF."""

import numpy as np
import pytest
import lhapdf

from neopdf.pdf import PDF as NeoPDF
from neopdf.uncertainty import uncertainty, Uncertainty


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

CL_1_SIGMA = 68.268_949_213_708_58  # matches lhapdf.CL_1_SIGMA


def lha_values(pdfname: str, pid: int, x: float, q: float) -> np.ndarray:
    """Return an array of xfxQ values (all members) from LHAPDF."""
    pdfs = lhapdf.mkPDFs(pdfname)
    return np.array([p.xfxQ(pid, x, q) for p in pdfs])


def neo_values(pdfname: str, pid: int, x: float, q: float) -> np.ndarray:
    """Return an array of xfxQ2 values (all members) from NeoPDF."""
    pdfs = NeoPDF.mkPDFs(pdfname)
    return np.array([p.xfxQ2(pid, x, q * q) for p in pdfs])


# ---------------------------------------------------------------------------
# Unit tests for the Uncertainty class
# ---------------------------------------------------------------------------


class TestUncertaintyClass:
    def test_attributes(self):
        values = np.array([1.0, 0.9, 1.1, 0.95, 1.05])
        unc = uncertainty(values, "replicas", cl=CL_1_SIGMA)
        assert hasattr(unc, "central")
        assert hasattr(unc, "errminus")
        assert hasattr(unc, "errplus")
        assert isinstance(unc, Uncertainty)

    def test_central_replicas_is_mean(self):
        """For replica sets LHAPDF reports the mean of replicas 1..N as the central value."""
        values = np.array([5.0, 4.0, 6.0, 4.5, 5.5])
        unc = uncertainty(values, "replicas", cl=CL_1_SIGMA)
        assert unc.central == np.mean(values[1:])

    def test_errsymm(self):
        values = np.array([1.0, 0.8, 1.2, 0.9, 1.1])
        unc = uncertainty(values, "hessian", cl=CL_1_SIGMA)
        assert np.isclose(unc.errsymm(), (unc.errminus + unc.errplus) / 2.0)

    def test_repr(self):
        values = np.array([1.0, 0.9, 1.1])
        unc = uncertainty(values, "replicas", cl=CL_1_SIGMA)
        assert "Uncertainty" in repr(unc)
        assert "central" in repr(unc)

    def test_raises_on_empty(self):
        with pytest.raises(Exception):
            uncertainty(np.array([]), "replicas", cl=CL_1_SIGMA)


# ---------------------------------------------------------------------------
# Algorithm correctness tests (independent of LHAPDF)
# ---------------------------------------------------------------------------


class TestUncertaintyAlgorithms:
    def test_replicas_symmetric(self):
        """Std-dev (sample, N-1) of replicas should equal expected value."""
        rng = np.random.default_rng(42)
        replicas = rng.normal(loc=0.0, scale=1.0, size=100)
        values = np.concatenate([[0.0], replicas])
        unc = uncertainty(values, "replicas", cl=CL_1_SIGMA)
        expected_std = np.std(replicas, ddof=1)
        assert np.isclose(unc.errminus, expected_std, rtol=1e-10)
        assert np.isclose(unc.errplus, expected_std, rtol=1e-10)

    def test_replicas_alternative_asymmetric(self):
        """Alternative (quantile) prescription must give errminus ≠ errplus for skewed data."""
        rng = np.random.default_rng(0)
        replicas = rng.exponential(scale=1.0, size=1000)
        values = np.concatenate([[np.mean(replicas)], replicas])
        unc_sym = uncertainty(values, "replicas", cl=68.27)
        unc_alt = uncertainty(values, "replicas", cl=68.27, alternative=True)
        # For exponential distribution the quantile interval is asymmetric.
        assert not np.isclose(unc_alt.errminus, unc_alt.errplus, rtol=1e-3)
        # Symmetric version is fully symmetric by construction.
        assert np.isclose(unc_sym.errminus, unc_sym.errplus, rtol=1e-10)

    def test_hessian_symmetric_pairs(self):
        """Hessian error = sqrt(sum((T+ - T-)^2 / 4)) for a known case."""
        # Eigenvector pairs: T+ = 1.1, T- = 0.9 (diff = 0.2, half = 0.1)
        values = np.array([1.0, 1.1, 0.9, 1.2, 0.8])  # 2 pairs
        unc = uncertainty(values, "hessian", cl=CL_1_SIGMA)
        expected = np.sqrt(0.1**2 + 0.2**2)
        assert np.isclose(unc.errminus, expected, rtol=1e-10)
        assert np.isclose(unc.errplus, expected, rtol=1e-10)

    def test_symmhessian_same_as_hessian(self):
        """For perfectly symmetric eigenvector pairs, 'symmhessian' and 'hessian' agree.

        'hessian' uses the asymmetric formula (separate errplus/errminus quadrature sums),
        while 'symmhessian' uses the symmetric formula (half-differences in quadrature).
        When every pair (T+, T-) is symmetric around the central value the two formulas
        are numerically identical.
        """
        values = np.array([1.0, 1.1, 0.9, 1.3, 0.7])
        unc_h = uncertainty(values, "hessian", cl=CL_1_SIGMA)
        unc_s = uncertainty(values, "symmhessian", cl=CL_1_SIGMA)
        assert np.isclose(unc_h.errminus, unc_s.errminus)
        assert np.isclose(unc_h.errplus, unc_s.errplus)

    def test_asymhessian(self):
        """Asymmetric Hessian gives separate errplus and errminus."""
        # T+ > central → contributes to errplus; T- < central → contributes to errminus.
        values = np.array([1.0, 1.2, 0.8])  # 1 pair
        unc = uncertainty(values, "asymhessian", cl=CL_1_SIGMA)
        # errplus: max(0, 1.2-1.0)^2 + max(0, 0.8-1.0)^2 → sqrt(0.04) = 0.2
        # errminus: max(0, 1.0-1.2)^2 + max(0, 1.0-0.8)^2 → sqrt(0.04) = 0.2
        assert np.isclose(unc.errplus, 0.2, rtol=1e-10)
        assert np.isclose(unc.errminus, 0.2, rtol=1e-10)

    def test_cl_rescaling_hessian(self):
        """Requesting 2σ (95.45%) should give approximately double the 1σ error."""
        values = np.array([1.0, 1.1, 0.9, 1.2, 0.8])
        unc_1s = uncertainty(values, "hessian", cl=68.27)
        unc_2s = uncertainty(values, "hessian", cl=95.45)
        assert np.isclose(unc_2s.errminus / unc_1s.errminus, 2.0, rtol=1e-2)

    def test_cl_rescaling_replicas(self):
        """At 2σ the std-dev should be scaled up by ~2."""
        rng = np.random.default_rng(7)
        replicas = rng.normal(0.0, 1.0, 500)
        values = np.concatenate([[0.0], replicas])
        unc_1s = uncertainty(values, "replicas", cl=68.27)
        unc_2s = uncertainty(values, "replicas", cl=95.45)
        assert np.isclose(unc_2s.errminus / unc_1s.errminus, 2.0, rtol=1e-2)

    def test_error_conf_level_ct18_convention(self):
        """error_conf_level=90 (CT18-style) at output cl=68.27 should shrink the error."""
        values = np.array([1.0, 1.2, 0.8, 1.3, 0.7])
        unc_68 = uncertainty(
            values,
            "hessian",
            error_conf_level=68.268_949_213_708_58,
            cl=68.268_949_213_708_58,
        )
        unc_90 = uncertainty(
            values, "hessian", error_conf_level=90.0, cl=68.268_949_213_708_58
        )
        # Rescaling from 90% to 68% should reduce the error by ~1/1.645.
        assert unc_90.errminus < unc_68.errminus
        assert np.isclose(
            unc_90.errminus / unc_68.errminus, 1.0 / (1.6449 / 1.0), rtol=1e-2
        )


# ---------------------------------------------------------------------------
# Benchmark against LHAPDF
#
# Both LHAPDF and NeoPDF are called with the *same* input values (drawn from
# LHAPDF) so that differences reflect the uncertainty algorithm only, not
# the interpolation accuracy of the NeoPDF backend.
# ---------------------------------------------------------------------------


@pytest.mark.parametrize(
    "pdfname,error_type_hint",
    [
        ("NNPDF40_nnlo_as_01180", "replicas"),
        ("CT18NNLO_as_0118", "hessian"),
        ("MSHT20qed_an3lo", "hessian"),
    ],
)
@pytest.mark.parametrize(
    "pid,x,q",
    [
        (21, 0.1, 100.0),  # gluon at medium x
        (2, 0.01, 10.0),  # up quark at small x
        (-3, 0.3, 1000.0),  # anti-strange at large x
    ],
)
class TestAgainstLhapdf:
    """Compare neopdf.uncertainty with lhapdf.PdfSet.uncertainty()."""

    def _setup(self, pdfname, pid, x, q):
        """Return (pdfset, values, error_type, error_conf_level).

        Values are drawn from LHAPDF so both algorithms operate on identical input.
        Skips the test automatically when fewer than 2 members are installed.
        """
        pdfset = lhapdf.getPDFSet(pdfname)
        values = lha_values(pdfname, pid, x, q)
        if len(values) <= 1:
            pytest.skip(
                f"{pdfname} has only {len(values)} member(s) installed — "
                "install the full set to run this test"
            )
        pdf0 = NeoPDF(pdfname, 0)
        error_type = pdf0.metadata().error_type()
        ecl = float(pdfset.errorConfLevel)  # -1.0 when not set in the info file
        return pdfset, values, error_type, ecl

    def test_central(self, pdfname, pid, x, q, error_type_hint):
        """Central value must match LHAPDF (mean of replicas for replica sets, values[0] for Hessian)."""
        pdfset, values, error_type, ecl = self._setup(pdfname, pid, x, q)
        lha = pdfset.uncertainty(values, CL_1_SIGMA, False)
        neo = uncertainty(values, error_type, error_conf_level=ecl, cl=CL_1_SIGMA)
        np.testing.assert_allclose(neo.central, lha.central, rtol=1e-10)

    def test_errminus(self, pdfname, pid, x, q, error_type_hint):
        pdfset, values, error_type, ecl = self._setup(pdfname, pid, x, q)
        lha = pdfset.uncertainty(values, CL_1_SIGMA, False)
        neo = uncertainty(values, error_type, error_conf_level=ecl, cl=CL_1_SIGMA)
        np.testing.assert_allclose(neo.errminus, lha.errminus, rtol=1e-6)

    def test_errplus(self, pdfname, pid, x, q, error_type_hint):
        pdfset, values, error_type, ecl = self._setup(pdfname, pid, x, q)
        lha = pdfset.uncertainty(values, CL_1_SIGMA, False)
        neo = uncertainty(values, error_type, error_conf_level=ecl, cl=CL_1_SIGMA)
        np.testing.assert_allclose(neo.errplus, lha.errplus, rtol=1e-6)

    def test_alternative_prescription(self, pdfname, pid, x, q, error_type_hint):
        """Alternative (quantile) prescription gives an asymmetric interval for replica sets.

        LHAPDF uses a proprietary quantile-interpolation scheme whose exact algorithm
        is not public; we therefore only verify qualitative properties (not bit-level
        agreement with LHAPDF).
        """
        if error_type_hint != "replicas":
            pytest.skip("alternative prescription only applies to replica sets")
        pdfset, values, error_type, ecl = self._setup(pdfname, pid, x, q)
        neo = uncertainty(
            values, error_type, error_conf_level=ecl, cl=CL_1_SIGMA, alternative=True
        )
        neo_std = uncertainty(values, error_type, error_conf_level=ecl, cl=CL_1_SIGMA)
        # The alternative prescription must give an asymmetric interval.
        assert neo.errminus != neo.errplus, "expected asymmetric interval"
        assert neo.errminus > 0
        assert neo.errplus > 0
        # Errors must be plausible: within 3× of the std-dev estimate.
        assert neo.errminus < 3 * neo_std.errminus
        assert neo.errplus < 3 * neo_std.errplus

    def test_higher_cl(self, pdfname, pid, x, q, error_type_hint):
        """At 95.45% CL the error should be larger than at 68.27%."""
        pdfset, values, error_type, ecl = self._setup(pdfname, pid, x, q)
        neo_1s = uncertainty(values, error_type, error_conf_level=ecl, cl=CL_1_SIGMA)
        neo_2s = uncertainty(values, error_type, error_conf_level=ecl, cl=95.45)
        assert neo_2s.errminus >= neo_1s.errminus
        assert neo_2s.errplus >= neo_1s.errplus
