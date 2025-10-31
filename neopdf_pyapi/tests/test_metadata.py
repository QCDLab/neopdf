from math import sqrt
import neopdf.metadata as metadata
import numpy as np
import pytest


class TestMetaData:
    @pytest.mark.parametrize("pdfname", ["NNPDF40_nnlo_as_01180", "MSHT20qed_an3lo"])
    def test_metadata_fields(self, neo_pdf, lha_pdf, pdfname):
        neopdf = neo_pdf(pdfname)
        lhapdf = lha_pdf(pdfname)

        neopdf_meta = neopdf.metadata()
        np.testing.assert_equal(neopdf_meta.x_min(), lhapdf.xMin)
        np.testing.assert_equal(neopdf_meta.x_max(), lhapdf.xMax)
        np.testing.assert_equal(neopdf_meta.q_min(), sqrt(lhapdf.q2Min))
        np.testing.assert_equal(neopdf_meta.q_max(), sqrt(lhapdf.q2Max))
        np.testing.assert_equal(neopdf_meta.set_index(), lhapdf.lhapdfID)

    def test_metadata_creation(self):
        phys_params = metadata.PhysicsParameters(
            flavor_scheme="test_scheme",
            order_qcd=2,
            alphas_order_qcd=2,
            m_w=80.4,
            m_z=91.2,
            m_up=0.0022,
            m_down=0.0047,
            m_strange=0.096,
            m_charm=1.27,
            m_bottom=4.18,
            m_top=172.9,
        )

        meta = metadata.MetaData(
            set_desc="test_desc",
            set_index=1,
            num_members=1,
            x_min=1e-5,
            x_max=1.0,
            q_min=1.65,
            q_max=1.0e4,
            xsi_min=0.0,
            xsi_max=0.0,
            delta_min=0.0,
            delta_max=0.0,
            flavors=[21, 1, 2, 3, 4, -1, -2, -3, -4],
            format="test_format",
            alphas_q_values=[1.0, 2.0],
            alphas_vals=[0.118, 0.110],
            polarised=False,
            set_type=metadata.SetType.SpaceLike,
            interpolator_type=metadata.InterpolatorType.LogBicubic,
            error_type="replicas",
            hadron_pid=2212,
            phys_params=phys_params,
        )

        assert meta.set_desc() == "test_desc"
        assert meta.set_index() == 1
        assert meta.number_sets() == 1
        assert meta.x_min() == 1e-5
        assert meta.x_max() == 1.0
        assert meta.q_min() == 1.65
        assert meta.q_max() == 1.0e4
        assert meta.xi_min() == 0.0
        assert meta.xi_max() == 0.0
        assert meta.delta_min() == 0.0
        assert meta.delta_max() == 0.0
        assert meta.pids() == [21, 1, 2, 3, 4, -1, -2, -3, -4]
        assert meta.format() == "test_format"
        assert meta.alphas_q() == [1.0, 2.0]
        assert meta.alphas_values() == [0.118, 0.110]
        assert not meta.is_polarised()
        assert meta.set_type() == metadata.SetType.SpaceLike
        assert meta.interpolator_type() == metadata.InterpolatorType.LogBicubic
        assert meta.error_type() == "replicas"
        assert meta.hadron_pid() == 2212
