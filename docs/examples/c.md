# C/C++ API Example

This example demonstrates how to use the `NeoPDF` C/C++ API to load and evaluate parton
distributions. More examples can be found in [neopdf_capi/tests](https://github.com/Radonirinaunimi/neopdf/tree/master/neopdf_capi/tests).

## Prerequisites

Build and install the C API as described in the [installation guide](../installation.md).

## C Examples

In this section, we are goinge to illustrate how to use the C/C++ API in a pure C project.

### Example 1: Loading and Evaluating PDFs

In this first examples, we are going to show how to load both single and multiple PDF members,
evaluate parton distributions for a given value of the momentum fraction $x$ and momentum
transfer $Q^2$.

```c linenums="1"
#include <neopdf_capi.h>
#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

double* geomspace(double start, double stop, int num, bool endpoint) {
    double* result = (double*)malloc(num * sizeof(double));
    if (num == 1) {
        result[0] = start;
        return result;
    }

    double log_start = log(start);
    double log_stop = log(stop);
    double step = (log_stop - log_start) / (endpoint ? (num - 1) : num);

    for (int i = 0; i < num; ++i) {
        result[i] = exp(log_start + i * step);
    }

    return result;
}

void test_single_pdf() {
    printf("=== Test Loading a Single PDF Member ===\n");

    const char* pdfname = "NNPDF40_nnlo_as_01180";
    NeoPDFWrapper* neo_pdf = neopdf_pdf_load(pdfname, 0);

    int pids[] = {-5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5};
    int num_pids = sizeof(pids) / sizeof(pids[0]);

    double x_min = neopdf_pdf_x_min(neo_pdf);
    double x_max = neopdf_pdf_x_max(neo_pdf);
    double q2_min = neopdf_pdf_q2_min(neo_pdf);
    double q2_max = neopdf_pdf_q2_max(neo_pdf);

    int num_xs = 20;
    int num_q2s = 20;
    double* xs = geomspace(x_min, x_max, num_xs, false);
    double* q2s = geomspace(q2_min, q2_max, num_q2s, false);

    printf("%6s %15s %15s %15s\n", "pid", "x", "Q2", "NeoPDF");
    printf("----------------------------------------------------------------\n");

    for (int i = 0; i < num_pids; ++i) {
        for (int j = 0; j < num_xs; ++j) {
            for (int k = 0; k < num_q2s; ++k) {
                double result = neopdf_pdf_xfxq2(neo_pdf, pids[i], xs[j], q2s[k]);
                printf("%6d %15.8e %15.8e %15.8e\n", pids[i], xs[j], q2s[k], result);
            }
        }
    }

    free(xs);
    free(q2s);
    neopdf_pdf_free(neo_pdf);
}

void test_all_pdf_members() {
    printf("=== Test Loading all the PDF Members ===\n");

    const char* pdfname = "NNPDF40_nnlo_as_01180";
    NeoPDFMembers neo_pdfs = neopdf_pdf_load_all(pdfname);

    printf("Loaded %zu PDF members\n", neo_pdfs.size);

    int pid = 1;
    double x = 1e-9;
    double q2 = 1.65 * 1.65;

    printf("\nEvaluating xfxQ2 for pid=%d, x=%e, Q2=%e across all members:\n", pid, x, q2);
    printf("%8s %15s\n", "Member", "NeoPDF");
    printf("-------------------------\n");

    double* results = (double*)malloc(neo_pdfs.size * sizeof(double));
    for (size_t i = 0; i < neo_pdfs.size; ++i) {
        NeoPDFWrapper* pdf = neo_pdfs.pdfs[i];
        double result = neopdf_pdf_xfxq2(pdf, pid, x, q2);
        results[i] = result;
        printf("%8zu %15.8e\n", i, result);
    }

    double sum = 0.0;
    for (size_t i = 0; i < neo_pdfs.size; ++i) {
        sum += results[i];
    }
    double mean = sum / neo_pdfs.size;

    double variance = 0.0;
    for (size_t i = 0; i < neo_pdfs.size; ++i) {
        variance += (results[i] - mean) * (results[i] - mean);
    }
    variance /= neo_pdfs.size;
    double std_dev = sqrt(variance);

    printf("\nStatistics across all members:\n");
    printf("Mean: %e\n", mean);
    printf("Std Dev: %e\n", std_dev);
    printf("Relative Std Dev: %e\n", std_dev / mean);

    free(results);
    neopdf_pdf_array_free(neo_pdfs);
}

void test_lazy_loading() {
    printf("=== Test Lazy Loading of PDF Members ===\n");

    const char* pdfname = "NNPDF40_nnlo_as_01180.neopdf.lz4";
    NeoPDFLazyIterator* lazy_iter = neopdf_pdf_load_lazy(pdfname);

    if (!lazy_iter) {
        fprintf(stderr, "Failed to load lazy iterator for %s\n", pdfname);
        return;
    }

    printf("Successfully loaded lazy iterator for %s\n", pdfname);

    int pid = 1;
    double x = 1e-9;
    double q2 = 1.65 * 1.65;

    printf("\nEvaluating xfxQ2 for pid=%d, x=%e, Q2=%e across all members (lazily):\n", pid, x, q2);
    printf("%8s %15s\n", "Member", "NeoPDF");
    printf("-------------------------\n");

    int member_idx = 0;
    NeoPDFWrapper* pdf;
    while ((pdf = neopdf_lazy_iterator_next(lazy_iter))) {
        double result = neopdf_pdf_xfxq2(pdf, pid, x, q2);
        printf("%8d %15.8e\n", member_idx, result);
        neopdf_pdf_free(pdf);
        member_idx++;
    }

    neopdf_lazy_iterator_free(lazy_iter);

    printf("\nSuccessfully iterated through all members lazily.\n");
}

int main() {
    test_single_pdf();
    test_all_pdf_members();
    test_lazy_loading();

    return EXIT_SUCCESS;
}
```

### Example 2: Filling and Writing a NeoPDF Grid

This example illustrates how to fill and write a `NeoPDF` grid using the C API. It demonstrates
the process of constructing a grid for each PDF member and serializing the collection to disk.

!!! tip "NOTE"

    As in the other sectioons, the following example fills the `NeoPDF` grid by re-computing
    the values of the subgrids from another set. This makes it possible to explicitly check
    that the filling of the grid is correct. However, this makes the codes very verbose. To
    easily spot the parts that actually fills the grid, some lines are highlighted.

```c linenums="1" hl_lines="66 76 130-138 141-149 164 172 191-205 207-228 241"
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
```

## C++ Examples

In the following section, we are going to see how to use the C/C++ API in a C++ code. For some
exampels of Object Oriented C++ codes, head over to the [C++ OOP section](c-oop.md).

### Example 1: Loading and Evaluating PDFs

This example demonstrates the use of the `NeoPDF` C API to load both single and multiple PDF
members, evaluate parton distributions for a range of $x$ and $Q^2$ values, and compare the
results to LHAPDF.

**Technical details:**

- The C API requires explicit memory management: objects created with `neopdf_pdf_load` or
  `neopdf_pdf_load_all` must be freed with `neopdf_pdf_free` or `neopdf_pdf_array_free` to
  avoid memory leaks.
- The evaluation of $x f(x, Q^2)$ and $\alpha_s(Q^2)$ is performed in nested loops for each parton,
  $x$, and $Q^2$ value. See the documentation on how to use the parallelized `xfxQ2s`.
- The code asserts that the results from `NeoPDF` and `LHAPDF` agree within a tight tolerance.

```cpp linenums="1"
#include <LHAPDF/PDF.h>
#include <neopdf_capi.h>
#include <cassert>
#include <cmath>
#include <cstdlib>
#include <iomanip>
#include <iostream>
#include <vector>

const double TOLERANCE= 1e-16;

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

void test_single_pdf() {
    std::cout << "=== Test Loading a Single PDF Member ===\n";

    // disable LHAPDF banners to guarantee deterministic output
    LHAPDF::setVerbosity(0);

    std::string pdfname = "NNPDF40_nnlo_as_01180";
    NeoPDFWrapper* neo_pdf = neopdf_pdf_load(pdfname.c_str(), 0);
    auto lha_pdf = std::unique_ptr<LHAPDF::PDF>(LHAPDF::mkPDF(pdfname, 0));

    std::vector<int> pids = {-5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5};
    std::vector<double> xs = geomspace(lha_pdf->xMin(), lha_pdf->xMax(), 200);
    std::vector<double> q2s = geomspace(lha_pdf->q2Min(), lha_pdf->q2Max(), 200);

    // Headers of the table to print the results
    std::cout << std::right
        << std::setw(6) << "pid"
        << std::setw(15) << "x"
        << std::setw(15) << "Q2"
        << std::setw(15) << "LHAPDF"
        << std::setw(15) << "NeoPDF"
        << std::setw(15) << "Rel. Diff." << "\n";
    std::cout << std::string(81, '-') << "\n";

    for (const auto &pid: pids) {
        for (const auto &x: xs) {
            for (const auto &q2: q2s) {
                double expected = lha_pdf->xfxQ2(pid, x, q2);
                double result = neopdf_pdf_xfxq2(neo_pdf, pid, x, q2);
                double reldif = std::abs(result - expected) / expected;

                assert(std::abs(result - expected) < TOLERANCE);

                // Print the results as a table
                std::cout << std::scientific << std::setprecision(8)
                    << std::right
                    << std::setw(6)  << pid
                    << std::setw(15) << x
                    << std::setw(15) << q2
                    << std::right
                    << std::setw(15) << expected
                    << std::setw(15) << result
                    << std::setw(15) << reldif << "\n";
                }
            }

    }

    // Delete PDF object from memory
    neopdf_pdf_free(neo_pdf);
}

void test_all_pdf_members() {
    std::cout << "=== Test Loading all the PDF Members ===\n";

    // disable LHAPDF banners to guarantee deterministic output
    LHAPDF::setVerbosity(0);

    std::string pdfname = "NNPDF40_nnlo_as_01180";
    NeoPDFMembers neo_pdfs = neopdf_pdf_load_all(pdfname.c_str());

    std::cout << "Loaded " << neo_pdfs.size << " PDF members\n";

    // Test case: evaluate a simple point across all members
    int pid = 1;
    double x = 1e-9;
    double q2 = 1.65 * 1.65;

    std::cout << "\nEvaluating xfxQ2 for pid=" << pid
              << ", x=" << std::scientific << x
              << ", Q2=" << q2 << " across all members:\n";

    std::cout << std::right
        << std::setw(8) << "Member"
        << std::setw(15) << "LHAPDF"
        << std::setw(15) << "NeoPDF"
        << std::setw(15) << "Rel. Diff." << "\n";
    std::cout << std::string(53, '-') << "\n";

    // Evaluate the same point across all PDF members
    std::vector<double> results;
    for (size_t i = 0; i < neo_pdfs.size; ++i) {
        NeoPDFWrapper* pdf = neo_pdfs.pdfs[i];
        auto lha_pdf = std::unique_ptr<LHAPDF::PDF>(LHAPDF::mkPDF(pdfname, i));

        double expected = lha_pdf->xfxQ2(pid, x, q2);
        double result = neopdf_pdf_xfxq2(pdf, pid, x, q2);

        double reldif = std::abs(result - expected) / expected;
        assert(std::abs(result - expected) < TOLERANCE);
        results.push_back(result);

        std::cout << std::right
            << std::setw(8) << i
            << std::scientific << std::setprecision(8)
            << std::setw(15) << expected
            << std::setw(15) << result
            << std::setw(15) << reldif << "\n";
    }

    // Calculate some statistics
    double sum = 0.0;
    for (double result : results) {
        sum += result;
    }
    double mean = sum / results.size();

    double variance = 0.0;
    for (double result : results) {
        variance += (result - mean) * (result - mean);
    }
    variance /= results.size();
    double std_dev = std::sqrt(variance);

    std::cout << "\nStatistics across all members:\n";
    std::cout << "Mean: " << std::scientific << std::setprecision(8) << mean << "\n";
    std::cout << "Std Dev: " << std_dev << "\n";
    std::cout << "Relative Std Dev: " << std_dev / mean << "\n";

    // Delete objects from memory.
    neopdf_pdf_array_free(neo_pdfs);
}

void test_lazy_loading() {
    std::cout << "=== Test Lazy Loading of PDF Members ===\n";

    // Disable LHAPDF banners to guarantee deterministic output
    LHAPDF::setVerbosity(0);

    std::string pdfname = "NNPDF40_nnlo_as_01180";
    std::string neopdf_name = pdfname + ".neopdf.lz4";
    NeoPDFLazyIterator* lazy_iter = neopdf_pdf_load_lazy(neopdf_name.c_str());

    if (!lazy_iter) {
        std::cerr << "Failed to load lazy iterator for " << pdfname << std::endl;
        return;
    }

    std::cout << "Successfully loaded lazy iterator for " << pdfname << "\n";

    // Test case: evaluate a simple point across all members
    int pid = 1;
    double x = 1e-9;
    double q2 = 1.65 * 1.65;

    std::cout << "\nEvaluating xfxQ2 for pid=" << pid
              << ", x=" << std::scientific << x
              << ", Q2=" << q2 << " across all members (lazily):\n";

    std::cout << std::right
        << std::setw(8) << "Member"
        << std::setw(15) << "LHAPDF"
        << std::setw(15) << "NeoPDF"
        << std::setw(15) << "Rel. Diff." << "\n";
    std::cout << std::string(53, '-') << "\n";

    // Evaluate the same point across all PDF members
    std::vector<double> results;
    int member_idx = 0;
    while (NeoPDFWrapper* pdf = neopdf_lazy_iterator_next(lazy_iter)) {
        auto lha_pdf = std::unique_ptr<LHAPDF::PDF>(LHAPDF::mkPDF(pdfname, member_idx));

        double expected = lha_pdf->xfxQ2(pid, x, q2);
        double result = neopdf_pdf_xfxq2(pdf, pid, x, q2);

        double reldif = std::abs(result - expected) / expected;
        assert(std::abs(result - expected) < TOLERANCE);
        results.push_back(result);

        std::cout << std::right
            << std::setw(8) << member_idx
            << std::scientific << std::setprecision(8)
            << std::setw(15) << expected
            << std::setw(15) << result
            << std::setw(15) << reldif << "\n";

        neopdf_pdf_free(pdf); // Free the individual PDF member
        member_idx++;
    }

    // Free the lazy iterator
    neopdf_lazy_iterator_free(lazy_iter);

    std::cout << "\nSuccessfully iterated through all members lazily.\n";
}


int main() {
    // Test loading single PDF member
    test_single_pdf();

    // Test loading all the PDF members
    test_all_pdf_members();

    // Test lazy loading of PDF members
    test_lazy_loading();

    return EXIT_SUCCESS;
}
```

### Example 2: Filling and Writing a NeoPDF Grid

This example illustrates how to fill and write a `NeoPDF` grid using the C API. It demonstrates
the process of constructing a grid for each PDF member and serializing the collection to disk.

**Technical details:**

- The grid axes are defined as arrays for $x$, $Q^2$, parton IDs, nucleons, and $\alpha_s$ values.
- The grid data is stored in a 6D array, with the layout `[nucleons][alphas][pids][kT][xs][q2s]`.
- The `NeoPDFGridArrayCollection` manages the collection of grids and handles compression and
  serialization to disk.
- Metadata is filled in a `NeoPDFMetaData` struct, which includes information about the set, axis
  ranges, flavors, and interpolation type. This metadata is essential for correct interpretation
  of the grid file.
- All memory management must be handled manually; every object created must be freed with the
  appropriate function to avoid leaks.
- The output file is compressed and written in the `.neopdf.lz4` format, suitable for use with
  `NeoPDF` tools and APIs.

!!! tip "NOTE"

    The following example fills the `NeoPDF` grid by re-computing the values of the subgrids
    from another set. This makes it possible to explicitly check that the filling of the grid
    is correct. However, this makes the codes very verbose. To easily spot the parts that
    actually fills the grid, some lines are highlighted.

```cpp linenums="1" hl_lines="112-121 124-132 141 149 168-182 184-205 215"
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

const double TOLERANCE= 1e-16;

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

    // Extrac the number of subgrids
    std::size_t num_subgrids = neopdf_pdf_num_subgrids(neo_pdfs.pdfs[0]);

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
        for (std::size_t subgrid_idx = 0; subgrid_idx != num_subgrids; subgrid_idx++) {
            // Extrac the parameter values of the given subgrid
            auto xs = extract_subgrid_params(pdf, NEOPDF_SUBGRID_PARAMS_MOMENTUM, subgrid_idx, num_subgrids);
            auto q2s = extract_subgrid_params(pdf, NEOPDF_SUBGRID_PARAMS_SCALE, subgrid_idx, num_subgrids);
            auto alphas = extract_subgrid_params(pdf, NEOPDF_SUBGRID_PARAMS_ALPHAS, subgrid_idx, num_subgrids);
            auto nucleons = extract_subgrid_params(pdf, NEOPDF_SUBGRID_PARAMS_NUCLEONS, subgrid_idx, num_subgrids);
            auto kts = extract_subgrid_params(pdf, NEOPDF_SUBGRID_PARAMS_KT, subgrid_idx, num_subgrids);

            // Compute grid_data: [q2s][xs][flavors], instead of [nucleons][alphas][q2s][xs][flavors]
            // NOTE: This assumes that there is no 'A' and `alphas` dependence.
            assert(nucleons.size() == 1);
            assert(alphas.size() == 1);
            assert(kts.size() == 1);
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

            // Add subgrid
            int add_subgrid = neopdf_grid_add_subgrid(
                grid,
                nucleons.data(), nucleons.size(),
                alphas.data(), alphas.size(),
                kts.data(), kts.size(),
                xs.data(), xs.size(),
                q2s.data(), q2s.size(),
                grid_data.data(), grid_data.size()
            );
            if (add_subgrid != NEOPDF_RESULT_SUCCESS) {
                std::cerr << "Failed to add subgrid for member: " << m << "!\n";
                neopdf_grid_free(grid);
                continue;
            }
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

    // Extrac the ranges for the momentum x and scale Q2
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

    NeoPDFMetaData meta = {
        .set_desc = "NNPDF40_nnlo_as_01180 collection",
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
    };

    // Check if `NEOPDF_DATA_PATH` is defined and store the Grid there.
    const char* filename = "check-writer.neopdf.lz4";
    const char* neopdf_path = std::getenv("NEOPDF_DATA_PATH");
    std::string output_path = neopdf_path
        ? std::string(neopdf_path) + (std::string(neopdf_path).back() == '/' ? "" : "/") + filename
        : filename;

    // Write the PDF Grid into disk
    int result = neopdf_grid_compress(collection, &meta, output_path.c_str());
    if (result != 0) {
        std::cerr << "Compression failed with code " << result << "\n";
    } else {
        std::cout << "Compression succeeded!" << "\n";
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

        assert(std::abs(res1 - ref1) < TOLERANCE);
        assert(std::abs(res2 - ref2) < TOLERANCE);

        // Delete PDF object from memory
        neopdf_pdf_free(wpdf);
    }

    // Cleanup
    neopdf_gridarray_collection_free(collection);
    neopdf_pdf_array_free(neo_pdfs);

    return result == 0 ? 0 : 1;
}
```

### Example 3: Filling and Writing a NeoPDF Grid with generic Parameters

In the case where the PDF grid depends on more parameters, the filling of `grid_data`
in the above example simply now becomes:

```cpp linenums="1"
std::vector<double> grid_data;
for (size_t ni = 0; ni < nucleons.size(); ++ni) {
    for (size_t asi = 0; asi < alphas.size(); ++asi) {
        for (size_t xi = 0; xi < xs.size(); ++xi) {
            for (size_t qi = 0; qi < q2s.size(); ++qi) {
                for (size_t f = 0; f < pids.size(); ++f) {
                    int pid = pids[f];
                    std::vector<double> params = {nucleons[ni], alphas[asi], xs[xi], q2s[qi]};
                    double val = neopdf_pdf_xfxq2_nd(pdf, pid, params.data(), params.size());
                    grid_data.push_back(val);
                }
            }
        }
    }
}
```
