#include <cstddef>
#include <neopdf_capi.h>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <string>
#include <vector>

const double TOLERANCE = 1e-10; // Increased tolerance for synthetic data

template<typename T>
std::vector<T> geomspace(T start, T stop, int num, bool endpoint = false) {
    std::vector<T> result(num);

    if (num == 1) {
        result[0] = start;
        return result;
    }

    T log_start = std::log(start);
    T log_stop = std::log(stop);
    T step = (log_stop - log_start) / (endpoint ? (num - 1) : num);

    for (int i = 0; i < num; ++i) {
        result[i] = std::exp(log_start + i * step);
    }

    return result;
}

// Helper function to extract subgrid parameters
std::vector<double> extract_subgrid_params(
    NeoPDFWrapper* pdf,
    NeopdfSubgridParams param_type,
    std::size_t subgrid_idx,
    std::size_t num_subgrids
) {
    std::vector<std::size_t> shape(num_subgrids);
    neopdf_pdf_subgrids_shape_for_param(
        pdf,
        shape.data(),
        num_subgrids,
        param_type
    );

    std::vector<double> values(shape[subgrid_idx]);
    neopdf_pdf_subgrids_for_param(
        pdf,
        values.data(),
        param_type,
        num_subgrids,
        shape.data(),
        subgrid_idx
    );

    return values;
}

int main() {
    const char* pdfname = "NNPDF40_nnlo_as_01180";
    // Load all PDF members
    NeoPDFMembers neo_pdfs = neopdf_pdf_load_all(pdfname);
    if (neo_pdfs.size == 0) {
        std::cerr << "Failed to load any PDF members!\n";
        return 1;
    }
    std::cout << "Loaded " << neo_pdfs.size << " PDF members\n";

    // Extract the PID values of the PDF set
    std::size_t num_pids = neopdf_pdf_num_pids(neo_pdfs.pdfs[0]);
    std::vector<int> pids(num_pids);
    neopdf_pdf_pids(neo_pdfs.pdfs[0], pids.data(), num_pids);

    // Extract the number of subgrids
    std::size_t num_subgrids = neopdf_pdf_num_subgrids(neo_pdfs.pdfs[0]);

    // Define synthetic xi and delta values
    std::vector<double> xis = {0.0};
    std::vector<double> deltas = {0.0};

    // Create a collection
    NeoPDFGridArrayCollection* collection = neopdf_gridarray_collection_new();
    if (!collection) {
        std::cerr << "Failed to create grid array collection!\n";
        neopdf_pdf_array_free(neo_pdfs);
        return 1;
    }

    // For each member, build a grid
    for (size_t m = 0; m < neo_pdfs.size; ++m) {
        NeoPDFWrapper* pdf = neo_pdfs.pdfs[m];
        NeoPDFGrid* grid = neopdf_grid_new();

        if (!grid) {
            std::cerr << "Failed to create grid for member: " << m << "!\n";
            continue;
        }

        // Loop over the Subgrids
        bool member_failed = false;
        for (std::size_t subgrid_idx = 0; subgrid_idx < num_subgrids; ++subgrid_idx) {
            // Extract base parameters from the original PDF (x, q2, nucleons, alphas, kts)
            auto xs = extract_subgrid_params(pdf, NEOPDF_SUBGRID_PARAMS_MOMENTUM, subgrid_idx, num_subgrids);
            auto q2s = extract_subgrid_params(pdf, NEOPDF_SUBGRID_PARAMS_SCALE, subgrid_idx, num_subgrids);
            auto alphas = extract_subgrid_params(pdf, NEOPDF_SUBGRID_PARAMS_ALPHAS, subgrid_idx, num_subgrids);
            auto nucleons = extract_subgrid_params(pdf, NEOPDF_SUBGRID_PARAMS_NUCLEONS, subgrid_idx, num_subgrids);
            auto kts = extract_subgrid_params(pdf, NEOPDF_SUBGRID_PARAMS_KT, subgrid_idx, num_subgrids);

            // Compute grid_data: [q2s][xs][flavors], instead of [nucleons][alphas][xis][deltas][q2s][xs][flavors]
            // NOTE: This assumes that there is no 'A', `alphas`, `xis`, and `deltas` dependence.
            assert(alphas.size() == 1);
            assert(kts.size() == 1);
            assert(xis.size() == 1);
            assert(deltas.size() == 1);
            assert(nucleons.size() == 1);
            std::vector<double> grid_data;
            for (size_t xi = 0; xi < xs.size(); ++xi) {
                for (size_t qi = 0; qi < q2s.size(); ++qi) {
                    for (size_t f = 0; f < pids.size(); ++f) {
                        int pid = pids[f];
                        double val = neopdf_pdf_xfxq2(pdf, pid, xs[xi], q2s[qi]);
                        grid_data.push_back(val);
                    }
                }
            }

            // Add subgrid using the v2 function
            NeopdfResult add_subgrid_res = neopdf_grid_add_subgridv2(
                grid,
                nucleons.data(), nucleons.size(),
                alphas.data(), alphas.size(),
                xis.data(), xis.size(),
                deltas.data(), deltas.size(),
                kts.data(), kts.size(),
                xs.data(), xs.size(),
                q2s.data(), q2s.size(),
                grid_data.data(), grid_data.size()
            );
            if (add_subgrid_res != NEOPDF_RESULT_SUCCESS) {
                std::cerr << "Failed to add subgrid (v2) for member: " << m << "!\n";
                neopdf_grid_free(grid);
                member_failed = true;
                break;
            }
        }

        if (member_failed) {
            continue;
        }

        // Set flavor PIDs
        int add_flavors = neopdf_grid_set_flavors(grid, pids.data(), pids.size());
        if (add_flavors != 0) {
            std::cerr << "Failed to set flavors for member: " << m << "!\n";
            neopdf_grid_free(grid);
            continue;
        }

        // Add grid to collection
        int add_grid = neopdf_gridarray_collection_add_grid(collection, grid);
        if (add_grid != 0) {
            std::cerr << "Failed to add grid to collection for member: " << m << "!\n";
            neopdf_grid_free(grid);
            continue;
        }
        std::cout << "Added grid for member " << m << "\n";
    }

    // Fill the running of alphas with some random values
    double alphas_qs[] = {2.0};
    double alphas_vals[] = {0.118};

    // Extract the ranges for the momentum x and scale Q2
    std::vector<double> x_range(2);
    std::vector<double> q2_range(2);
    neopdf_pdf_param_range(neo_pdfs.pdfs[0], NEOPDF_SUBGRID_PARAMS_MOMENTUM, x_range.data());
    neopdf_pdf_param_range(neo_pdfs.pdfs[0], NEOPDF_SUBGRID_PARAMS_SCALE, q2_range.data());

    NeoPDFPhysicsParameters phys_params = {
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

    NeoPDFMetaDataV2 meta = {
        .set_desc = "NNPDF40_nnlo_as_01180 8D collection",
        .set_index = 0,
        .num_members = (uint32_t)neo_pdfs.size,
        .x_min = x_range[0],
        .x_max = x_range[1],
        .q_min = sqrt(q2_range[0]),
        .q_max = sqrt(q2_range[1]),
        .flavors = pids.data(),
        .num_flavors = (size_t)pids.size(),
        .format = "neopdf",
        .alphas_q_values = alphas_qs,
        .num_alphas_q = 1,
        .alphas_vals = alphas_vals,
        .num_alphas_vals = 1,
        .polarised = false,
        .set_type = NEOPDF_SET_TYPE_SPACE_LIKE,
        .interpolator_type = NEOPDF_INTERPOLATOR_TYPE_LOG_BICUBIC,
        .error_type = "replicas",
        .hadron_pid = 2212,
        .phys_params = phys_params,
        .xi_min = xis[0],
        .xi_max = xis.back(),
        .delta_min = deltas[0],
        .delta_max = deltas.back(),
    };

    // Check if `NEOPDF_DATA_PATH` is defined and store the Grid there.
    const char* filename = "check-writer-8d.neopdf.lz4";
    const char* neopdf_path = std::getenv("NEOPDF_DATA_PATH");
    std::string output_path = neopdf_path
        ? std::string(neopdf_path) + (std::string(neopdf_path).back() == '/' ? "" : "/") + filename
        : filename;

    // Write the PDF Grid into disk
    int result = neopdf_grid_compress_v2(collection, &meta, output_path.c_str());
    if (result != 0) {
        std::cerr << "Compression failed with code " << result << "\n";
    } else {
        std::cout << "Compression succeeded!\n";
    }

    // If `NEOPDF_DATA_PATH` is defined, reload the grid and check the results.
    if (neopdf_path) {
        int pid_test = 21;
        double x_test = 1e-3;
        double q2_test = 1e2;

        double ref = neopdf_pdf_xfxq2(neo_pdfs.pdfs[0], pid_test, x_test, q2_test);

        // For the newly written 8D PDF
        NeoPDFWrapper* wpdf = neopdf_pdf_load(output_path.c_str(), 0);
        std::vector<double> params = {x_test, q2_test};
        double res = neopdf_pdf_xfxq2_nd(wpdf, pid_test, params.data(), params.size());

        assert(std::abs(res - ref) < TOLERANCE);

        // Delete PDF object from memory
        neopdf_pdf_free(wpdf);
    }

    // Cleanup
    neopdf_gridarray_collection_free(collection);
    neopdf_pdf_array_free(neo_pdfs);

    return result == 0 ? 0 : 1;
}
