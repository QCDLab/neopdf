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
