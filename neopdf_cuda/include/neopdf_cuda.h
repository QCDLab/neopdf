/*
 * NeoPDF - A fast library for interpolating PDF (GPU interface)
 * Copyright (C) 2024-2025  Tanjona R. Rabemananjara

 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.

 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.

 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef NEOPDF_CUDA_H
#define NEOPDF_CUDA_H

/* Generated with cbindgen:0.26.0 */

#include <stddef.h>
#include <stdint.h>

/**
 * Result codes for `neopdf_cuda` operations.
 */
typedef enum {
  /**
   * Operation completed successfully.
   */
  NEOPDF_CUDA_RESULT_SUCCESS = 0,
  /**
   * A null pointer was encountered where a valid pointer was expected.
   */
  NEOPDF_CUDA_RESULT_ERROR_NULL_POINTER = -1,
  /**
   * The provided length or size argument was invalid.
   */
  NEOPDF_CUDA_RESULT_ERROR_INVALID_LENGTH = -4,
  /**
   * A CUDA driver call (allocation, copy, launch, synchronization) failed.
   */
  NEOPDF_CUDA_RESULT_ERROR_DEVICE = -5,
} NeopdfCudaResult;

/**
 * Opaque pointer to every member of a PDF set, resident on a GPU.
 */
typedef struct NeoPDFDeviceGrid NeoPDFDeviceGrid;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Loads every member of a PDF set and uploads it to GPU `device`.
 *
 * Only collinear `LogBicubic`/`LogChebyshev` sets with a tabulated `alpha_s` are
 * supported. Returns `NULL` if the set can't be laid out for the device or the
 * device can't be initialized. Release the grid with `neopdf_device_grid_free`.
 *
 * # Panics
 *
 * Panics if `pdf_name` is not valid UTF-8 or the set cannot be loaded.
 *
 * # Safety
 *
 * `pdf_name` must be a null-terminated C string.
 */
NeoPDFDeviceGrid *neopdf_device_grid_new(const char *pdf_name, size_t device);

/**
 * Frees a device grid and its device memory.
 *
 * # Safety
 *
 * `grid` must be `NULL` or come from `neopdf_device_grid_new`, and not be used afterwards.
 */
void neopdf_device_grid_free(NeoPDFDeviceGrid *grid);

/**
 * Returns the number of members on the device.
 *
 * # Safety
 *
 * `grid` must be a valid device grid.
 */
size_t neopdf_device_grid_num_members(const NeoPDFDeviceGrid *grid);

/**
 * Returns the number of flavors per member.
 *
 * # Safety
 *
 * `grid` must be a valid device grid.
 */
size_t neopdf_device_grid_num_pids(const NeoPDFDeviceGrid *grid);

/**
 * Writes the flavor PIDs, in output column order, into `pids`.
 *
 * # Safety
 *
 * `grid` must be a valid device grid and `pids` must hold `num_pids` values, with
 * `num_pids` equal to `neopdf_device_grid_num_pids(grid)`.
 */
NeopdfCudaResult neopdf_device_grid_pids(const NeoPDFDeviceGrid *grid,
                                         int32_t *pids,
                                         size_t num_pids);

/**
 * Returns the length of the batch output for `n_events` events:
 * `n_events * num_members * (num_pids + 1)`.
 *
 * # Safety
 *
 * `grid` must be a valid device grid.
 */
size_t neopdf_device_grid_output_len(const NeoPDFDeviceGrid *grid, size_t n_events);

/**
 * Evaluates every member and flavor, plus `alpha_s`, at `n_events` host-resident
 * `(xs[i], q2s[i])` points into the host-resident `out`, returning once `out` is filled.
 *
 * `out` is laid out `[n_events][num_members][num_pids + 1]`, the last column holding
 * `alpha_s(Q²)`, and `out_len` must equal `neopdf_device_grid_output_len(grid, n_events)`.
 *
 * # Safety
 *
 * `grid` must be a valid device grid, `xs`/`q2s` must hold `n_events` values and
 * `out` must hold `out_len` values.
 */
NeopdfCudaResult neopdf_device_grid_xfxq2_batch_host(const NeoPDFDeviceGrid *grid,
                                                     const double *xs,
                                                     const double *q2s,
                                                     size_t n_events,
                                                     double *out,
                                                     size_t out_len);

/**
 * Enqueues the same evaluation as `neopdf_device_grid_xfxq2_batch_host` on
 * device-resident buffers, ordered on the caller's CUDA `stream`.
 *
 * Does not synchronize the host: the results are in `out` once `stream` reaches
 * this point.
 *
 * `stream` is a `CUstream`/`cudaStream_t` of the same device (`NULL` for the default
 * stream); `out` must hold `neopdf_device_grid_output_len(grid, n_events)` values.
 *
 * # Safety
 *
 * `grid` must be a valid device grid; `xs`, `q2s` and `out` must be device
 * allocations (e.g. from `cudaMalloc`) on the grid's device, of the sizes above.
 */
NeopdfCudaResult neopdf_device_grid_xfxq2_batch_device(const NeoPDFDeviceGrid *grid,
                                                       const double *xs,
                                                       const double *q2s,
                                                       size_t n_events,
                                                       double *out,
                                                       void *stream);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* NEOPDF_CUDA_H */
