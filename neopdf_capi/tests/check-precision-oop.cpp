#include <NeoPDF.hpp>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <stdexcept>

using namespace neopdf;

/* A number type whose logarithm throws, to check that its exceptions reach the
 * caller. */
struct Throwing {
    double value;
    explicit Throwing(double v) : value(v) {}
};

Throwing operator+(const Throwing &a, const Throwing &b) {
    return Throwing(a.value + b.value);
}
Throwing operator-(const Throwing &a, const Throwing &b) {
    return Throwing(a.value - b.value);
}
Throwing operator*(const Throwing &a, const Throwing &b) {
    return Throwing(a.value * b.value);
}
Throwing operator/(const Throwing &a, const Throwing &b) {
    return Throwing(a.value / b.value);
}
bool operator<(const Throwing &a, const Throwing &b) {
    return a.value < b.value;
}
bool operator>(const Throwing &a, const Throwing &b) {
    return a.value > b.value;
}
bool operator==(const Throwing &a, const Throwing &b) {
    return a.value == b.value;
}
bool     isfinite(const Throwing &a) { return std::isfinite(a.value); }
Throwing log(const Throwing &) { throw std::domain_error("log of Throwing"); }

void     test_long_double() {
    std::printf("=== Test the long double view against the double API ===\n");

    NeoPDF                    pdf("NNPDF40_nnlo_as_01180");
    PrecisionPDF<long double> view = pdf.with_arithmetic<long double>();

    std::vector<int>          pids = {-5, -2, -1, 1, 2, 4, 21};
    std::vector<double>       xs   = {1e-7, 1e-4, 1e-2, 0.3, 0.8};
    std::vector<double>       q2s  = {3.0, 25.0, 1e4, 1e6};

    std::printf("%6s %12s %12s %20s\n", "pid", "x", "Q2", "long double");
    std::printf("------------------------------------------------------\n");
    for (int pid : pids) {
        for (double x : xs) {
            for (double q2 : q2s) {
                long double xf        = view.xfxQ2(pid, x, q2);
                double      reference = pdf.xfxQ2(pid, x, q2);
                assert(std::fabs(xf - reference) <=
                       1e-13 * std::fabs(reference) + 1e-300);
                std::printf("%6d %12.4e %12.4e %20.12Le\n", pid, x, q2, xf);
            }
        }
    }
    assert(view.xfxQ2(6, 0.1L, 100.0L) == 0.0L);
    long double alphas = view.alphasQ2(8315.0L);
    assert(std::fabs(alphas - pdf.alphasQ2(8315.0)) <= 1e-15);
    std::printf("alpha_s(Q2 = 8315) = %.12Le\n\n", alphas);
}

void test_errors() {
    std::printf("=== Test the errors of the view ===\n");

    NeoPDF               pdf("NNPDF40_nnlo_as_01180");
    PrecisionPDF<double> view(pdf);
    try {
        view.xfxQ2(21, 1e-12, 100.0);
        assert(false);
    } catch (const PrecisionError &error) {
        std::printf("x outside the grid:  %d (%s)\n",
            error.code(),
            error.what());
    }
    try {
        view.xfxQ2(21, NAN, 100.0);
        assert(false);
    } catch (const PrecisionError &error) {
        std::printf("non-finite x:        %d (%s)\n",
            error.code(),
            error.what());
    }
    try {
        view.alphasQ2(1.0);
        assert(false);
    } catch (const PrecisionError &error) {
        std::printf("Q2 outside alpha_s:  %d (%s)\n",
            error.code(),
            error.what());
    }

    // An exception thrown by the number type is rethrown as it is.
    try {
        PrecisionPDF<Throwing> throwing(pdf);
        assert(false);
    } catch (const std::domain_error &error) {
        std::printf("exception of T:      %s\n", error.what());
    }

    NeoPDF tmd("MAP22_grids_FF_Km_N3LL.neopdf.lz4");
    try {
        PrecisionPDF<double> unsupported(tmd);
        assert(false);
    } catch (const PrecisionError &error) {
        std::printf("unsupported set:     %d (%s)\n",
            error.code(),
            error.what());
    }
}

int main() {
    test_long_double();
    test_errors();
    return 0;
}
