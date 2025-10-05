# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added an additional `alpha_s` grid extraction (https://github.com/Radonirinaunimi/neopdf/pull/77).
- Added a logic to compute Chebyshev interpolations in batches (https://github.com/Radonirinaunimi/neopdf/pull/64).
- Added proper LHAPDF drop-in compatibility layer for no-code migration.
- Added an interface to the Wolfram Language to allow Rust APIs to be called in
  Mathematica.
- Added the logic to determine the Euclidean distance of a point to the closest
  subgrid in order to allow extrapolation.
- Added version-aware serialization of the `MetaData` struct to ensure backward
  and forward compatibility in writing and reading grids.
- Added a new module `alphas.rs` to store the logics of computing the strong
  coupling `alpha_s`. It contains a new struct `AlphaSAnalytic` to compute the
  `alpha_s` values analytically instead of interpolating.
- Added Chebyshev interpolation strategy for 1D, 2D, and 3D data.
- Added `pdf:mkpdfs_lazy` that loads the PDF members lazily and propagated the
  methods into the Python, C/C++, and Fortran APIs.
- Added `gridpdf::ForcePositive` enum to set the clipping method to negative
  interpolated values.
- Python API: Added `pdf:LoaderMehod` to select the method to load all the PDF
  members.

### Fixed

- Fixed how the subgrid ranges are determined for `A` and `alpha_s` when combining
  multiple sets.

### Changed

- Return zero if PID is not in the Grid (https://github.com/QCDLab/neopdf/pull/78).
- Changed the passing of the name of the PDF as a positional argument in the
  CLI (https://github.com/QCDLab/neopdf/pull/78).
- Move the computation of the logarithmic transformation out of the interpolation.
- Modified `GridArray::find_subgrid` to accept more combinations of variables
  so that the construction of subgrids is generic.
- Modified `GridArray::pid_index` to accept both `0` and `21` for the Gluon.
- Modified the NeoPDF format with the inclusion of `alphas_type` and
  `number_flavors` in the `MetaData` struct. This breaks the lazy loader using
  the `LazyGridArrayIterator` struct.

## [0.1.1] - 30/07/2025

### Added

- Initial implementation of the `neopdf` crate for collinear and transverse
  momentum dependent Parton Distribution Functions (PDFs) interpolation. This
  includes various features such as: interpolation logic for both collinear
  and TMD PDFs with support for interpolation of the nucleon numbers `A` and
  the strong coupling; reading and writing PDF grid files in the NeoPDF format.
- Python bindings via the `neopdf_pyapi` crate.
- C API interface via the `neopdf_capi` crate for C/C++ interoperability.
- Fortran interface via the `neopdf_fapi` crate for Fortran integration.
- Command line interface via the `neopdf_cli` crate for PDF manipulation
  and inspection from the terminal.
- Comprehensive documentation and usage examples for all interfaces.
