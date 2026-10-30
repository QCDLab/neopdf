#pragma once

#include <cstddef>
#include <cstdint>
#include <neopdf_cuda.h>
#include <stdexcept>
#include <string>
#include <vector>

/** @brief Object Oriented interface to NeoPDF's GPU batch evaluation. */
namespace neopdf {

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
