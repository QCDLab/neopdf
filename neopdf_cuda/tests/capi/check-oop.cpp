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

int main() {
    neopdf::DeviceGrid   grid("NNPDF40_nnlo_as_01180");
    std::vector<int32_t> pids        = grid.pids();
    size_t               num_members = grid.num_members();
    size_t               num_columns = pids.size() + 1;
    size_t               gluon       = 0;
    for (size_t i = 0; i < pids.size(); ++i) {
        if (pids[i] == 21) gluon = i;
    }

    std::vector<double> xs  = {1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 3e-1, 5e-1, 8e-1};
    std::vector<double> q2s = {4.0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e8};
    size_t              num_events = xs.size();
    size_t              out_len    = grid.output_len(num_events);

    // Host interface.
    std::vector<double> host       = grid.xfxQ2_batch(xs, q2s);

    // Device interface, as an event generator would use it: the buffers and the
    // stream belong to the caller, and the evaluation is ordered on that
    // stream.
    cudaStream_t        stream;
    double             *xs_dev, *q2s_dev, *out_dev;
    cuda_check(cudaStreamCreate(&stream));
    cuda_check(cudaMalloc(&xs_dev, num_events * sizeof(double)));
    cuda_check(cudaMalloc(&q2s_dev, num_events * sizeof(double)));
    cuda_check(cudaMalloc(&out_dev, out_len * sizeof(double)));
    cuda_check(cudaMemcpyAsync(xs_dev,
        xs.data(),
        num_events * sizeof(double),
        cudaMemcpyHostToDevice,
        stream));
    cuda_check(cudaMemcpyAsync(q2s_dev,
        q2s.data(),
        num_events * sizeof(double),
        cudaMemcpyHostToDevice,
        stream));
    grid.xfxQ2_batch_device(xs_dev, q2s_dev, num_events, out_dev, stream);
    std::vector<double> device(out_len);
    cuda_check(cudaMemcpyAsync(device.data(),
        out_dev,
        out_len * sizeof(double),
        cudaMemcpyDeviceToHost,
        stream));
    cuda_check(cudaStreamSynchronize(stream));

    size_t mismatches = 0;
    for (size_t i = 0; i < out_len; ++i) {
        if (host[i] != device[i]) ++mismatches;
    }
    std::printf("members: %zu, entries: %zu, host/device mismatches: %zu\n",
        num_members,
        out_len,
        mismatches);

    std::printf("              x              Q2       xg (central)      xg "
                "(mean)    xg (std dev)\n");
    std::printf("--------------------------------------------------------------"
                "---------------------\n");
    for (size_t e = 0; e < num_events; ++e) {
        auto xg = [&](size_t member) {
            return device[(e * num_members + member) * num_columns + gluon];
        };
        double mean = 0.0, sq = 0.0;
        for (size_t m = 1; m < num_members; ++m) mean += xg(m);
        mean /= static_cast<double>(num_members - 1);
        for (size_t m = 1; m < num_members; ++m)
            sq += (xg(m) - mean) * (xg(m) - mean);
        double std_dev = std::sqrt(sq / static_cast<double>(num_members - 2));
        std::printf("%15.8e %15.8e %15.8e %15.8e %15.8e\n",
            xs[e],
            q2s[e],
            xg(0),
            mean,
            std_dev);
    }

    cuda_check(cudaFree(xs_dev));
    cuda_check(cudaFree(q2s_dev));
    cuda_check(cudaFree(out_dev));
    cuda_check(cudaStreamDestroy(stream));
    return mismatches == 0 ? EXIT_SUCCESS : EXIT_FAILURE;
}
