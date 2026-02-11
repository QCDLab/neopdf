#include <LHAPDF/PDF.h>
#include <LHAPDF/GridPDF.h>
#include <NeoPDF.hpp>
#include <cassert>
#include <cmath>
#include <iomanip>
#include <iostream>
#include <string>
#include <vector>

using namespace neopdf;

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

template<typename T>
std::vector<T> linspace(T start, T stop, int num, bool endpoint = true) {
    std::vector<T> result(num);

    if (num == 1) {
        result[0] = start;
        return result;
    }

    T step = (stop - start) / (endpoint ? (num - 1) : num);

    for (int i = 0; i < num; ++i) {
        result[i] = start + i * step;
    }

    return result;
}

void test_xfxq2() {
    std::cout << "=== Test xfxQ2 for single PDF member ===\n";

    // disable LHAPDF banners to guarantee deterministic output
    LHAPDF::setVerbosity(0);

    std::string pdfname = "NNPDF40_nnlo_as_01180";
    NeoPDF neo_pdf(pdfname.c_str(), 0);
    const LHAPDF::PDF* basepdf = LHAPDF::mkPDF(pdfname);
    const LHAPDF::GridPDF& lha_pdf = * dynamic_cast<const LHAPDF::GridPDF*>(basepdf);

    std::vector<int> pids = {-5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5};
    std::vector<double> xs = geomspace(neo_pdf.x_min(), neo_pdf.x_max(), 200);
    std::vector<double> q2s = geomspace(neo_pdf.q2_min(), neo_pdf.q2_max(), 200);

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
                double expected = lha_pdf.xfxQ2(pid, x, q2);
                double result = neo_pdf.xfxQ2(pid, x, q2);
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
}

void test_alphas_q2() {
    std::cout << "=== Test alphasQ2 for single PDF member ===\n";

    // disable LHAPDF banners to guarantee deterministic output
    LHAPDF::setVerbosity(0);

    std::string pdfname = "NNPDF40_nnlo_as_01180";
    NeoPDF neo_pdf(pdfname.c_str(), 0);
    const LHAPDF::PDF* basepdf = LHAPDF::mkPDF(pdfname);
    const LHAPDF::GridPDF& lha_pdf = * dynamic_cast<const LHAPDF::GridPDF*>(basepdf);

    std::vector<double> q2_points = linspace(4.0, 1e10, 500);

    // Headers of the table to print the results
    std::cout << std::right
        << std::setw(15) << "Q2"
        << std::setw(15) << "LHAPDF"
        << std::setw(15) << "NeoPDF"
        << std::setw(15) << "Rel. Diff." << "\n";
    std::cout << std::string(60, '-') << "\n";

    for (const auto& q2: q2_points) {
        double expected = lha_pdf.alphasQ2(q2);
        double result = neo_pdf.alphasQ2(q2);
        double reldif = std::abs(result - expected) / expected;

        assert(std::abs(result - expected) < TOLERANCE);

        // Print the results as a table
        std::cout << std::scientific << std::setprecision(8)
            << std::right
            << std::setw(15) << q2
            << std::right
            << std::setw(15) << expected
            << std::setw(15) << result
            << std::setw(15) << reldif << "\n";
    }
}

void test_all_pdf_members() {
    std::cout << "=== Test PDFs class (loading all members) ===\n";

    // disable LHAPDF banners to guarantee deterministic output
    LHAPDF::setVerbosity(0);

    std::string pdfname = "NNPDF40_nnlo_as_01180";
    NeoPDFs neo_pdfs(pdfname.c_str());

    std::cout << "Loaded " << neo_pdfs.size() << " PDF members\n";

    // Test case: evaluate a simple point across all members
    int pid = 1;
    double x = 1e-9;
    double q2 = 1.65 * 1.65;

    std::cout << std::right
        << std::setw(8) << "Member"
        << std::setw(15) << "LHAPDF"
        << std::setw(15) << "NeoPDF"
        << std::setw(15) << "Rel. Diff." << "\n";
    std::cout << std::string(53, '-') << "\n";

    // Evaluate the same point across all PDF members
    std::vector<double> results;
    for (size_t i = 0; i < neo_pdfs.size(); ++i) {
        const LHAPDF::PDF* basepdf = LHAPDF::mkPDF(pdfname, i);
        const LHAPDF::GridPDF& lha_pdf = * dynamic_cast<const LHAPDF::GridPDF*>(basepdf);

        double expected = lha_pdf.xfxQ2(pid, x, q2);
        double result = neo_pdfs[i].xfxQ2(pid, x, q2);

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
}

void test_lazy_loading() {
    std::cout << "=== Test NeoPDFLazy class (lazy loading) ===\n";

    // Disable LHAPDF banners to guarantee deterministic output
    LHAPDF::setVerbosity(0);

    std::string pdfname = "NNPDF40_nnlo_as_01180.neopdf.lz4";
    NeoPDFLazy lazy_pdfs(pdfname);

    std::cout << "Initialized lazy loader for " << pdfname << "\n";

    // Test case: evaluate a simple point across all members
    int pid = 1;
    double x = 1e-9;
    double q2 = 1.65 * 1.65;

    std::cout << std::right
        << std::setw(8) << "Member"
        << std::setw(15) << "LHAPDF"
        << std::setw(15) << "NeoPDF"
        << std::setw(15) << "Rel. Diff." << "\n";
    std::cout << std::string(53, '-') << "\n";

    // Evaluate the same point across all PDF members
    std::vector<double> results;
    int member_idx = 0;
    while (auto neo_pdf = lazy_pdfs.next()) {
        const LHAPDF::PDF* basepdf = LHAPDF::mkPDF("NNPDF40_nnlo_as_01180", member_idx);
        const LHAPDF::GridPDF& lha_pdf = *dynamic_cast<const LHAPDF::GridPDF*>(basepdf);

        double expected = lha_pdf.xfxQ2(pid, x, q2);
        double result = neo_pdf->xfxQ2(pid, x, q2);

        double reldif = std::abs(result - expected) / expected;
        assert(std::abs(result - expected) < TOLERANCE);
        results.push_back(result);

        std::cout << std::right
            << std::setw(8) << member_idx
            << std::scientific << std::setprecision(8)
            << std::setw(15) << expected
            << std::setw(15) << result
            << std::setw(15) << reldif << "\n";
        member_idx++;
    }

    std::cout << "\nSuccessfully iterated through all members lazily.\n";
}

void test_xfxq2_allpids() {
    std::cout << "=== Test xfxQ2_allpids via OOP ===\n";

    LHAPDF::setVerbosity(0);

    std::string pdfname = "NNPDF40_nnlo_as_01180";
    NeoPDF neo_pdf(pdfname.c_str(), 0);
    const LHAPDF::PDF* basepdf = LHAPDF::mkPDF(pdfname);
    const LHAPDF::GridPDF& lha_pdf = *dynamic_cast<const LHAPDF::GridPDF*>(basepdf);

    std::vector<int32_t> pids = {-5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5};
    std::vector<double> xs = geomspace(neo_pdf.x_min(), neo_pdf.x_max(), 50);
    std::vector<double> q2s = geomspace(neo_pdf.q2_min(), neo_pdf.q2_max(), 50);

    std::cout << std::right
        << std::setw(6) << "pid"
        << std::setw(15) << "x"
        << std::setw(15) << "Q2"
        << std::setw(15) << "LHAPDF"
        << std::setw(15) << "NeoPDF"
        << std::setw(15) << "Rel. Diff." << "\n";
    std::cout << std::string(81, '-') << "\n";

    for (const auto &x : xs) {
        for (const auto &q2 : q2s) {
            std::vector<double> results = neo_pdf.xfxQ2_allpids(pids, {x, q2});

            for (size_t i = 0; i < pids.size(); ++i) {
                double expected = lha_pdf.xfxQ2(pids[i], x, q2);
                double reldif = std::abs(results[i] - expected) / std::abs(expected);

                assert(std::abs(results[i] - expected) < TOLERANCE);

                std::cout << std::scientific << std::setprecision(8)
                    << std::right
                    << std::setw(6)  << pids[i]
                    << std::setw(15) << x
                    << std::setw(15) << q2
                    << std::setw(15) << expected
                    << std::setw(15) << results[i]
                    << std::setw(15) << reldif << "\n";
            }
        }
    }
}

void test_xfxq2s() {
    std::cout << "=== Test xfxQ2s via OOP ===\n";

    LHAPDF::setVerbosity(0);

    std::string pdfname = "NNPDF40_nnlo_as_01180";
    NeoPDF neo_pdf(pdfname.c_str(), 0);
    const LHAPDF::PDF* basepdf = LHAPDF::mkPDF(pdfname);
    const LHAPDF::GridPDF& lha_pdf = *dynamic_cast<const LHAPDF::GridPDF*>(basepdf);

    std::vector<int32_t> pids = {-5, -3, -1, 21, 1, 3, 5};
    std::vector<double> xs = geomspace(neo_pdf.x_min(), neo_pdf.x_max(), 30);
    std::vector<double> q2s = geomspace(neo_pdf.q2_min(), neo_pdf.q2_max(), 30);

    // Build kinematic points: each point is {x, q2}
    std::vector<std::vector<double>> points;
    for (const auto &x : xs) {
        for (const auto &q2 : q2s) {
            points.push_back({x, q2});
        }
    }

    std::vector<double> results = neo_pdf.xfxQ2s(pids, points);

    std::cout << std::right
        << std::setw(6) << "pid"
        << std::setw(15) << "x"
        << std::setw(15) << "Q2"
        << std::setw(15) << "LHAPDF"
        << std::setw(15) << "NeoPDF"
        << std::setw(15) << "Rel. Diff." << "\n";
    std::cout << std::string(81, '-') << "\n";

    size_t num_points = points.size();
    for (size_t p = 0; p < pids.size(); ++p) {
        for (size_t k = 0; k < num_points; ++k) {
            double expected = lha_pdf.xfxQ2(pids[p], points[k][0], points[k][1]);
            double result = results[p * num_points + k];
            double reldif = std::abs(result - expected) / std::abs(expected);

            assert(std::abs(result - expected) < TOLERANCE);

            std::cout << std::scientific << std::setprecision(8)
                << std::right
                << std::setw(6)  << pids[p]
                << std::setw(15) << points[k][0]
                << std::setw(15) << points[k][1]
                << std::setw(15) << expected
                << std::setw(15) << result
                << std::setw(15) << reldif << "\n";
        }
    }
}

int main() {
    // Test the computation of the PDF interpolations
    test_xfxq2();

    // Test the computation of the `alphas` interpolations
    test_alphas_q2();

    // Test the PDF interpolations by loading all the members
    test_all_pdf_members();

    // Test the lazy loading of PDF members
    test_lazy_loading();

    // Test xfxQ2_allpids
    test_xfxq2_allpids();

    // Test xfxQ2s
    test_xfxq2s();

    return EXIT_SUCCESS;
}
