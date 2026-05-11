//! This module provides the high-level interface for working with PDF sets.
//!
//! It defines the [`PDF`] struct, which serves as the main entry point for accessing,
//! interpolating, and retrieving metadata from PDF sets. The module abstracts over different
//! PDF set formats (LHAPDF and NeoPDF) and provides convenient loader functions for both
//! single and multiple PDF members.
//!
//! # Main Features
//!
//! - Unified interface for loading and accessing PDF sets from different formats.
//! - Parallel loading of all PDF members for efficient batch operations.
//! - High-level interpolation methods for PDF values and strong coupling constant (`alpha_s`).
//! - Access to underlying grid data and metadata for advanced use cases.
//!
//! # Key Types
//!
//! - [`PDF`]: Represents a single PDF member, providing methods for interpolation and metadata access.
//! - [`PdfSet`]: Trait for abstracting over different PDF set backends.
//! - Loader functions: [`PDF::load`], [`PDF::load_pdfs`], and internal helpers for batch loading.
//!
//! See the documentation for [`PDF`] for more details on available methods and usage patterns.
use ndarray::{Array1, Array2};
use rayon::prelude::*;

use super::gridpdf::{ForcePositive, GridArray, GridPDF};
use super::metadata::MetaData;
use super::parser::{LhapdfSet, NeopdfSet};
use super::subgrid::{RangeParameters, SubGrid};

/// Trait for abstracting over different PDF set backends (e.g., LHAPDF, NeoPDF).
///
/// Provides a unified interface for accessing the number of members and retrieving individual
/// members as metadata and grid arrays.
trait PdfSet: Send + Sync {
    /// Returns the number of members in the PDF set.
    fn num_members(&self) -> usize;
    /// Retrieves the metadata and grid array for the specified member index.
    fn member(&self, idx: usize) -> (MetaData, GridArray);
}

impl PdfSet for LhapdfSet {
    fn num_members(&self) -> usize {
        self.info.num_members as usize
    }
    fn member(&self, idx: usize) -> (MetaData, GridArray) {
        self.member(idx)
    }
}

impl PdfSet for NeopdfSet {
    fn num_members(&self) -> usize {
        self.info.num_members as usize
    }
    fn member(&self, idx: usize) -> (MetaData, GridArray) {
        self.member(idx)
    }
}

/// Loads a single PDF member from a generic PDF set backend.
///
/// # Arguments
///
/// * `set` - The PDF set backend implementing [`PdfSet`].
/// * `member` - The index of the member to load.
///
/// # Returns
///
/// A [`PDF`] instance for the specified member.
fn pdfset_loader<T: PdfSet>(set: T, member: usize) -> PDF {
    let (info, knot_array) = set.member(member);
    PDF {
        grid_pdf: GridPDF::new(info, knot_array),
    }
}

/// Loads all PDF members from a generic PDF set backend in sequential.
///
/// # Arguments
///
/// * `set` - The PDF set backend implementing [`PdfSet`].
///
/// # Returns
///
/// A vector of [`PDF`] instances, one for each member in the set.
fn pdfsets_seq_loader<T: PdfSet + Send + Sync>(set: T) -> Vec<PDF> {
    (0..set.num_members())
        .map(|idx| {
            let (info, knot_array) = set.member(idx);
            PDF {
                grid_pdf: GridPDF::new(info, knot_array),
            }
        })
        .collect()
}

/// Loads all PDF members from a generic PDF set backend in parallel.
///
/// # Arguments
///
/// * `set` - The PDF set backend implementing [`PdfSet`].
///
/// # Returns
///
/// A vector of [`PDF`] instances, one for each member in the set.
fn pdfsets_par_loader<T: PdfSet + Send + Sync>(set: T) -> Vec<PDF> {
    (0..set.num_members())
        .into_par_iter()
        .map(|idx| {
            let (info, knot_array) = set.member(idx);
            PDF {
                grid_pdf: GridPDF::new(info, knot_array),
            }
        })
        .collect()
}

/// Represents a Parton Distribution Function (PDF) set.
///
/// This struct provides a high-level interface for accessing PDF data,
/// including interpolation and metadata retrieval. It encapsulates the
/// `GridPDF` struct, which handles the low-level grid operations.
pub struct PDF {
    grid_pdf: GridPDF,
}

impl PDF {
    /// Loads a given member of the PDF set.
    ///
    /// This function reads the `.info` file and the corresponding `.dat` member file
    /// to construct a `GridPDF` object, which is then wrapped in a `PDF` instance.
    ///
    /// # Arguments
    ///
    /// * `pdf_name` - The name of the PDF set (e.g., "NNPDF40_nnlo_as_01180").
    /// * `member` - The ID of the PDF member to load (0-indexed).
    ///
    /// # Returns
    ///
    /// A `PDF` instance representing the loaded PDF member.
    pub fn load(pdf_name: &str, member: usize) -> Self {
        if pdf_name.ends_with(".neopdf.lz4") {
            pdfset_loader(NeopdfSet::new(pdf_name), member)
        } else {
            pdfset_loader(LhapdfSet::new(pdf_name), member)
        }
    }

    /// Loads all members of a PDF set in parallel.
    ///
    /// This function reads the `.info` file and all `.dat` member files
    /// to construct a `Vec<PDF>`, with each `PDF` instance representing a member
    /// of the set. The loading is performed in parallel.
    ///
    /// # Arguments
    ///
    /// * `pdf_name` - The name of the PDF set.
    ///
    /// # Returns
    ///
    /// A `Vec<PDF>` where each element is a `PDF` instance for a member of the set.
    pub fn load_pdfs(pdf_name: &str) -> Vec<PDF> {
        if pdf_name.ends_with(".neopdf.lz4") {
            pdfsets_par_loader(NeopdfSet::new(pdf_name))
        } else {
            pdfsets_par_loader(LhapdfSet::new(pdf_name))
        }
    }

    /// Loads all members of a PDF set in sequential.
    ///
    /// This function reads the `.info` file and all `.dat` member files
    /// to construct a `Vec<PDF>`, with each `PDF` instance representing a member
    /// of the set. The loading is performed in parallel.
    ///
    /// # Arguments
    ///
    /// * `pdf_name` - The name of the PDF set.
    ///
    /// # Returns
    ///
    /// A `Vec<PDF>` where each element is a `PDF` instance for a member of the set.
    pub fn load_pdfs_seq(pdf_name: &str) -> Vec<PDF> {
        if pdf_name.ends_with(".neopdf.lz4") {
            pdfsets_seq_loader(NeopdfSet::new(pdf_name))
        } else {
            pdfsets_seq_loader(LhapdfSet::new(pdf_name))
        }
    }

    /// Creates an iterator that loads PDF members lazily.
    ///
    /// This function is suitable for `.neopdf.lz4` files, which support lazy loading.
    /// It returns an iterator that yields `PDF` instances on demand, which is useful
    /// for reducing memory consumption when working with large PDF sets.
    ///
    /// # Arguments
    ///
    /// * `pdf_name` - The name of the PDF set (must end with `.neopdf.lz4`).
    ///
    /// # Returns
    ///
    /// An iterator over `Result<PDF, Box<dyn std::error::Error>>`.
    pub fn load_pdfs_lazy(
        pdf_name: &str,
    ) -> impl Iterator<Item = Result<PDF, Box<dyn std::error::Error>>> {
        assert!(
            pdf_name.ends_with(".neopdf.lz4"),
            "Lazy loading is only supported for .neopdf.lz4 files"
        );

        let iter_lazy = NeopdfSet::new(pdf_name).into_lazy_iterators();

        iter_lazy.map(|grid_array_with_metadata_result| {
            grid_array_with_metadata_result.map(|grid_array_with_metadata| {
                let info = (*grid_array_with_metadata.metadata).clone();
                let knot_array = grid_array_with_metadata.grid;
                PDF {
                    grid_pdf: GridPDF::new(info, knot_array),
                }
            })
        })
    }

    /// Clip the negative values for the `PDF` object.
    ///
    /// # Arguments
    ///
    /// * `option` - The method used to clip negative values.
    pub fn set_force_positive(&mut self, option: ForcePositive) {
        self.grid_pdf.set_force_positive(option);
    }

    /// Clip the negative values for all the `PDF` objects.
    ///
    /// # Arguments
    ///
    /// * `pdfs` - A `Vec<PDF>` where each element is a `PDF` instance.
    /// * `option` - The method used to clip negative values.
    pub fn set_force_positive_members(pdfs: &mut [PDF], option: ForcePositive) {
        for pdf in pdfs {
            pdf.set_force_positive(option.clone());
        }
    }

    /// Returns the clipping method used for a single `PDF` object.
    ///
    /// # Returns
    ///
    /// The clipping method given as a `ForcePositive` object.
    pub fn is_force_positive(&self) -> &ForcePositive {
        self.grid_pdf
            .force_positive
            .as_ref()
            .unwrap_or(&ForcePositive::NoClipping)
    }

    /// Interpolates the PDF value (xf) for a given nucleon, alphas, flavor, x, and Q2.
    ///
    /// Abstraction to the `GridPDF::xfxq2` method.
    ///
    /// # Arguments
    ///
    /// * `id` - The flavor ID (PDG ID).
    /// * `points` - A slice containing the collection of points to interpolate on.
    ///
    /// # Returns
    ///
    /// The interpolated PDF value `xf(nuclone, alphas, flavor, x, Q^2)`.
    pub fn xfxq2(&self, pid: i32, points: &[f64]) -> f64 {
        self.grid_pdf.xfxq2(pid, points).unwrap()
    }

    /// Interpolates the PDF value (xf) for multiple nucleons, alphas, flavors, xs, and Q2s.
    ///
    /// Abstraction to the `GridPDF::xfxq2s` method.
    ///
    /// # Arguments
    ///
    /// * `ids` - A vector of flavor IDs.
    /// * `slice_points` - A slice containing the collection of knots to interpolate on.
    ///   A knot is a collection of points containing `(nucleon, alphas, x, Q2)`.
    ///
    /// # Returns
    ///
    /// A 2D array of interpolated PDF values with shape `[flavors, N_knots]`.
    pub fn xfxq2s(&self, pids: Vec<i32>, slice_points: &[&[f64]]) -> Array2<f64> {
        self.grid_pdf.xfxq2s(pids, slice_points)
    }

    /// Interpolates the PDF value (xf) for multiple points using Chebyshev batch interpolation.
    ///
    /// Abstraction to the `GridPDF::xfxq2_cheby_batch` method.
    ///
    /// # Arguments
    ///
    /// * `pid` - The flavor ID.
    /// * `points` - A slice containing the collection of knots to interpolate on.
    ///   A knot is a collection of points containing `(nucleon, alphas, x, Q2)`.
    ///
    /// # Returns
    ///
    /// A `Vec<f64>` of interpolated PDF values.
    pub fn xfxq2_cheby_batch(&self, pid: i32, points: &[&[f64]]) -> Vec<f64> {
        self.grid_pdf.xfxq2_cheby_batch(pid, points).unwrap()
    }

    /// Interpolates the strong coupling constant `alpha_s` for a given Q2.
    ///
    /// Abstraction to the `GridPDF::alphas_q2` method.
    ///
    /// # Arguments
    ///
    /// * `q2` - The squared energy scale.
    ///
    /// # Returns
    ///
    /// The interpolated `alpha_s` value.
    pub fn alphas_q2(&self, q2: f64) -> f64 {
        self.grid_pdf.alphas_q2(q2)
    }

    /// Returns a reference to the PDF metadata.
    ///
    /// Abstraction to the `GridPDF::info` method.
    ///
    /// # Returns
    ///
    /// A `MetaData` struct containing information about the PDF set.
    pub fn metadata(&self) -> &MetaData {
        self.grid_pdf.metadata()
    }

    /// Returns the number of subgrids in the PDF set.
    ///
    /// # Returns
    ///
    /// The number of subgrids.
    pub fn num_subgrids(&self) -> usize {
        self.grid_pdf.knot_array.subgrids.len()
    }

    /// Returns a reference to the subgrid at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the subgrid.
    ///
    /// # Returns
    ///
    /// A reference to the `SubGrid`.
    pub fn subgrid(&self, index: usize) -> &SubGrid {
        &self.grid_pdf.knot_array.subgrids[index]
    }

    /// Returns references to all the subgrid at the given index.
    ///
    /// # Returns
    ///
    /// A reference to all the `SubGrid`.
    pub fn subgrids(&self) -> &Vec<SubGrid> {
        &self.grid_pdf.knot_array.subgrids
    }

    /// Returns the flavor PIDS of the PDG Grid.
    ///
    /// # Returns
    ///
    /// PID representation of the PDF.
    pub fn pids(&self) -> &Array1<i32> {
        &self.grid_pdf.knot_array.pids
    }

    /// Retrieves the ranges for the parameters.
    ///
    /// Abstraction to the `GridPDF::param_ranges` method.
    ///
    /// # Returns
    ///
    /// The minimum and maximum values for the parameters (x, q2, ...).
    pub fn param_ranges(&self) -> RangeParameters {
        self.grid_pdf.param_ranges()
    }

    /// Retrieves the PDF value (xf) at a specific knot point in the grid.
    ///
    /// Abstraction to the `GridArray::xf_from_index` method. This method does not
    /// perform any interpolation.
    ///
    /// # Arguments
    ///
    /// * `i_nucleons` - The index of the nucleon.
    /// * `i_alphas` - The index of the alpha_s value.
    /// * `i_kt` - The index of the `kT` value.
    /// * `ix` - The index of the x-value.
    /// * `iq2` - The index of the Q2-value.
    /// * `id` - The flavor ID.
    /// * `subgrid_id` - The ID of the subgrid.
    ///
    /// # Returns
    ///
    /// The PDF value at the specified knot.
    #[allow(clippy::too_many_arguments)]
    pub fn xf_from_index(
        &self,
        i_nucleons: usize,
        i_alphas: usize,
        i_kt: usize,
        ix: usize,
        iq2: usize,
        id: i32,
        subgrid_id: usize,
    ) -> f64 {
        self.grid_pdf
            .knot_array
            .xf_from_index(i_nucleons, i_alphas, i_kt, ix, iq2, id, subgrid_id)
    }
}
