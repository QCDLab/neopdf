#include <neopdf_capi.h>
#include <NeoPDF.hpp>
#include <cassert>
#include <cmath>
#include <iostream>
#include <string>

using namespace NEOLHAPDF;

const double TOLERANCE = 1e-12;

void test_lhapdf_compatibility_oop() {
    std::cout << "=== Test LHAPDF C++ OOP Compatibility Layer ===\n";

    std::string pdfname = "NNPDF40_nnlo_as_01180";

    // Use the compatibility layer
    setVerbosity(0);
    PDF* neo_lha_pdf = mkPDF(pdfname, 0);

    // Use the native NeoPDF C API for comparison
    NeoPDFWrapper* neo_pdf = neopdf_pdf_load(pdfname.c_str(), 0);

    double x = 1e-4;
    double q2 = 100.0;
    int pid = 21;

    double result = neo_lha_pdf->xfxQ2(pid, x, q2);
    double expected = neopdf_pdf_xfxq2(neo_pdf, pid, x, q2);
    double reldif = std::abs(result - expected) / expected;

    std::cout << "pid=" << pid << ", x=" << x << ", Q2=" << q2 << std::endl;
    std::cout << "Native NeoPDF: " << expected << std::endl;
    std::cout << "NeoPDF (LHAPDF compat): " << result << std::endl;
    std::cout << "Relative difference: " << reldif << std::endl;

    double as_result = neo_lha_pdf->alphasQ2(q2);
    double as_expected = neopdf_pdf_alphas_q2(neo_pdf, q2);
    double as_reldif = std::abs(as_result - as_expected) / as_expected;

    std::cout << "Q2=" << q2 << std::endl;
    std::cout << "Native NeoPDF: " << as_expected << std::endl;
    std::cout << "NeoPDF (LHAPDF compat): " << as_result << std::endl;
    std::cout << "Relative difference: " << as_reldif << std::endl;

    assert(reldif < TOLERANCE);
    assert(as_reldif < TOLERANCE);

    delete neo_lha_pdf;
    neopdf_pdf_free(neo_pdf);

    std::cout << "LHAPDF C++ OOP Compatibility test passed." << std::endl;
}

void test_lhapdf_compatibility_c() {
    printf("=== Test LHAPDF C Compatibility Layer ===\n");

    const char* pdfname = "NNPDF40_nnlo_as_01180";

    // Use the compatibility layer
    initpdfsetbyname(pdfname);
    initpdf(0);
    double x = 0.1;
    double q = 10.0;
    double xfxs[14];
    evolvepdf(x, q, xfxs);
    double a_s = alphaspdf(q);

    // Use the native NeoPDF API for comparison
    NeoPDFWrapper* neo_pdf = neopdf_pdf_load(pdfname, 0);
    double expected_g = neopdf_pdf_xfxq2(neo_pdf, -1, x, q*q);
    double as_expected = neopdf_pdf_alphas_q2(neo_pdf, q*q);

    double reldiff = fabs(xfxs[6] - expected_g) / expected_g;
    double as_reldiff = fabs(a_s - as_expected) / as_expected;

    printf("Comparing gluon PDF at x=0.1, Q=10.0\n");
    printf("Native NeoPDF: %e\n", expected_g);
    printf("NeoPDF (LHAPDF compat): %e\n", xfxs[6]);
    printf("Relative difference: %e\n", reldiff);

    printf("Comparing AlphaS at Q=10.0\n");
    printf("Native NeoPDF: %e\n", as_expected);
    printf("NeoPDF (LHAPDF compat): %e\n", a_s);
    printf("Relative difference: %e\n", as_reldiff);

    assert(reldiff < TOLERANCE);
    assert(as_reldiff < TOLERANCE);

    printf("LHAPDF C Compatibility test passed.\n");
    neopdf_pdf_free(neo_pdf);
}

int main() {
    test_lhapdf_compatibility_oop();
    test_lhapdf_compatibility_c();

    return 0;
}
