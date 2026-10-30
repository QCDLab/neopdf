# The NeOPDF GPU support with CUDA

GPU batch evaluation of NeoPDF's collinear PDF grids, built with
[cuda-oxide](https://github.com/NVIDIA/cuda-rust). Every member
of a set is uploaded once and each call evaluates all members and
flavors, plus `alpha_s`, at a batch of `(x, Q^2)` points, producing
a `[n_events][n_members][n_flavors + 1]` array.

Currently, this only supports `LogBicubic` and 2D `LogChebyshev`
sets with a tabulated `alpha_s`, in f64. Furthremore, it needs an
`sm_80+` GPU with a CUDA 13 (R580+) driver.

For the time being, this crate is standalone (outside the NeoPDF
workspace) because cuda-oxide pins its own toolchain
(`rust-toolchain.toml`) and is only available from git.

## Building and testing

```sh
cargo test                                   # host-only layout/evaluation, any GPU-less machine
cargo oxide test -- --features cuda          # + device kernel against the host evaluation
cargo oxide build -- --features capi --release
make -C tests/capi test-examples             # C and C++ interfaces
```

## C ABI and C++ Interface

In the `neopdf_cuda` directory, running:

```bash
cargo oxide build -- --features capi --release
```

produces a `target/release/libneopdf_cuda.so` and generates
`include/neopdf_cuda.h` and the C++ header `include/NeoPDFCuda.hpp`
is just a wrapper on it.

The following is a simple example on how to use the header:

```cpp
#include <NeoPDFCuda.hpp>

neopdf::DeviceGrid grid("NNPDF40_nnlo_as_01180"); // all members, uploaded once

// Host arrays in and out:
std::vector<double> out = grid.xfxQ2_batch(xs, q2s);

// Or, inside a GPU event generator: caller-owned device buffers, ordered on the
// caller's stream, without synchronizing the host.
grid.xfxQ2_batch_device(xs_dev, q2s_dev, n_events, out_dev, stream);
```

See `tests/capi/check-oop.cpp` and `tests/capi/check-generator.cu`
for complete examples, `examples/` for Rust ones, and the "GPU API"
page of the documentation for a walkthrough.
