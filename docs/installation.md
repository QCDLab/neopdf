# Installation

This guide provides detailed instructions for installing `NeoPDF` and its APIs for Rust, Python,
Fortran, and C/C++, as well as the command line interface (CLI).

!!! danger "Development with Pixi"

    For developers who want to contribute to NeoPDF or work with the source code and want a proper
    environment manager, a `pixi.toml` configuration is provided to work with the [Pixi](https://pixi.sh/latest/)
    environment and package manager. See the comprehensive [Development with Pixi](./development-with-pixi.md)
    guide for detailed instructions on setting up a reproducible development environment.

!!! note "Where to store PDF sets?"

    By default, `NeoPDF` stores PDF sets in `${HOME}/.local/share/neopdf`, however this can be
    overwritten via the environment variable `NEOPDF_DATA_PATH` to point to the LHAPDF path for
    example.

    ```bash
    export NEOPDF_DATA_PATH=${LHAPDF_DATA_PATH}
    ```

---

## Rust Crate

To use `neopdf` in your Rust project, simply add the following to your `Cargo.toml`:

```toml
[dependencies]
neopdf = "0.1.0" # select the version
```

Cargo will automatically fetch and compile the dependencies the next time you build your project with:

```bash
cargo build
```

---

## Python API

`NeoPDF` is available on the Python Packaged Index (PyPI) as `neopdf-hep` and therefore can be
installed easily with any of the Python's package managers. For example, using
[pipx](https://pipx.pypa.io/stable/):

```bash
pipx install neopdf-hep
```

!!! info "Development Option"

    Alternatively, to build the Python API from source, make sure that [maturin](https://www.maturin.rs/)
    is installed, go into the `neopdf_pyapi` directory, and then simply run:

    ```bash
    maturin develop --release --extras test
    ```

    This will build and install the `NeoPDF` Python extension in your current environment.

---

## C/C++ API

The simplest way to install the C API and the C++ OOP header is to download the pre-built libraries:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Radonirinaunimi/neopdf/refs/heads/master/install-capi.sh | sh
```

To pass the installation directory for where to put the files, change the arguments of the shell as
follows:

```bash
.. | sh -s -- --prefix /custom/installation/path
```

By default, the script will download the latest stable release. If you would like a specific version,
pass the version along with `--version`:

```bash
.. | sh -s -- --version 0.2.0-alpha1
```

!!! info "Development Option"

    Alternatively, to build the C-API from source, first install `cargo-c`:

    ```bash
    cargo install cargo-c
    ```

    Then go into the `neopdf_capi` directory and run the following command:

    ```bash
    export CARGO_C_INSTALL_PREFIX=${prefix} # Required for the OOP C++ header
    cargo cinstall --release --prefix=${prefix}
    ```

    This will install the library in the `${prefix}` path. This path can then be added to the `PKG_CONFIG_PATH`
    and `LD_LIBRARY_PATH` environment variables by running:

    ```bash
    export LD_LIBRARY_PATH=${prefix}/lib:$LD_LIBRARY_PATH
    export PKG_CONFIG_PATH=${prefix}/lib/pkgconfig:$PKG_CONFIG_PATH
    ```

    Remember to source your shell configuration or restart your terminal for the changes to take effect.

---

## Fortran API

In order to use the Fortran API, you have to install first the C/C++ API. Then, simply copy the
`neopdf_fapi/neopdf.f90` in your working directory and generate the Fortran module:

```bash
gfortran -c neopdf.f90
```

If everything went fine, this will generate a `neopdf.mod` file. You can now the module in your
fortran program by including the following:

```fortran
use neopdf
```

---

## CLI Tool

The Command Line Interface (CLI) to the `NeoPDF` APIs is also available on the Python Packaged
Index (PyPI) as `neopdf-cli` and therefore can be installed easily with any of the Python's
package managers. For example, using [pipx](https://pipx.pypa.io/stable/):

```bash
pipx install neopdf-cli
```

!!! info " Development Option"

    To build and install the NeoPDF command-line interface (CLI) from source, simply run:

    ```bash
    cargo install --path neopdf_cli
    ```

    This will compile the CLI in debug mode and make the `neopdf` command available in your
    cargo bin directory (usually `~/.cargo/bin`). You can then run `neopdf --help` to see
    the available commands.
