#include <neopdf_capi.h>
#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const double TOLERANCE = 1e-16;

struct SubgridParams {
    double * values;
    size_t size;
};

// Helper function to extract subgrid parameters
struct SubgridParams extract_subgrid_params(
    NeoPDFWrapper* pdf,
    NeopdfSubgridParams param_type,
    size_t subgrid_idx,
    size_t num_subgrids
) {
    size_t* shape = (size_t*)malloc(num_subgrids * sizeof(size_t));
    neopdf_pdf_subgrids_shape_for_param(
        pdf,
        shape,
        num_subgrids,
        param_type
    );
    size_t size = shape[subgrid_idx];

    double* values = (double*)malloc(size * sizeof(double));
    neopdf_pdf_subgrids_for_param(
        pdf,
        values,
        param_type,
        num_subgrids,
        shape,
        subgrid_idx
    );

    free(shape);
    struct SubgridParams results = {.values=values, .size=size};
    return results;
}

int main() {
    const char* pdfname = "NNPDF40_nnlo_as_01180";
    // Load all PDF members
    NeoPDFMembers neo_pdfs = neopdf_pdf_load_all(pdfname);
    if (neo_pdfs.size == 0) {
        fprintf(stderr, "Failed to load any PDF members!\n");
        return 1;
    }
    printf("Loaded %zu PDF members\n", neo_pdfs.size);

    // Extract the PID values of the PDF set
    size_t num_pids = neopdf_pdf_num_pids(neo_pdfs.pdfs[0]);
    int* pids = (int*)malloc(num_pids * sizeof(int));
    neopdf_pdf_pids(neo_pdfs.pdfs[0], pids, num_pids);

    // Extrac the number of subgrids
    size_t num_subgrids = neopdf_pdf_num_subgrids(neo_pdfs.pdfs[0]);

    // Create a collection
    NeoPDFGridArrayCollection* collection = neopdf_gridarray_collection_new();
    if (!collection) {
        fprintf(stderr, "Failed to create grid array collection!\n");
        neopdf_pdf_array_free(neo_pdfs);
        return 1;
    }

    // For each member, build a grid
    for (size_t m = 0; m < neo_pdfs.size; ++m) {
        NeoPDFWrapper* pdf = neo_pdfs.pdfs[m];
        NeoPDFGrid* grid = neopdf_grid_new();

        if (!grid) {
            fprintf(stderr, "Failed to create grid for member: %zu!\n", m);
            continue;
        }

        // Loop over the Subgrids
        for (size_t subgrid_idx = 0; subgrid_idx != num_subgrids; subgrid_idx++) {
            // Extrac the parameter values of the given subgrid
            struct SubgridParams xs_obj = extract_subgrid_params(
                pdf,
                NEOPDF_SUBGRID_PARAMS_MOMENTUM,
                subgrid_idx,
                num_subgrids
            );
            struct SubgridParams q2s_obj = extract_subgrid_params(
                pdf,
                NEOPDF_SUBGRID_PARAMS_SCALE,
                subgrid_idx,
                num_subgrids
            );
            struct SubgridParams alphas_obj = extract_subgrid_params(
                pdf,
                NEOPDF_SUBGRID_PARAMS_ALPHAS,
                subgrid_idx,
                num_subgrids
            );
            struct SubgridParams nucleons_obj = extract_subgrid_params(
                pdf,
                NEOPDF_SUBGRID_PARAMS_NUCLEONS,
                subgrid_idx,
                num_subgrids
            );
            struct SubgridParams kts_obj = extract_subgrid_params(
                pdf,
                NEOPDF_SUBGRID_PARAMS_KT,
                subgrid_idx,
                num_subgrids
            );

            // Extract the values
            double* xs =  xs_obj.values;
            double* q2s =  q2s_obj.values;
            double* alphas =  alphas_obj.values;
            double* nucleons =  nucleons_obj.values;
            double* kts =  kts_obj.values;

            // Compute grid_data: [q2s][xs][flavors], instead of [nucleons][alphas][q2s][xs][flavors]
            // NOTE: This assumes that there is no `A`, `kT`, and `alphas` dependence.
            size_t xs_size = xs_obj.size;
            size_t q2s_size = q2s_obj.size;
            double* grid_data = (double*)malloc(xs_size * q2s_size * num_pids * sizeof(double));
            int data_idx = 0;
            for (size_t xi = 0; xi < xs_size; ++xi) {
                for (size_t qi = 0; qi < q2s_size; ++qi) {
                    for (size_t f = 0; f < num_pids; ++f) {
                        int pid = pids[f];
                        double val = neopdf_pdf_xfxq2(pdf, pid, xs[xi], q2s[qi]);
                        grid_data[data_idx++] = val;
                    }
                }
            }

            // Add subgrid
            int add_subgrid = neopdf_grid_add_subgrid(
                grid,
                nucleons, 1,
                alphas, 1,
                kts, 1,
                xs, xs_size,
                q2s, q2s_size,
                grid_data, data_idx
            );
            if (add_subgrid != NEOPDF_RESULT_SUCCESS) {
                fprintf(stderr, "Failed to add subgrid for member: %zu!\n", m);
                neopdf_grid_free(grid);
                continue;
            }
            free(xs);
            free(q2s);
            free(alphas);
            free(nucleons);
            free(kts);
            free(grid_data);
        }

        // Set flavor PIDs
        int add_flavors = neopdf_grid_set_flavors(grid, pids, num_pids);
        if (add_flavors != 0) {
            fprintf(stderr, "Failed to set flavors for member: %zu!\n", m);
            neopdf_grid_free(grid);
            continue;
        }

        // Add grid to collection
        int add_grid = neopdf_gridarray_collection_add_grid(collection, grid);
        if (add_grid != 0) {
            fprintf(stderr, "Failed to add grid to collection for member: %zu!\n", m);
            neopdf_grid_free(grid);
            continue;
        }
        printf("Added grid for member %zu\n", m);
    }

    // Fill the running of alphas with some random values
    double alphas_qs[] = {2.0};
    double alphas_vals[] = {0.118};

    // Extrac the ranges for the momentum x and scale Q2
    double x_range[2];
    double q2_range[2];
    neopdf_pdf_param_range(neo_pdfs.pdfs[0], NEOPDF_SUBGRID_PARAMS_MOMENTUM, x_range);
    neopdf_pdf_param_range(neo_pdfs.pdfs[0], NEOPDF_SUBGRID_PARAMS_SCALE, q2_range);

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

    NeoPDFMetaData meta = {
        .set_desc = "NNPDF40_nnlo_as_01180 collection",
        .set_index = 0,
        .num_members = (uint32_t)neo_pdfs.size,
        .x_min = x_range[0],
        .x_max = x_range[1],
        .q_min = sqrt(q2_range[0]),
        .q_max = sqrt(q2_range[1]),
        .flavors = pids,
        .num_flavors = (size_t)num_pids,
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
    };

    // Check if `NEOPDF_DATA_PATH` is defined and store the Grid there.
    const char* filename = "check-writer.neopdf.lz4";
    const char* neopdf_path = getenv("NEOPDF_DATA_PATH");
    char output_path[256];
    if (neopdf_path) {
        snprintf(output_path, sizeof(output_path), "%s/%s", neopdf_path, filename);
    } else {
        snprintf(output_path, sizeof(output_path), "%s", filename);
    }

    // Write the PDF Grid into disk
    int result = neopdf_grid_compress(collection, &meta, output_path);
    if (result != 0) {
        fprintf(stderr, "Compression failed with code %d\n", result);
    } else {
        printf("Compression succeeded!\n");
    }

    // If `NEOPDF_DATA_PATH` is defined, reload the grid and check ther results.
    if (neopdf_path) {
        int pid_test = 21;
        double x_test = 1e-3;
        double q2_test1 = 1e2;
        double q2_test2 = 1e4;

        double ref1 = neopdf_pdf_xfxq2(neo_pdfs.pdfs[0], pid_test, x_test, q2_test1);
        double ref2 = neopdf_pdf_xfxq2(neo_pdfs.pdfs[0], pid_test, x_test, q2_test2);

        NeoPDFWrapper* wpdf = neopdf_pdf_load(filename, 0);
        double res1 = neopdf_pdf_xfxq2(wpdf, pid_test, x_test, q2_test1);
        double res2 = neopdf_pdf_xfxq2(wpdf, pid_test, x_test, q2_test2);

        assert(fabs(res1 - ref1) < TOLERANCE);
        assert(fabs(res2 - ref2) < TOLERANCE);

        // Delete PDF object from memory
        neopdf_pdf_free(wpdf);
    }

    // Clip the interpolated values to zero when negatives
    neopdf_pdf_set_force_positive(neo_pdfs.pdfs[0], NEOPDF_FORCE_POSITIVE_CLIP_NEGATIVE);
    neopdf_force_positive pos_clip = neopdf_pdf_is_force_positive(neo_pdfs.pdfs[0]);
    assert(pos_clip == NEOPDF_FORCE_POSITIVE_CLIP_NEGATIVE);

    // Clip the interpolated values for all the PDF members
    neopdf_pdf_set_force_positive_members(&neo_pdfs, NEOPDF_FORCE_POSITIVE_CLIP_SMALL);
    neopdf_force_positive clip_small = neopdf_pdf_is_force_positive(neo_pdfs.pdfs[4]);
    assert(clip_small == NEOPDF_FORCE_POSITIVE_CLIP_SMALL);

    // Cleanup
    free(pids);
    neopdf_gridarray_collection_free(collection);
    neopdf_pdf_array_free(neo_pdfs);

    return result == 0 ? 0 : 1;
}
