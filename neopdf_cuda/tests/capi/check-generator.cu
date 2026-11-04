// A GPU-resident workflow: phase-space points are generated, the PDFs evaluated
// and the weights reduced on the device, all on one stream. Only the final
// per-member results are copied back. Computes the gluon-gluon luminosity:
//
//     dL/dtau = 1/tau * int dy xg(x1, Q2) xg(x2, Q2),  x1,2 = sqrt(tau)
//     e^(+-y),  Q2 = tau s,
//
// for every member of the set, hence with its replica uncertainty.
#include <NeoPDFCuda.hpp>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cuda_runtime.h>
#include <vector>

// Exits on a failed CUDA runtime call.
static void cuda_check(cudaError_t status) {
    if (status != cudaSuccess) {
        std::fprintf(stderr, "CUDA error: %s\n", cudaGetErrorString(status));
        std::exit(EXIT_FAILURE);
    }
}

// Midpoint rule in rapidity: events [0, n) carry x1, events [n, 2n) carry x2.
__global__ void
generate(size_t n, double tau, double s, double *xs, double *q2s) {
    size_t i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= n) return;
    double y_max = -0.5 * log(tau);
    double y     = -y_max + 2.0 * y_max * (i + 0.5) / n;
    xs[i]        = sqrt(tau) * exp(y);
    xs[n + i]    = sqrt(tau) * exp(-y);
    q2s[i] = q2s[n + i] = tau * s;
}

// One thread per member: sums xg(x1) xg(x2) over the rapidity points.
__global__ void luminosity(size_t n,
    size_t                        num_members,
    size_t                        num_columns,
    size_t                        gluon,
    double                        weight,
    const double                 *pdfs,
    double                       *lumi) {
    size_t m = blockIdx.x * blockDim.x + threadIdx.x;
    if (m >= num_members) return;
    double sum = 0.0;
    for (size_t i = 0; i < n; ++i) {
        sum += pdfs[(i * num_members + m) * num_columns + gluon] *
               pdfs[((n + i) * num_members + m) * num_columns + gluon];
    }
    lumi[m] = weight * sum;
}

int main() {
    const double               sqrt_s = 13600.0;
    const size_t               n      = 4096;

    neopdf::DeviceGrid         grid("NNPDF40_nnlo_as_01180");
    const size_t               num_members = grid.num_members();
    const std::vector<int32_t> pids        = grid.pids();
    const size_t               num_columns = pids.size() + 1;
    size_t                     gluon       = 0;
    for (size_t i = 0; i < pids.size(); ++i) {
        if (pids[i] == 21) gluon = i;
    }

    cudaStream_t stream;
    double      *xs, *q2s, *pdfs, *lumi;
    cuda_check(cudaStreamCreate(&stream));
    cuda_check(cudaMalloc(&xs, 2 * n * sizeof(double)));
    cuda_check(cudaMalloc(&q2s, 2 * n * sizeof(double)));
    cuda_check(cudaMalloc(&pdfs, grid.output_len(2 * n) * sizeof(double)));
    cuda_check(cudaMalloc(&lumi, num_members * sizeof(double)));

    std::printf("     M [GeV]   dL/dtau (central)   replica std dev\n");
    std::printf("---------------------------------------------------\n");
    std::vector<double> result(num_members);
    for (double mass : {125.0, 500.0, 1000.0, 3000.0}) {
        const double tau = mass * mass / (sqrt_s * sqrt_s);
        const double weight =
            -std::log(tau) / static_cast<double>(n) / tau; // dy / tau

        generate<<<(n + 255) / 256, 256, 0, stream>>>(n,
            tau,
            sqrt_s * sqrt_s,
            xs,
            q2s);
        grid.xfxQ2_batch_device(xs, q2s, 2 * n, pdfs, stream);
        luminosity<<<(num_members + 127) / 128, 128, 0, stream>>>(n,
            num_members,
            num_columns,
            gluon,
            weight,
            pdfs,
            lumi);
        cuda_check(cudaMemcpyAsync(result.data(),
            lumi,
            num_members * sizeof(double),
            cudaMemcpyDeviceToHost,
            stream));
        cuda_check(cudaStreamSynchronize(stream));

        double mean = 0.0, sq = 0.0;
        for (size_t m = 1; m < num_members; ++m) mean += result[m];
        mean /= static_cast<double>(num_members - 1);
        for (size_t m = 1; m < num_members; ++m)
            sq += (result[m] - mean) * (result[m] - mean);
        std::printf("%12.1f %19.6e %17.6e\n",
            mass,
            result[0],
            std::sqrt(sq / static_cast<double>(num_members - 2)));
    }

    cuda_check(cudaFree(xs));
    cuda_check(cudaFree(q2s));
    cuda_check(cudaFree(pdfs));
    cuda_check(cudaFree(lumi));
    cuda_check(cudaStreamDestroy(stream));
    return EXIT_SUCCESS;
}
