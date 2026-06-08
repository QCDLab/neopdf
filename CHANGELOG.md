# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added further `LHAPDF` compatibility layers ([#93](https://github.com/QCDLab/neopdf/pull/93))

## [0.3.3] - 12/05/2026

### Added

- Added routines to compute uncertainties from non-perturbative functions.

### Changed

- Separate `neopdf_legacy` to be its own crate.

## [0.3.2] - 29/03/2026

## [0.3.1] - 18/03/2026

### Added

- Added `xfxQ2_allpids` and `xfxQ2_allpids_ND` methods into the Python API.

## [0.3.0] - 18/03/2026

### Added

- Major refactor to speed up significantly the interpolation on a single point and to
  remove overheads when computing multiple PIDs at once for the same kinematic points.
  The latter is addressed by pre-computing and interleaving the coefficients of the
  interpolations so that they can be efficiently re-used.
- Added an abstract `neopdf::interleave.rs` crate to interleave the coefficients for a
  given data dimension.
- Added a `neopdf::pdf::xfxq2_allpids` method to compute the interpolated function on
  a given kinematics for all the PIDs.
- Added a `neopdf::gridpdfs::xfxq2_fast` to bypass the V-table lookup dispatch, validation,
  and result unwrapping.
- Added `neopdf_pdf_xfxq2_allpids` and `neopdf_pdf_xfxq2s` to the C/C++ APIs.
- Added Chebyshev interpolation strategies for 4D and 5D data.
- Added `LogFourCubic` and `LogFiveCubic` interpolation strategies for 4D and 5D data.
- Added new methods to the Fortran and C/C++ APIs to write and compress grids
  with `xi` and `delta` dependence.
- Added `load_by_lhaid` method to load PDF set using the LHAPDF ID.
- Added `load_lhapdf_by_file` method to load a PDF member using full path.

### Changed

- Breaking change to the Python API for the `PyMetaData` and `PySubgrid`.
- Bump `PyO3` and `numpy` versions to `v0.27` to support Python `3.14`.
- Extended the Grid layout to support GTMDs and GPDs (https://github.com/QCDLab/neopdf/pull/79).

## [0.2.0] - 06/10/2025

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
