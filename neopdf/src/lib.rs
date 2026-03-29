//! # NeoPDF Library
//!
//! NeoPDF is a modern, fast, and reliable Rust library for reading, managing, and interpolating
//! non-perturbative functions, including but not limited to PDFs, TMDS, GPDs, and GTMDs.
//!
//! ## Main Features
//!
//! - **Unified PDF Set Interface:** Load, access, and interpolate PDF sets from both LHAPDF and
//!   NeoPDF formats using a consistent API.
//! - **High-Performance Interpolation:** Provides multi-dimensional interpolation (including
//!   log-bicubic, log-tricubic, Chebyshev, and more) for PDF values, supporting advanced
//!   use-cases in high-energy physics.
//! - **Flexible Metadata Handling:** Rich metadata structures for describing PDF sets, including
//!   support for an arbitrary type of hadrons.
//! - **Conversion and Compression:** Tools to convert LHAPDF and TMDlib sets to NeoPDF format and
//!   to combine multiple nuclear PDF sets into a single file with explicit A dependence.
//! - **Efficient Storage:** Compressed storage and random access to large PDF sets using LZ4 and
//!   bincode serialization.
//!
//! ## Module Overview
//!
//! - [`converter`]: Utilities for converting and combining PDF sets.
//! - [`gridpdf`]: Core grid data structures and high-level PDF grid interface.
//! - [`interleaved`]: Pre-compute the interpolation coefficients for caching.
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
//! ## Example 1: Single Point Interpolation
//!
//! Evaluate a single flavor at a single kinematic point *(x, Q²)*.
//!
//! ```rust
//! use neopdf::pdf::PDF;
//!
//! // Load a PDF member from a set (LHAPDF or NeoPDF format)
//! let pdf = PDF::load("NNPDF40_nnlo_as_01180", 0);
//! let xf = pdf.xfxq2(21, &[0.01, 100.0]);
//! println!("xf = {}", xf);
//! ```
//!
//! ## Example 2: Multiple Flavors at a Single Kinematic Point
//!
//! [`PDF::xfxq2_allpids`] evaluates a set of flavors in a single pass, reusing the
//! subgrid lookup and interpolation coefficients.
//!
//! ```rust
//! use neopdf::pdf::PDF;
//!
//! let pdf = PDF::load("NNPDF40_nnlo_as_01180", 0);
//!
//! // PDG IDs to evaluate: gluon + light quarks and their anti-quarks.
//! let pids = [21_i32, -3, -2, -1, 1, 2, 3];
//!
//! // Single kinematic point: x = 0.1, Q² = 10 000 GeV².
//! let point = [0.1_f64, 10_000.0];
//!
//! let mut out = vec![0.0_f64; pids.len()];
//! pdf.xfxq2_allpids(&pids, &point, &mut out);
//!
//! for (&pid, &xf) in pids.iter().zip(out.iter()) {
//!     println!("pid = {:3}  xf = {:14.8e}", pid, xf);
//! }
//! ```
//!
//! ## Example 3: Batch Point Interpolation
//!
//! [`PDF::xfxq2s`] evaluates multiple flavors across a grid of *(x, Q²)* points,
//! returning a 2-D array of shape `[flavors, N_knots]`. Use this when you need a
//! dense grid evaluation rather than a single kinematic point.
//!
//! ```rust
//! use ndarray::Array2;
//! use neopdf::pdf::PDF;
//!
//! let pdf_name = "NNPDF40_nnlo_as_01180";
//! let member = 0usize;
//! let pdf = PDF::load(pdf_name, member);
//!
//! // Select a subset of flavors.
//! let pids = vec![21, 1, 2, 3, 4, 5];
//!
//! // Build a list of knots (x, Q2). Each "knot" is a slice &[x, Q2].
//! let xs = vec![1e-4, 1e-3, 1e-2, 1e-1];
//! let q2s = vec![10.0, 100.0];
//!
//! let mut points: Vec<[f64; 2]> = Vec::new();
//! for &x in &xs {
//!     for &q2 in &q2s {
//!         points.push([x, q2]);
//!     }
//! }
//!
//! // Slice-of-slices view required by xfxq2s.
//! let slice_points: Vec<&[f64]> = points.iter().map(|p| &p[..]).collect();
//!
//! // Shape: [flavors, N_knots].
//! let xf_grid: Array2<f64> = pdf.xfxq2s(pids.clone(), &slice_points);
//!
//! println!("{:-^80}", " xfxQ2 grid ");
//! for (iflav, pid) in pids.iter().enumerate() {
//!     println!("Flavor pid = {}", pid);
//!     for (iknot, values) in slice_points.iter().enumerate() {
//!         let (x, q2) = (values[0], values[1]);
//!         let val = xf_grid[[iflav, iknot]];
//!         println!("  x = {:10.3e}, Q2 = {:10.3e} -> xf = {:14.8e}", x, q2, val);
//!     }
//! }
//! ```
//!
//! ## Example 4: Inspecting Metadata
//!
//! Access the set description, kinematic coverage, flavor content, and per-subgrid
//! structure through the [`MetaData`](metadata::MetaData) object returned by
//! [`PDF::metadata`](pdf::PDF::metadata).
//!
//! ```rust
//! use neopdf::pdf::PDF;
//!
//! let pdf_name = "NNPDF40_nnlo_as_01180";
//! let member = 0usize;
//! let pdf = PDF::load(pdf_name, member);
//!
//! // --- Metadata inspection ---
//! let meta = pdf.metadata();
//! println!("Set description: {}", meta.set_desc);
//! println!("Set index: {}", meta.set_index);
//! println!("Number of members: {}", meta.num_members);
//! println!("x range: [{}, {}]", meta.x_min, meta.x_max);
//! println!("Q range: [{}, {}]", meta.q_min, meta.q_max);
//! println!("Flavors (PIDs): {:?}", meta.flavors);
//! println!("Format: {}", meta.format);
//! println!("Set type: {:?}", meta.set_type);
//! println!("Interpolator type: {:?}", meta.interpolator_type);
//!
//! // --- Subgrid information ---
//! let num_subgrids = pdf.num_subgrids();
//! println!("\nNumber of subgrids: {}", num_subgrids);
//!
//! for subgrid_idx in 0..num_subgrids {
//!     let sg = pdf.subgrid(subgrid_idx);
//!     println!("Subgrid {}:", subgrid_idx);
//!     println!("  A values:   {:?}", sg.nucleons);
//!     println!("  alphas:     {:?}", sg.alphas);
//!     println!("  kT values:  {:?}", sg.kts);
//!     println!("  x knots:    len = {}", sg.xs.len());
//!     println!("  Q2 knots:   len = {}", sg.q2s.len());
//! }
//! ```
//!
//! ## Example 5: Controlling Positivity Clipping
//!
//! PDF replicas can produce negative values in sparsely-sampled regions. Use
//! [`ForcePositive`](gridpdf::ForcePositive) to clip those values either for a
//! single member or for an entire ensemble at once.
//!
//! ```rust
//! use neopdf::gridpdf::ForcePositive;
//! use neopdf::pdf::PDF;
//!
//! let pdf_name = "NNPDF40_nnlo_as_01180";
//! let mut pdf = PDF::load(pdf_name, 0);
//!
//! // Set positivity clipping for a single member.
//! pdf.set_force_positive(ForcePositive::ClipNegative);
//! println!("Current clipping mode: {:?}", pdf.is_force_positive());
//!
//! // Set clipping for all members at once.
//! let mut all_pdfs = PDF::load_pdfs(pdf_name);
//! PDF::set_force_positive_members(&mut all_pdfs, ForcePositive::ClipSmall);
//! println!(
//!     "Clipping mode for member 4: {:?}",
//!     all_pdfs[4].is_force_positive()
//! );
//! ```
//!
//! ## Example 6: Computing PDF Uncertainties
//!
//! Load all members of a replica set, evaluate the gluon PDF at a single kinematic point,
//! and compute the 1-sigma uncertainty band using the LHAPDF-compatible
//! [`uncertainty`](uncertainty::uncertainty) function.
//!
//! ```no_run
//! use neopdf::pdf::PDF;
//! use neopdf::uncertainty::{uncertainty, CL_1_SIGMA};
//!
//! let pdf_name = "NNPDF40_nnlo_as_01180";
//! let pdfs = PDF::load_pdfs(pdf_name);
//! let meta = pdfs[0].metadata();
//!
//! // Evaluate xf(g, x=0.1, Q²=10000) for every member.
//! let values: Vec<f64> = pdfs
//!     .iter()
//!     .map(|p| p.xfxq2(21, &[0.1, 10_000.0]))
//!     .collect();
//!
//! let unc = uncertainty(
//!     &values,
//!     &meta.error_type,
//!     CL_1_SIGMA, // native CL of the set (1σ for NNPDF replicas)
//!     CL_1_SIGMA, // desired output CL
//!     false,
//! )
//! .expect("uncertainty computation failed");
//!
//! println!("central  = {:.6}", unc.central);
//! println!("errminus = {:.6}", unc.errminus);
//! println!("errplus  = {:.6}", unc.errplus);
//! println!("errsymm  = {:.6}", (unc.errminus + unc.errplus) / 2.0);
//! ```
//!
//! See module-level documentation for more details and advanced usage.

pub mod alphas;
pub mod converter;
pub mod gridpdf;
pub mod interleaved;
pub mod interpolator;
pub mod manage;
pub mod metadata;
pub mod parser;
pub mod pdf;
pub mod strategy;
pub mod subgrid;
pub mod uncertainty;
pub mod utils;
pub mod writer;
