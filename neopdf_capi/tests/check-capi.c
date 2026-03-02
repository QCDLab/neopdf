#include <neopdf_capi.h>
#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

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

void test_load_by_lhaid() {
    printf("=== Test Loading a PDF Member by LHAID ===\n");

    /* NNPDF40_nnlo_as_01180 has base LHAID 331100; member 0 = LHAID 331100 */
    uint32_t lhaid = 331100;
    const char* pdfname = "NNPDF40_nnlo_as_01180";

    NeoPDFWrapper* pdf_by_id   = neopdf_pdf_load_by_lhaid(lhaid);
    NeoPDFWrapper* pdf_by_name = neopdf_pdf_load(pdfname, 0);

    int    pid = 21;
    double x   = 1e-4;
    double q2  = 100.0;

    double result   = neopdf_pdf_xfxq2(pdf_by_id,   pid, x, q2);
    double expected = neopdf_pdf_xfxq2(pdf_by_name, pid, x, q2);
    double reldif   = fabs(result - expected) / fabs(expected);

    printf("LHAID %u -> xfxQ2(pid=%d, x=%e, Q2=%e)\n", lhaid, pid, x, q2);
    printf("  load_by_name:  %e\n", expected);
    printf("  load_by_lhaid: %e\n", result);
    printf("  Relative diff: %e\n", reldif);

    double as_result   = neopdf_pdf_alphas_q2(pdf_by_id,   q2);
    double as_expected = neopdf_pdf_alphas_q2(pdf_by_name, q2);
    double as_reldif   = fabs(as_result - as_expected) / fabs(as_expected);

    printf("LHAID %u -> alphasQ2(Q2=%e)\n", lhaid, q2);
    printf("  load_by_name:  %e\n", as_expected);
    printf("  load_by_lhaid: %e\n", as_result);
    printf("  Relative diff: %e\n", as_reldif);

    assert(reldif < 1e-15);
    assert(as_reldif < 1e-15);

    neopdf_pdf_free(pdf_by_id);
    neopdf_pdf_free(pdf_by_name);

    printf("LHAID loading test passed.\n");
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
    test_load_by_lhaid();
    // test_lazy_loading();

    return EXIT_SUCCESS;
}
