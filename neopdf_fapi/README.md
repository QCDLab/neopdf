# Fortran-API for NeoPDF

This crate provides a Fortran-compatible API for the `neopdf` Rust library.
To use it, you first need to install the C-API by following the instructions
[here](https://radonirinaunimi.github.io/neopdf/installation/#cc-api), then
put the `neopdf.f90` file in your working directory, and finally link to it
during compilation.
