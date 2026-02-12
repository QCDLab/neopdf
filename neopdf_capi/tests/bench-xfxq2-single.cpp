#include "NeoPDF.hpp"
#include <LHAPDF/PDF.h>
#include <neopdf_capi.h>

#include <chrono>
#include <cassert>
#include <cmath>
#include <cstdlib>
#include <iomanip>
#include <iostream>

int main() {
    LHAPDF::setVerbosity(0);

    std::string pdfname = "NNPDF40_nnlo_as_01180";
    neopdf::NeoPDF* neo_pdf;
    neo_pdf = new neopdf::NeoPDF(pdfname.c_str(), 0);
    auto lha_pdf = std::unique_ptr<LHAPDF::PDF>(LHAPDF::mkPDF(pdfname, 0));

    const int pid = 21;
    const double x = 1e-3;
    const double q2 = 4.0;
    const int N = 10000000;

    // Warm-up both libraries
    for (int w = 0; w < 1000; ++w) {
        volatile double r1 = lha_pdf->xfxQ2(pid, x, q2);
        volatile double r2 = neo_pdf->xfxQ2(pid, x, q2);
        (void)r1; (void)r2;
    }

    // Benchmark LHAPDF
    volatile double sink = 0.0;
    auto t0 = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < N; ++i) {
        sink = lha_pdf->xfxQ2(pid, x, q2);
    }
    auto t1 = std::chrono::high_resolution_clock::now();

    // Benchmark NeoPDF
    auto t2 = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < N; ++i) {
        sink = neo_pdf->xfxQ2(pid, x, q2);
    }
    auto t3 = std::chrono::high_resolution_clock::now();

    double lhapdf_ns = std::chrono::duration<double, std::nano>(t1 - t0).count() / N;
    double neopdf_ns = std::chrono::duration<double, std::nano>(t3 - t2).count() / N;

    std::cout << std::fixed << std::setprecision(1);
    std::cout << "=== Single-point xfxQ2 benchmark ===\n";
    std::cout << "PDF:        " << pdfname << "\n";
    std::cout << "pid=" << pid
              << "  x=" << std::scientific << std::setprecision(1) << x
              << "  Q2=" << q2 << "\n";
    std::cout << "Repeats:    " << N << "\n\n";
    std::cout << std::fixed << std::setprecision(1);

    assert(neopdf_ns < lhapdf_ns);
    std::cout << "NeoPDF is faster than LHAPDF!" << std::endl;
    // std::cout << "LHAPDF:     " << lhapdf_ns << " ns/call\n";
    // std::cout << "NeoPDF:     " << neopdf_ns << " ns/call\n";
    // std::cout << "Ratio:      " << neopdf_ns / lhapdf_ns << "x\n";

    (void)sink;
    return EXIT_SUCCESS;
}
