#include "neopdf_tmdlib/src/tmdlib.hpp"
#include "tmdlib/TMDlib.h"
#include <iostream>
#include <string>
#include <utility>
#include <vector>
#include <iterator>
#include <cmath>

namespace {

// TMDlib writes grid-lookup diagnostics with `std::cout`. The CLI stdout is
// only the numeric result, so keep that chatter on stderr for the call.
template <typename F>
void quiet_tmdlib(F&& call) {
    struct Redirect {
        std::streambuf* previous;

        Redirect() : previous(std::cout.rdbuf(std::cerr.rdbuf())) {}
        ~Redirect() { std::cout.rdbuf(previous); }
    } redirect;

    std::forward<F>(call)();
}

} // namespace

template <typename T>
rust::Vec<T> std_vector_to_rust_vec(std::vector<T> vector)
{
    rust::Vec<T> result;
    result.reserve(vector.size());
    std::move(vector.begin(), vector.end(), std::back_inserter(result));
    return result;
}

std::unique_ptr<TMDlib::TMD> make_tmd() {
    return std::unique_ptr<TMDlib::TMD>(new TMDlib::TMD());
}

void tmd_init(TMDlib::TMD& tmd, rust::Str setname, int member) {
    quiet_tmdlib([&] { tmd.TMDinit(std::string(setname), member); });
}

void tmd_init_set(TMDlib::TMD& tmd, rust::Str setname) {
    quiet_tmdlib([&] { tmd.TMDinit(std::string(setname)); });
}

size_t tmd_get_num_members(TMDlib::TMD& tmd) {
    return tmd.TMDgetNumMembers();
}

double tmd_get_xmin(TMDlib::TMD& tmd) {
    return tmd.TMDgetXmin();
}

double tmd_get_xmax(TMDlib::TMD& tmd) {
    return tmd.TMDgetXmax();
}

double tmd_get_q2min(TMDlib::TMD& tmd) {
    return tmd.TMDgetQ2min();
}

double tmd_get_q2max(TMDlib::TMD& tmd) {
    return tmd.TMDgetQ2max();
}

double tmd_get_ktmin(TMDlib::TMD& tmd) {
    return tmd.TMDgetKtmin();
}

double tmd_get_ktmax(TMDlib::TMD& tmd) {
    return tmd.TMDgetKtmax();
}

rust::Vec<double> tmd_pdf(TMDlib::TMD& tmd, double x, double kt, double q) {
    std::vector<double> pdfs;
    quiet_tmdlib([&] { pdfs = tmd.TMDpdf(x, 0.0, kt, q); });
    return std_vector_to_rust_vec(std::move(pdfs));
}

void tmd_set_verbosity(TMDlib::TMD& tmd, int verbosity) {
    quiet_tmdlib([&] { tmd.setVerbosity(verbosity); });
}
