//! # NeoPDF Library
//!
//! NeoPDF is a modern, fast, and reliable Rust library for reading, managing, and interpolating
//! both collinear and transverse momentum Parton Distribution Functions ([TMD] PDFs) from both
//! the LHAPDF, TMDlib, and NeoPDF formats.
//!
//! ## Main Features
//!
//! - **Unified PDF Set Interface:** Load, access, and interpolate PDF sets from both LHAPDF and
//!   NeoPDF formats using a consistent API.
//! - **High-Performance Interpolation:** Provides multi-dimensional interpolation (including
//!   log-bicubic, log-tricubic, and more) for PDF values, supporting advanced use cases in
//!   high-energy physics.
//! - **Flexible Metadata Handling:** Rich metadata structures for describing PDF sets, including
//!   support for an arbitrary type of hadrons.
//! - **Conversion and Compression:** Tools to convert LHAPDF sets to NeoPDF format and to combine
//!   multiple nuclear PDF sets into a single file with explicit A dependence.
//! - **Efficient Storage:** Compressed storage and random access to large PDF sets using LZ4 and
//!   bincode serialization.
//!
//! ## Module Overview
//!
//! - [`converter`]: Utilities for converting and combining PDF sets.
//! - [`gridpdf`]: Core grid data structures and high-level PDF grid interface.
//! - [`interpolator`]: Dynamic interpolation traits and factories for PDF grids.
//! - [`manage`]: Management utilities for PDF set installation, download, and path resolution.
//! - [`metadata`]: Metadata structures and types for describing PDF sets.
//! - [`parser`]: Parsing utilities for reading and interpreting PDF set data files.
//! - [`pdf`]: High-level interface for working with PDF sets and interpolation.
//! - [`strategy`]: Interpolation strategy implementations (bilinear, log-bicubic, etc.).
//! - [`subgrid`]: Subgrid data structures and parameter range logic.
//! - [`utils`]: Utility functions for interpolation and grid operations.
//! - [`writer`]: Utilities for serializing, compressing, and accessing PDF grid data.
//!
//! ## Example Usage
//!
//! ```rust
//! use neopdf_legacy::pdf::PDF;
//!
//! // Load a PDF member from a set (LHAPDF or NeoPDF format)
//! let pdf = PDF::load("NNPDF40_nnlo_as_01180", 0);
//! let xf = pdf.xfxq2(21, &[0.01, 100.0]);
//! println!("xf = {}", xf);
//! ```
//!
//! See module-level documentation for more details and advanced usage.

pub mod alphas;
pub mod converter;
pub mod gridpdf;
pub mod interpolator;
pub mod manage;
pub mod metadata;
pub mod parser;
pub mod pdf;
pub mod strategy;
pub mod subgrid;
pub mod utils;
pub mod writer;
