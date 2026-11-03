#pragma once

#include <cstddef>
#include <cstdint>
#include <neopdf_cuda.h>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

/** @brief Object Oriented interface to NeoPDF's GPU batch evaluation. */
namespace neopdf {

/**
 * @brief Page-locked host memory for batch outputs, from `DeviceGrid::host_buffer`.
 *
 * Results are copied into it from the GPU several times faster than into a
 * `std::vector`. Allocating it is slow, so allocate it once and reuse it.
 */
class HostBuffer {
    private:
        /** @brief Underlying raw object. */
        NeoPDFHostBuffer* raw;

        explicit HostBuffer(NeoPDFHostBuffer* raw) : raw(raw) {}
        friend class DeviceGrid;

    public:
        /** @brief Deleted copy semantics; the buffer can be moved. */
        HostBuffer(const HostBuffer&) = delete;
        HostBuffer& operator=(const HostBuffer&) = delete;
        HostBuffer(HostBuffer&& other) noexcept : raw(std::exchange(other.raw, nullptr)) {}
        HostBuffer& operator=(HostBuffer&& other) noexcept {
            std::swap(this->raw, other.raw);
            return *this;
        }

        /** @brief Destructor, releasing the memory. */
        ~HostBuffer() { neopdf_host_buffer_free(this->raw); }

        /** @brief The values, valid while the buffer lives. */
        double* data() { return neopdf_host_buffer_data(this->raw); }

        /** @brief Number of values. */
        size_t size() const { return neopdf_host_buffer_len(this->raw); }
};

/** @brief Every member of a PDF set, resident on a GPU. */
class DeviceGrid {
    private:
        /** @brief Underlying raw object. */
        NeoPDFDeviceGrid* raw;

        /** @brief Throws if a C call did not succeed. */
        static void check(NeopdfCudaResult result) {
            switch (result) {
                case NEOPDF_CUDA_RESULT_SUCCESS:
                    return;
                case NEOPDF_CUDA_RESULT_ERROR_NULL_POINTER:
                    throw std::invalid_argument("neopdf_cuda: null pointer");
                case NEOPDF_CUDA_RESULT_ERROR_INVALID_LENGTH:
                    throw std::length_error("neopdf_cuda: inconsistent batch sizes");
                default:
                    throw std::runtime_error("neopdf_cuda: CUDA driver error");
            }
        }

    public:
        /**
         * @brief Loads every member of a PDF set and uploads it to a GPU.
         * @param pdf_name Name of the PDF set.
         * @param device Ordinal of the CUDA device.
         */
        explicit DeviceGrid(const std::string& pdf_name, size_t device = 0)
            : raw(neopdf_device_grid_new(pdf_name.c_str(), device)) {
            if (this->raw == nullptr) {
                throw std::runtime_error("neopdf_cuda: cannot upload '" + pdf_name + "' to the device");
            }
        }

        /** @brief Deleted copy/move semantics. */
        DeviceGrid(const DeviceGrid&) = delete;
        DeviceGrid(DeviceGrid&&) = delete;
        DeviceGrid& operator=(const DeviceGrid&) = delete;
        DeviceGrid& operator=(DeviceGrid&&) = delete;

        /** @brief Destructor, releasing the device memory. */
        ~DeviceGrid() { neopdf_device_grid_free(this->raw); }

        /** @brief Number of members on the device. */
        size_t num_members() const { return neopdf_device_grid_num_members(this->raw); }

        /** @brief Flavor PIDs, in output column order. */
        std::vector<int32_t> pids() const {
            std::vector<int32_t> pids(neopdf_device_grid_num_pids(this->raw));
            check(neopdf_device_grid_pids(this->raw, pids.data(), pids.size()));
            return pids;
        }

        /** @brief Length of the batch output: `n_events * num_members * (num_pids + 1)`. */
        size_t output_len(size_t n_events) const {
            return neopdf_device_grid_output_len(this->raw, n_events);
        }

        /**
         * @brief Evaluates every member and flavor, plus `alpha_s`, at host points.
         * @return Flat row-major vector of shape `[xs.size()][num_members][num_pids + 1]`,
         *         the last column holding `alpha_s(Q2)`.
         */
        std::vector<double> xfxQ2_batch(const std::vector<double>& xs,
                                        const std::vector<double>& q2s) const {
            if (xs.size() != q2s.size()) {
                throw std::length_error("neopdf_cuda: xs and q2s differ in size");
            }
            std::vector<double> out(this->output_len(xs.size()));
            check(neopdf_device_grid_xfxq2_batch_host(this->raw, xs.data(), q2s.data(),
                                                      xs.size(), out.data(), out.size()));
            return out;
        }

        /**
         * @brief Allocates page-locked host memory for the output of up to `n_events`
         *        events, to pass to the `HostBuffer` overload of `xfxQ2_batch`.
         */
        HostBuffer host_buffer(size_t n_events) const {
            NeoPDFHostBuffer* raw = neopdf_device_grid_host_buffer_new(this->raw, n_events);
            if (raw == nullptr) {
                throw std::runtime_error("neopdf_cuda: cannot allocate page-locked host memory");
            }
            return HostBuffer(raw);
        }

        /**
         * @brief The same evaluation into `out`, whose first `output_len(xs.size())`
         *        values are filled; faster than the `std::vector` version for large
         *        batches.
         */
        void xfxQ2_batch(const std::vector<double>& xs, const std::vector<double>& q2s,
                         HostBuffer& out) const {
            if (xs.size() != q2s.size()) {
                throw std::length_error("neopdf_cuda: xs and q2s differ in size");
            }
            size_t out_len = this->output_len(xs.size());
            if (out.size() < out_len) {
                throw std::length_error("neopdf_cuda: the host buffer is too small");
            }
            check(neopdf_device_grid_xfxq2_batch_host(this->raw, xs.data(), q2s.data(),
                                                      xs.size(), out.data(), out_len));
        }

        /**
         * @brief Enqueues the same evaluation on device buffers, ordered on `stream`.
         *
         * `xs`, `q2s` (`n_events` values each) and `out` (`output_len(n_events)` values)
         * must be device allocations on this grid's device; `stream` is a
         * `cudaStream_t` (`nullptr` for the default stream). Does not synchronize.
         */
        void xfxQ2_batch_device(const double* xs, const double* q2s, size_t n_events,
                                double* out, void* stream = nullptr) const {
            check(neopdf_device_grid_xfxq2_batch_device(this->raw, xs, q2s, n_events, out, stream));
        }
};

} // namespace neopdf
