#include <NeoPDF.hpp>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <vector>

using namespace neopdf;

const double TOLERANCE = 1e-10; // Increased tolerance for synthetic data

int main() {
    const char* pdfname = "NNPDF40_nnlo_as_01180";
    // Load all PDF members
    NeoPDFs neo_pdfs(pdfname);
    if (neo_pdfs.size() == 0) {
        std::cerr << "Failed to load any PDF members!\n";
        return 1;
    }
    std::cout << "Loaded " << neo_pdfs.size() << " PDF members\n";

    // Get the first PDF as a reference for metadata
    NeoPDF& ref_pdf = neo_pdfs[0];

    // Extract the PID values of the PDF set
    auto pids = ref_pdf.pids();

    // Extract the number of subgrids
    std::size_t num_subgrids = ref_pdf.num_subgrids();

    // Define synthetic xi and delta values
    std::vector<double> xis = {0.0};
    std::vector<double> deltas = {0.0};

    // Create a grid writer
    GridWriter writer;

    // For each member, build a grid
    for (size_t m = 0; m < neo_pdfs.size(); ++m) {
        NeoPDF& pdf = neo_pdfs[m];

        // Start a new grid for the current member
        writer.new_grid();

        // Loop over the Subgrids
        for (std::size_t subgrid_idx = 0; subgrid_idx < num_subgrids; ++subgrid_idx) {
            // Extract base parameters from the original PDF (x, q2, nucleons, alphas, kts)
            auto xs = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_MOMENTUM, subgrid_idx);
            auto q2s = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_SCALE, subgrid_idx);
            auto alphas = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_ALPHAS, subgrid_idx);
            auto nucleons = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_NUCLEONS, subgrid_idx);
            auto kts = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_KT, subgrid_idx);

            // Compute grid_data: [q2s][xs][flavors], instead of [nucleons][alphas][xis][deltas][q2s][xs][flavors]
            // NOTE: This assumes that there is no 'A', `alphas`, `xis`, and `deltas` dependence.
            assert(alphas.size() == 1);
            assert(kts.size() == 1);
            assert(xis.size() == 1);
            assert(deltas.size() == 1);
            assert(nucleons.size() == 1);
            std::vector<double> grid_data;
            for (double x : xs) {
                for (double q2 : q2s) {
                    for (int pid : pids) {
                        double val = pdf.xfxQ2(pid, x, q2);
                        grid_data.push_back(val);
                    }
                }
            }

            // Add subgrid using the v2 function
            writer.add_subgrid_v2(
                nucleons,
                alphas,
                xis,
                deltas,
                kts,
                xs,
                q2s,
                grid_data
            );
        }

        // Finalize the Grid (inc. its subgrids) for this member.
        writer.push_grid(pids);
        std::cout << "Added grid for member " << m << "\n";
    }

    // Fill the running of alphas with some random values
    std::vector<double> alphas_qs = {2.0};
    std::vector<double> alphas_vals = {0.118};

    // Extract the ranges for the momentum x and scale Q2
    auto x_range = ref_pdf.param_range(NEOPDF_SUBGRID_PARAMS_MOMENTUM);
    auto q2_range = ref_pdf.param_range(NEOPDF_SUBGRID_PARAMS_SCALE);

    PhysicsParameters phys_params = {
        .flavor_scheme = "variable",
        .order_qcd = 2,
        .alphas_order_qcd = 2,
        .m_w = 80.352,
        .m_z = 91.1876,
        .m_up = 0.0,
        .m_down = 0.0,
        .m_strange = 0.0,
        .m_charm = 1.51,
        .m_bottom = 4.92,
        .m_top = 172.5,
        .alphas_type = "ipol",
        .number_flavors = 4,
    };

    MetaDataV2 meta;
    meta.set_desc = "NNPDF40_nnlo_as_01180 8D collection";
    meta.set_index = 0;
    meta.num_members = (uint32_t)neo_pdfs.size();
    meta.x_min = x_range[0];
    meta.x_max = x_range[1];
    meta.q_min = sqrt(q2_range[0]);
    meta.q_max = sqrt(q2_range[1]);
    meta.flavors = pids;
    meta.format = "neopdf";
    meta.alphas_q_values = alphas_qs;
    meta.alphas_vals = alphas_vals;
    meta.polarised = false;
    meta.set_type = NEOPDF_SET_TYPE_SPACE_LIKE;
    meta.interpolator_type = NEOPDF_INTERPOLATOR_TYPE_LOG_BICUBIC;
    meta.error_type = "replicas";
    meta.hadron_pid = 2212;
    meta.phys_params = phys_params;
    meta.xi_min = xis[0];
    meta.xi_max = xis.back();
    meta.delta_min = deltas[0];
    meta.delta_max = deltas.back();

    // Check if `NEOPDF_DATA_PATH` is defined and store the Grid there.
    const char* filename = "check-writer-8d.neopdf.lz4";
    const char* neopdf_path = std::getenv("NEOPDF_DATA_PATH");
    std::string output_path = neopdf_path
        ? std::string(neopdf_path) + (std::string(neopdf_path).back() == '/' ? "" : "/") + filename
        : filename;

    // Write the PDF Grid into disk
    try {
        writer.compress_v2(meta, output_path);
        std::cout << "Compression succeeded!\n";
    } catch (const std::runtime_error& err) {
        std::cerr << "Compression failed: " << err.what() << "\n";
        return EXIT_FAILURE;
    }

    // If `NEOPDF_DATA_PATH` is defined, reload the grid and check the results.
    if (neopdf_path) {
        int pid_test = 21;
        double x_test = 1e-3;
        double q2_test = 1e2;

        double ref = neo_pdfs[0].xfxQ2(pid_test, x_test, q2_test);

        // For the newly written 8D PDF
        NeoPDF wpdf(output_path);
        std::vector<double> params = {x_test, q2_test};
        double res = wpdf.xfxQ2_ND(pid_test, params);

        assert(std::abs(res - ref) < TOLERANCE);
    }

    return EXIT_SUCCESS;
}
