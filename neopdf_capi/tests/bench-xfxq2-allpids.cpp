#include <LHAPDF/PDF.h>
#include <chrono>
#include <cmath>
#include <cstdlib>
#include <iomanip>
#include <iostream>
#include <vector>

#include "NeoPDF.hpp"

int main() {
    LHAPDF::setVerbosity(0);

    std::string pdfname = "NNPDF40_nnlo_as_01180";
    neopdf::NeoPDF* neo_pdf = new neopdf::NeoPDF(pdfname.c_str(), 0);
    auto lha_pdf = std::unique_ptr<LHAPDF::PDF>(LHAPDF::mkPDF(pdfname, 0));

    // NeoPDF (C compatibility layer)
    initpdfsetbyname(pdfname.c_str());
    initpdf(0);

    const double x = 1e-3;
    const double q2 = 4.0;
    const int N = 10000000;
    // `xfxQ2_pids` takes as inputs a vector of parameters. It is the
    // generalized form because a set might depend on more parameters.
    std::vector<double> kins = { x, q2 };
    std::vector<int32_t> pids = { -5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5 };

    // Warm-up both libraries
    std::vector<double> lha_xfs(13);
    std::vector<double> neo_xfs;
    for (int w = 0; w < 1000; ++w) {
        lha_pdf->xfxQ2(x, q2, lha_xfs);
        neo_xfs = neo_pdf->xfxQ2_pids(pids, kins);
    }

    // Benchmark LHAPDF: xfxQ2(x, Q2, vector) — all flavors at once
    volatile double sink = 0.0;
    auto t0 = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < N; ++i) {
        lha_pdf->xfxQ2(x, q2, lha_xfs);
        sink = lha_xfs[6]; // gluon
    }
    auto t1 = std::chrono::high_resolution_clock::now();

    // Benchmark NeoPDF: evolvepdf(x, Q, xfxs) — all flavors at once
    auto t2 = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < N; ++i) {
        neo_xfs = neo_pdf->xfxQ2_pids(pids, kins);
        sink = neo_xfs[6]; // gluon
    }
    auto t3 = std::chrono::high_resolution_clock::now();

    double lhapdf_ns = std::chrono::duration<double, std::nano>(t1 - t0).count() / N;
    double neopdf_ns = std::chrono::duration<double, std::nano>(t3 - t2).count() / N;

    std::cout << std::fixed << std::setprecision(1);
    std::cout << "=== All-flavor evolvepdf benchmark ===\n";
    std::cout << "PDF:        " << pdfname << "\n";
    std::cout << "x=" << std::scientific << std::setprecision(1) << x
              << "  Q2=" << q2 << "\n";
    std::cout << "Repeats:    " << N << "\n\n";
    std::cout << std::fixed << std::setprecision(1);

    if (!(neopdf_ns < lhapdf_ns)) {
        std::cerr << "Assertion failed: neopdf_ns < lhapdf_ns\n"
                  << "neopdf_ns = " << neopdf_ns << "\n"
                  << "lhapdf_ns = " << lhapdf_ns << std::endl;
        std::abort();
    }
    std::cout << "NeoPDF is faster than LHAPDF!" << std::endl;
    // std::cout << "LHAPDF:     " << lhapdf_ns << " ns/call  (xfxQ2 -> vector)\n";
    // std::cout << "NeoPDF:     " << neopdf_ns << " ns/call  (evolvepdf)\n";
    // std::cout << "Ratio:      " << neopdf_ns / lhapdf_ns << "x\n";

    (void)sink;
    return EXIT_SUCCESS;
}
