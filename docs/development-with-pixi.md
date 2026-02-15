# Development with Pixi

This guide provides comprehensive instructions for developing `NeoPDF` using [Pixi](https://pixi.sh/latest/),
a modern package management tool that ensures reproducible development environments across
different platforms.

!!! danger "Why Pixi?"

    `Pixi` provides isolated, reproducible environments with built-in lockfiles, making it perfect
    for managing complex multi-language projects like `NeoPDF` that involve Rust, Python, C/C++,
    and Fortran components.

---

## Prerequisites

### Installing Pixi

First, you need to install `Pixi`. Depending on your platform, run one of the following commands:

**Linux & macOS:**

```bash
curl -fsSL https://pixi.sh/install.sh | sh
```

**Windows:**

```powershell
powershell -ExecutionPolicy ByPass -c "irm -useb https://pixi.sh/install.ps1 | iex"
```

After the installation, restart your terminal or source your shell configuration.

---

## Project Setup

### Initializing the Development Environment

1. **Clone the repository and navigate to the project:**

   ```bash
   git clone https://github.com/Radonirinaunimi/neopdf
   cd neopdf
   ```

   This directory contains the `pixi.toml` configuration file which contains the list of
   dependencies and tasks. For more details on how to configure the `pixi.toml` file, head
   over to the [documentation](https://pixi.sh/latest/reference/pixi_manifest/).

2. **Verify the environment:**

   ```bash
   # Check Rust version
   pixi run rustc --version

   # Check Python version
   pixi run python --version

   # Check C compiler
   pixi run gcc --version
   ```

!!! danger "Omitting `pixi run`"

    One can omit the `pixi run` prefix by invoking the `pixi` shell:

    ```bash
    pixi shell
    ```

    This activates the development environment with all dependencies installed.

---

## Core Components

### Rust Library (`neopdf/`)

The core Rust library provides the main interpolation functionality.

**Building the library:**

```bash
# Build in debug mode
pixi run cargo build --manifest-path neopdf/Cargo.toml

# Build in release mode
pixi run cargo build --release --manifest-path neopdf/Cargo.toml
```

**Running tests:**

```bash
# Run all Rust tests
pixi run test-rust

# Run specific test file
pixi run cargo test --no-fail-fast --manifest-path neopdf/Cargo.toml --test pdf

# Run with verbose output
pixi run cargo test --no-fail-fast --manifest-path neopdf/Cargo.toml -- --nocapture
```

**Running benchmarks:**

```bash
pixi run cargo bench --manifest-path neopdf/Cargo.toml
```

### Python API (`neopdf_pyapi/`)

The Python API provides high-level bindings to the Rust library using PyO3. For
more examples on using the Python API, see the [tutorials](https://radonirinaunimi.github.io/neopdf/examples/neopdf-pyapi/).

**Installing the Python API:**

```bash
# Install in development mode
pixi run install-pyapi

# Install with test dependencies
pixi run maturin develop --manifest-path neopdf_pyapi/Cargo.toml --extras test
```

**Running Python tests:**

```bash
# Run all Python tests
pixi run test-pyapi

# Run with coverage
pixi run pytest neopdf_pyapi/tests --cov=neopdf_hep

# Run specific test file
pixi run pytest neopdf_pyapi/tests/test_pdfs.py -v
```

### C/C++ API (`neopdf_capi/`)

The C-API provides low-level bindings for C and C++ applications. For more examples
on using the C/C++ API, see the corresponding [tutorials](examples/c-oop.md).

**Installing the C-API:**

```bash
# Install C-API
pixi run install-capi

# Install with custom prefix
pixi run cargo cinstall --manifest-path neopdf_capi/Cargo.toml --prefix=/usr/local
```

**Running C-API tests:**

```bash
# Run C-API tests
pixi run test-capi

# Run specific test
pixi run make -C neopdf_capi/tests check-capi
```

### Command Line Interface (`neopdf_cli/`)

The CLI provides command-line tools for working with `NeoPDF` files. For more illustrations
on how to use the CLI, head over to the [tutorials](cli-tutorials.md)

**Installing the CLI:**

```bash
# Install CLI
pixi run install-cli

# Install in debug mode
pixi run cargo install --path neopdf_cli --debug
```

**Using the CLI:**

```bash
# Show help
pixi run neopdf --help

# List available PDF sets
pixi run neopdf list

# Convert LHAPDF to NeoPDF format
pixi run neopdf write convert NNPDF40_nnlo_as_01180 --output NNPDF40_nnlo_as_01180.neopdf.lz4

# Perform xfxQ2 interpolation
neopdf compute xfx_q2 --pdf-name NNPDF40_nnlo_as_01180.neopdf.lz4 --member 0 --pid 21 1e-3 10.0
```

### Fortran API (`neopdf_fapi/`)

The Fortran API provides bindings for Fortran applications. Fore some examples on how to
use the Fortran API, see the [tutorials](examples/fortran.md).

**Building the Fortran module:**

```bash
# Compile Fortran module
pixi run gfortran -c neopdf_fapi/neopdf.f90

# Test Fortran API
pixi run make -C neopdf_fapi check-fapi
```

---

## Development Workflow

### Environment Management

**Activating the environment:**

```bash
# Activate the full development environment
pixi shell

# Activate with specific features
pixi shell --feature pyapi
pixi shell --feature capi
```

**Adding new dependencies:**

```bash
# Add Python dependency
pixi add numpy

# Add Rust dependency
pixi add cargo-edit

# Add system dependency
pixi add cmake
```

### Building and Testing

**Complete build and test workflow:**

```bash
# 1. Activate environment
pixi shell

# 2. Build all components
pixi run install-pyapi
pixi run install-capi
pixi run install-cli

# 3. Run all tests
pixi run test-rust
pixi run test-pyapi
pixi run test-capi

# 4. Run benchmarks
pixi run cargo bench --manifest-path neopdf/Cargo.toml
```

**Continuous Integration tasks:**

```bash
# Run all tests in CI mode
pixi run cargo test --no-fail-fast --release

# Run with specific target
pixi run cargo test --target x86_64-unknown-linux-gnu
```

### Documentation

**Building documentation:**

```bash
# Serve documentation locally
pixi run docs

# Build static documentation
pixi run mkdocs build
```

**Generating API documentation:**

```bash
# Generate Rust documentation
pixi run cargo doc --manifest-path neopdf/Cargo.toml --open

# Generate Python documentation
pixi run maturin build --manifest-path neopdf_pyapi/Cargo.toml --documentation
```

---

## Advanced Development

### Custom Tasks

**Adding custom tasks to `pixi.toml`:**

```toml
[tasks]
# Custom development task
dev-setup = "cargo build && maturin develop && cargo cinstall"

# Custom testing task
test-all = "cargo test && pytest && make -C neopdf_capi/tests"
```

**Running custom tasks:**

```bash
pixi run dev-setup
pixi run test-all
```

### Platform-Specific Development

**Linux development:**

```bash
# Linux-specific tasks
pixi run test-rust  # Uses Linux-specific configuration
```

**Cross-platform development:**

```bash
# Build for multiple platforms
pixi run cargo build --target x86_64-unknown-linux-gnu
pixi run cargo build --target x86_64-apple-darwin
```

### Performance Profiling

**Profiling Rust code:**

```bash
# Install profiling tools
pixi add cargo-instruments  # macOS
pixi add cargo-flamegraph   # Linux

# Run with profiling
pixi run cargo flamegraph --manifest-path neopdf/Cargo.toml
```

**Profiling Python code:**

```bash
# Install Python profiling tools
pixi add pytest-profiling

# Run with profiling
pixi run pytest neopdf_pyapi/tests --profile
```

---

## Troubleshooting

### Common Issues

**Environment activation problems:**

```bash
# Reset environment
pixi clean

# Reinstall dependencies
pixi install
```

**Build failures:**

```bash
# Clean all builds
pixi run cargo clean --manifest-path neopdf/Cargo.toml
pixi run cargo clean --manifest-path neopdf_pyapi/Cargo.toml
pixi run cargo clean --manifest-path neopdf_capi/Cargo.toml

# Rebuild from scratch
pixi run install-pyapi
pixi run install-capi
```

**Test failures:**

```bash
# Run tests with verbose output
pixi run cargo test --manifest-path neopdf/Cargo.toml -- --nocapture

# Run specific failing test
pixi run cargo test --manifest-path neopdf/Cargo.toml test_name -- --nocapture
```

### Debugging

**Rust debugging:**

```bash
# Run with debug symbols
pixi run cargo build --manifest-path neopdf/Cargo.toml --debug

# Run with logging
pixi run RUST_LOG=debug cargo test --manifest-path neopdf/Cargo.toml
```

**Python debugging:**

```bash
# Run with Python debugger
pixi run python -m pdb -m pytest neopdf_pyapi/tests/test_pdfs.py

# Run with verbose output
pixi run pytest neopdf_pyapi/tests -v -s
```

---

## Deployment

### Building Release Artifacts

**Rust crates:**

```bash
# Build for crates.io
pixi run cargo build --release --manifest-path neopdf/Cargo.toml
pixi run cargo package --manifest-path neopdf/Cargo.toml
```

**Python wheels:**

```bash
# Build Python wheels
pixi run maturin build --manifest-path neopdf_pyapi/Cargo.toml --release

# Build for specific platforms
pixi run maturin build --manifest-path neopdf_pyapi/Cargo.toml --release --target x86_64-unknown-linux-gnu
```

**C-API libraries:**

```bash
# Build C-API for distribution
pixi run cargo cinstall --manifest-path neopdf_capi/Cargo.toml --release --prefix=/usr/local
```

### Publishing

**Publishing to crates.io:**

```bash
# Publish Rust crates
pixi run cargo publish --manifest-path neopdf/Cargo.toml
```

**Publishing to PyPI:**

```bash
# Publish Python package
pixi run maturin upload --manifest-path neopdf_pyapi/Cargo.toml target/wheels/*
```
