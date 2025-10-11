//! The C-language interface for `NeoPDF`

use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_int};
use std::slice;

use neopdf::gridpdf::{ForcePositive, GridArray};
use neopdf::metadata::{InterpolatorType, MetaData, SetType};
use neopdf::parser::SubgridData;
use neopdf::pdf::PDF;
use neopdf::writer::GridArrayCollection;

const DEFAULT_PIDS: [i32; 14] = [21, -6, -5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 6, 22];

/// Result codes for `NeoPDF` operations
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeopdfResult {
    /// Operation completed successfully.
    Success = 0,
    /// A null pointer was encountered where a valid pointer was expected.
    ErrorNullPointer = -1,
    /// The provided data was invalid or could not be processed.
    ErrorInvalidData = -2,
    /// A memory allocation or deallocation error occurred.
    ErrorMemoryError = -3,
    /// The provided length or size argument was invalid.
    ErrorInvalidLength = -4,
}

impl From<NeopdfResult> for c_int {
    fn from(result: NeopdfResult) -> Self {
        result as Self
    }
}

/// Opaque pointer to a PDF object.
pub struct NeoPDFWrapper(PDF);

/// Structure to hold an array of PDF pointers and its length.
#[repr(C)]
pub struct NeoPDFMembers {
    /// Pointers to the `NeoPDF` objects.
    pub pdfs: *mut *mut NeoPDFWrapper,
    /// The number of PDF members.
    pub size: usize,
}

/// Loads a given member of the PDF set.
///
/// # Panics
///
/// This function will panic if the provided C string is not valid UTF-8.
///
/// # Safety
///
/// The `pdf_name` C string must be null-terminated and valid UTF-8.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_load(
    pdf_name: *const c_char,
    member: usize,
) -> *mut NeoPDFWrapper {
    let c_str = unsafe { CStr::from_ptr(pdf_name) };
    let pdf_name = c_str.to_str().expect("Invalid UTF-8 string");
    let pdf = PDF::load(pdf_name, member);
    Box::into_raw(Box::new(NeoPDFWrapper(pdf)))
}

/// Loads all members of the PDF set.
///
/// Returns a `NeoPDFMembers` containing pointers to all PDF objects in the set.
/// The caller is responsible for freeing the memory using `neopdf_pdf_array_free`.
///
/// # Panics
///
/// This function will panic if the provided C string is not valid UTF-8.
///
/// # Safety
///
/// The `pdf_name` C string must be null-terminated and valid UTF-8.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_load_all(pdf_name: *const c_char) -> NeoPDFMembers {
    let c_str = unsafe { CStr::from_ptr(pdf_name) };
    let pdf_name = c_str.to_str().expect("Invalid UTF-8 string");

    let pdfs = PDF::load_pdfs(pdf_name);
    let length = pdfs.len();

    let mut pdf_pointers: Vec<*mut NeoPDFWrapper> = pdfs
        .into_iter()
        .map(|pdf| Box::into_raw(Box::new(NeoPDFWrapper(pdf))))
        .collect();

    let pdfs_ptr = pdf_pointers.as_mut_ptr();
    std::mem::forget(pdf_pointers); // Prevent Vec from being dropped

    NeoPDFMembers {
        pdfs: pdfs_ptr,
        size: length,
    }
}

/// Frees a PDF object.
///
/// # Panics
///
/// This function does not panic.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object previously
/// allocated by `neopdf_pdf_load`.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_free(pdf: *mut NeoPDFWrapper) {
    if pdf.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(pdf)) };
}

/// Frees the memory allocated for a `NeoPDFMembers`.
///
/// # Safety
///
/// The `array` must be a valid `NeoPDFMembers` returned by `neopdf_pdf_load_all`.
/// After calling this function, the array and all PDF objects it contains
/// become invalid and must not be used.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_array_free(pdfs: NeoPDFMembers) {
    if pdfs.pdfs.is_null() {
        return;
    }

    let pdf_pointers = unsafe { Vec::from_raw_parts(pdfs.pdfs, pdfs.size, pdfs.size) };

    for pdf_ptr in pdf_pointers {
        if !pdf_ptr.is_null() {
            unsafe { drop(Box::from_raw(pdf_ptr)) };
        }
    }
}

/// Opaque pointer to a lazy PDF iterator object.
pub struct NeoPDFLazyIterator(Box<dyn Iterator<Item = Result<PDF, Box<dyn std::error::Error>>>>);

/// Loads a PDF set for lazy iteration.
///
/// This function is only supported for `.neopdf.lz4` files.
/// Returns a pointer to a `NeoPDFLazyIterator`. The caller is responsible for
/// freeing the memory using `neopdf_lazy_iterator_free`.
///
/// # Panics
///
/// This function will panic if the provided C string is not valid UTF-8.
///
/// # Safety
///
/// The `pdf_name` C string must be null-terminated and valid UTF-8.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_load_lazy(pdf_name: *const c_char) -> *mut NeoPDFLazyIterator {
    let c_str = unsafe { CStr::from_ptr(pdf_name) };
    let pdf_name = c_str.to_str().expect("Invalid UTF-8 string");

    if !pdf_name.ends_with(".neopdf.lz4") {
        return std::ptr::null_mut();
    }

    let lazy_iter = PDF::load_pdfs_lazy(pdf_name);
    let boxed_iter: Box<dyn Iterator<Item = Result<PDF, Box<dyn std::error::Error>>>> =
        Box::new(lazy_iter);

    Box::into_raw(Box::new(NeoPDFLazyIterator(boxed_iter)))
}

/// Retrieves the next PDF member from the lazy iterator.
///
/// Returns a pointer to a `NeoPDFWrapper` for the next member, or `NULL` if the
/// iterator is exhausted or an error occurs. The caller is responsible for freeing
/// the returned `NeoPDFWrapper` with `neopdf_pdf_free`.
///
/// # Safety
///
/// The `iter` pointer must be a valid pointer to a `NeoPDFLazyIterator` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_lazy_iterator_next(
    iter: *mut NeoPDFLazyIterator,
) -> *mut NeoPDFWrapper {
    if iter.is_null() {
        return std::ptr::null_mut();
    }
    let iter_wrapper = unsafe { &mut (*iter).0 };

    match iter_wrapper.next() {
        Some(Ok(pdf)) => Box::into_raw(Box::new(NeoPDFWrapper(pdf))),
        Some(Err(_)) | None => std::ptr::null_mut(),
    }
}

/// Frees a lazy PDF iterator object.
///
/// # Safety
///
/// The `iter` pointer must be a valid pointer to a `NeoPDFLazyIterator` object
/// previously allocated by `neopdf_pdf_load_lazy`.
#[no_mangle]
pub unsafe extern "C" fn neopdf_lazy_iterator_free(iter: *mut NeoPDFLazyIterator) {
    if !iter.is_null() {
        unsafe { drop(Box::from_raw(iter)) };
    }
}

/// Retrieves the `x_min` for this PDF set.
///
/// # Panics
///
/// This function will panic if the `pdf` pointer is null.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_x_min(pdf: *mut NeoPDFWrapper) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.param_ranges().x.min
}

/// Retrieves the `x_max` for this PDF set.
///
/// # Panics
///
/// This function will panic if the `pdf` pointer is null.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_x_max(pdf: *mut NeoPDFWrapper) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.param_ranges().x.max
}

/// Retrieves the `q2_min` for this PDF set.
///
/// # Panics
///
/// This function will panic if the `pdf` pointer is null.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_q2_min(pdf: *mut NeoPDFWrapper) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.param_ranges().q2.min
}

/// Retrieves the `q2_max` for this PDF set.
///
/// # Panics
///
/// This function will panic if the `pdf` pointer is null.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_q2_max(pdf: *mut NeoPDFWrapper) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.param_ranges().q2.max
}

/// Interpolates the PDF value (xf) for a given flavor, x, and Q2.
///
/// # Panics
///
/// This function will panic if the `pdf` pointer is null.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_xfxq2(
    pdf: *mut NeoPDFWrapper,
    id: i32,
    x: f64,
    q2: f64,
) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.xfxq2(id, &[x, q2])
}

/// Interpolates the PDF value (xf) for a generic set of parameters.
///
/// # Panics
///
/// This function will panic if the `pdf` pointer is null.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_xfxq2_nd(
    pdf: *mut NeoPDFWrapper,
    id: i32,
    params: *mut f64,
    num_params: usize,
) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    let params = unsafe { slice::from_raw_parts(params, num_params) };

    pdf_obj.xfxq2(id, params)
}

/// Interpolates PDF values for multiple points in parallel using Chebyshev batch interpolation.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
/// The `points` pointer must be a valid pointer to an array of pointers to `c_double`.
/// The `lengths` pointer must be a valid pointer to an array of `usize`.
/// `num_points` must be the correct number of points.
/// The `results` pointer must be valid for writing `num_points` elements.
///
/// # Panics
///
/// TODO
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_xfxq2_cheby_batch(
    pdf: *mut NeoPDFWrapper,
    pid: i32,
    points: *const *const c_double,
    lengths: *const usize,
    num_points: usize,
    results: *mut c_double,
) {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };

    let points_slices: &[*const c_double] = unsafe { slice::from_raw_parts(points, num_points) };
    let lengths_slice: &[usize] = unsafe { slice::from_raw_parts(lengths, num_points) };

    let rust_points: Vec<&[f64]> = points_slices
        .iter()
        .zip(lengths_slice)
        .map(|(&p, &l)| unsafe { slice::from_raw_parts(p, l) })
        .collect();

    let res_vec = pdf_obj.xfxq2_cheby_batch(pid, &rust_points);

    let results_slice = unsafe { slice::from_raw_parts_mut(results, res_vec.len()) };
    results_slice.copy_from_slice(&res_vec);
}

/// Clip the interpolated values if they turned out negatives.
///
/// # Panics
///
/// This function will panic if the `pdf` pointer is null.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_set_force_positive(
    pdf: *mut NeoPDFWrapper,
    option: ForcePositive,
) {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &mut (*pdf).0 };

    pdf_obj.set_force_positive(option);
}

/// Clip the interpolated values if they turned out negatives for all members.
///
/// # Panics
///
/// This function will panic if the `pdfs` pointer is null.
///
/// # Safety
///
/// The `pdfs` pointer must be a valid pointer to a `NeoPDFMembers` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_set_force_positive_members(
    pdfs: *mut NeoPDFMembers,
    option: ForcePositive,
) {
    assert!(!pdfs.is_null());
    let members = unsafe { &mut *pdfs };
    let pdf_slice = unsafe { slice::from_raw_parts_mut(members.pdfs, members.size) };

    for pdf_ptr in pdf_slice {
        let pdf_obj = unsafe { &mut (**pdf_ptr).0 };
        pdf_obj.set_force_positive(option.clone());
    }
}

/// Returns the value of `ForcePositive` defining the PDF grid.
///
/// # Panics
///
/// This function will panic if the `pdf` pointer is null.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_is_force_positive(pdf: *mut NeoPDFWrapper) -> ForcePositive {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &mut (*pdf).0 };

    pdf_obj.is_force_positive().clone()
}

/// Computes the `alpha_s` value at a given Q2.
///
/// # Panics
///
/// This function will panic if the `pdf` pointer is null.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_alphas_q2(pdf: *mut NeoPDFWrapper, q2: f64) -> f64 {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.alphas_q2(q2)
}

/// Returns the number of PIDs.
///
/// # Panics
///
/// This function will panic if the `pdf` pointer is null.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_num_pids(pdf: *mut NeoPDFWrapper) -> usize {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.pids().len()
}

/// Returns the PID representation of the PDF Grid.
///
/// # Panics
///
/// This function will panic if the `pdf` pointer is null.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object, and the `pids` pointer
/// must be valid for writing `num_pids` elements.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_pids(pdf: *mut NeoPDFWrapper, pids: *mut i32, num_pids: usize) {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };

    let pids = unsafe { slice::from_raw_parts_mut(pids, num_pids) };
    let pid_values = pdf_obj.pids();

    pids.copy_from_slice(pid_values.as_slice().unwrap());
}

/// Parameters for subgrids in the PDF grid.
#[repr(C)]
pub enum NeopdfSubgridParams {
    /// Parameters for subgrids in the PDF grid.
    Nucleons,
    /// The strong coupling constant (`alpha_s`) parameter.
    Alphas,
    /// The transverse momentum `kT` parameter.
    Kt,
    /// The momentum fraction (x) parameter.
    Momentum,
    /// The energy scale (Q^2) parameter.
    Scale,
}

/// Returns the number of subgrids in the PDF Grid.
///
/// # Panics
///
/// This function will panic if the `pdf` pointer is null.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_num_subgrids(pdf: *mut NeoPDFWrapper) -> usize {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };
    pdf_obj.num_subgrids()
}

/// Returns the minimum and maximum value for a given parameter.
///
/// # Panics
///
/// This function will panic if the `pdf` pointer is null.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object, and the `param_range` pointer
/// must be valid for writing two `f64` values.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_param_range(
    pdf: *mut NeoPDFWrapper,
    param: NeopdfSubgridParams,
    param_range: *mut f64,
) {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };

    let param_range = unsafe { slice::from_raw_parts_mut(param_range, 2) };
    let range_params = match param {
        NeopdfSubgridParams::Nucleons => &[
            pdf_obj.param_ranges().nucleons.min,
            pdf_obj.param_ranges().nucleons.max,
        ],
        NeopdfSubgridParams::Alphas => &[
            pdf_obj.param_ranges().alphas.min,
            pdf_obj.param_ranges().alphas.max,
        ],
        NeopdfSubgridParams::Kt => &[pdf_obj.param_ranges().kt.min, pdf_obj.param_ranges().kt.max],
        NeopdfSubgridParams::Momentum => {
            &[pdf_obj.param_ranges().x.min, pdf_obj.param_ranges().x.max]
        }
        NeopdfSubgridParams::Scale => {
            &[pdf_obj.param_ranges().q2.min, pdf_obj.param_ranges().q2.max]
        }
    };

    param_range.copy_from_slice(range_params);
}

/// Returns the shape of the subgrids in the order of their index for a given parameter.
///
/// # Panics
///
/// This function will panic if the `pdf` pointer is null.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object, and the `subgrid_shape` pointer
/// must be valid for writing `num_subgrid` elements.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_subgrids_shape_for_param(
    pdf: *mut NeoPDFWrapper,
    subgrid_shape: *mut usize,
    num_subgrid: usize,
    subgrid_param: NeopdfSubgridParams,
) {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };

    let subgrid_shape = unsafe { slice::from_raw_parts_mut(subgrid_shape, num_subgrid) };
    let shape_subgrids: Vec<usize> = pdf_obj
        .subgrids()
        .iter()
        .map(|sub| match subgrid_param {
            NeopdfSubgridParams::Nucleons => sub.nucleons.len(),
            NeopdfSubgridParams::Alphas => sub.alphas.len(),
            NeopdfSubgridParams::Kt => sub.kts.len(),
            NeopdfSubgridParams::Momentum => sub.xs.len(),
            NeopdfSubgridParams::Scale => sub.q2s.len(),
        })
        .collect();

    subgrid_shape.copy_from_slice(&shape_subgrids);
}

/// Returns the grid values of a parameter for a given subgrid.
///
/// # Panics
///
/// This function will panic if the `pdf` pointer is null.
///
/// # Safety
///
/// The `pdf` pointer must be a valid pointer to a `NeoPDF` object. The `subgrid` pointer must be
/// valid for writing the number of elements specified by `subgrid_shape[subgrid_index]`.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_subgrids_for_param(
    pdf: *mut NeoPDFWrapper,
    subgrid: *mut f64,
    subgrid_param: NeopdfSubgridParams,
    num_subgrid: usize,
    subgrid_shape: *mut usize,
    subgrid_index: usize,
) {
    assert!(!pdf.is_null());
    let pdf_obj = unsafe { &(*pdf).0 };

    let subgrid_shape = unsafe { slice::from_raw_parts(subgrid_shape, num_subgrid) };
    let subgrid = unsafe { slice::from_raw_parts_mut(subgrid, subgrid_shape[subgrid_index]) };
    let subgrid_knots = match subgrid_param {
        NeopdfSubgridParams::Nucleons => &pdf_obj.subgrids()[subgrid_index].nucleons,
        NeopdfSubgridParams::Alphas => &pdf_obj.subgrids()[subgrid_index].alphas,
        NeopdfSubgridParams::Kt => &pdf_obj.subgrids()[subgrid_index].kts,
        NeopdfSubgridParams::Momentum => &pdf_obj.subgrids()[subgrid_index].xs,
        NeopdfSubgridParams::Scale => &pdf_obj.subgrids()[subgrid_index].q2s,
    };

    subgrid.copy_from_slice(subgrid_knots.as_slice().unwrap());
}

/// An opaque struct holding the data for a single grid, including its subgrids and flavors.
/// C code should not access its fields directly.
pub struct NeoPDFGrid {
    subgrids: Vec<SubgridData>,
    flavors: Vec<i32>,
}

impl NeoPDFGrid {
    /// Creates a new, empty grid
    const fn new() -> Self {
        Self {
            subgrids: Vec::new(),
            flavors: Vec::new(),
        }
    }

    /// Adds a subgrid to the grid
    #[allow(clippy::too_many_arguments)]
    unsafe fn add_subgrid(
        &mut self,
        nucleons: *const c_double,
        num_nucleons: usize,
        alphas: *const c_double,
        num_alphas: usize,
        kts: *const c_double,
        num_kts: usize,
        xs: *const c_double,
        num_xs: usize,
        q2s: *const c_double,
        num_q2s: usize,
        grid_data: *const c_double,
        grid_data_len: usize,
    ) -> NeopdfResult {
        // Check for null pointers
        if nucleons.is_null()
            || alphas.is_null()
            || kts.is_null()
            || xs.is_null()
            || q2s.is_null()
            || grid_data.is_null()
        {
            return NeopdfResult::ErrorNullPointer;
        }

        let subgrid = unsafe {
            SubgridData {
                nucleons: slice::from_raw_parts(nucleons, num_nucleons).to_vec(),
                alphas: slice::from_raw_parts(alphas, num_alphas).to_vec(),
                kts: slice::from_raw_parts(kts, num_kts).to_vec(),
                xs: slice::from_raw_parts(xs, num_xs).to_vec(),
                q2s: slice::from_raw_parts(q2s, num_q2s).to_vec(),
                grid_data: slice::from_raw_parts(grid_data, grid_data_len).to_vec(),
            }
        };
        self.subgrids.push(subgrid);

        NeopdfResult::Success
    }

    /// Sets the flavor IDs for the grid
    unsafe fn set_flavors(&mut self, flavors: *const c_int, num_flavors: usize) -> NeopdfResult {
        if flavors.is_null() {
            return NeopdfResult::ErrorNullPointer;
        }
        self.flavors = unsafe { slice::from_raw_parts(flavors, num_flavors).to_vec() };

        NeopdfResult::Success
    }
}

/// Creates a new, empty `NeoPDFGrid`.
///
/// The caller is responsible for freeing the returned grid using `neopdf_grid_free`.
#[no_mangle]
pub extern "C" fn neopdf_grid_new() -> *mut NeoPDFGrid {
    Box::into_raw(Box::new(NeoPDFGrid::new()))
}

/// Adds a subgrid to an existing `NeoPDFGrid`.
///
/// This function takes ownership of the provided data arrays and resizes them as needed.
///
/// # Safety
/// - `grid` must be a valid pointer to a `NeoPDFGrid` created by `neopdf_grid_new`.
/// - The data pointers must be valid for the specified lengths.
#[no_mangle]
pub unsafe extern "C" fn neopdf_grid_add_subgrid(
    grid: *mut NeoPDFGrid,
    nucleons: *const c_double,
    num_nucleons: usize,
    alphas: *const c_double,
    num_alphas: usize,
    kts: *const c_double,
    num_kts: usize,
    xs: *const c_double,
    num_xs: usize,
    q2s: *const c_double,
    num_q2s: usize,
    grid_data: *const c_double,
    grid_data_len: usize,
) -> NeopdfResult {
    unsafe {
        grid.as_mut()
            .map_or(NeopdfResult::ErrorNullPointer, |grid| {
                grid.add_subgrid(
                    nucleons,
                    num_nucleons,
                    alphas,
                    num_alphas,
                    kts,
                    num_kts,
                    xs,
                    num_xs,
                    q2s,
                    num_q2s,
                    grid_data,
                    grid_data_len,
                )
            })
    }
}

/// Sets the flavor IDs for a `NeoPDFGrid`.
///
/// # Safety
/// - `grid` must be a valid pointer to a `NeoPDFGrid`.
/// - `flavors` must be a valid pointer to an array of integers of size `num_flavors`.
#[no_mangle]
pub unsafe extern "C" fn neopdf_grid_set_flavors(
    grid: *mut NeoPDFGrid,
    flavors: *const c_int,
    num_flavors: usize,
) -> NeopdfResult {
    unsafe {
        grid.as_mut()
            .map_or(NeopdfResult::ErrorNullPointer, |grid| {
                grid.set_flavors(flavors, num_flavors)
            })
    }
}

/// Frees the memory allocated for a `NeoPDFGrid`.
///
/// # Safety
/// `grid` must be a valid pointer to a `NeoPDFGrid` created by `neopdf_grid_new`.
#[no_mangle]
pub unsafe extern "C" fn neopdf_grid_free(grid: *mut NeoPDFGrid) {
    if !grid.is_null() {
        unsafe { drop(Box::from_raw(grid)) };
    }
}

/// Physical Parameters of the PDF set.
#[repr(C)]
pub struct NeoPDFPhysicsParameters {
    /// The flavor scheme used for the PDF set.
    pub flavor_scheme: *const c_char,
    /// Number of QCD loops in the calculation of PDF evolution.
    pub order_qcd: u32,
    /// Number of QCD loops in the calculation of `alpha_s`.
    pub alphas_order_qcd: u32,
    /// Value of the W boson mass.
    pub m_w: f64,
    /// Value of the Z boson mass.
    pub m_z: f64,
    /// Value of the `u` quark mass.
    pub m_up: f64,
    /// Value of the `d` quark mass.
    pub m_down: f64,
    /// Value of the `s` quark mass.
    pub m_strange: f64,
    /// Value of the `c` quark mass.
    pub m_charm: f64,
    /// Value of the `b` quark mass.
    pub m_bottom: f64,
    /// Value of the `t` quark mass.
    pub m_top: f64,
    /// Method to compute strong coupling.
    pub alphas_type: *const c_char,
    /// Number of active flavors.
    pub number_flavors: u32,
}

/// Metadata for PDF grids
#[repr(C)]
pub struct NeoPDFMetaData {
    set_desc: *const c_char,
    set_index: u32,
    num_members: u32,
    x_min: c_double,
    x_max: c_double,
    q_min: c_double,
    q_max: c_double,
    flavors: *const c_int,
    num_flavors: usize,
    format: *const c_char,
    alphas_q_values: *const c_double,
    num_alphas_q: usize,
    alphas_vals: *const c_double,
    num_alphas_vals: usize,
    polarised: bool,
    set_type: SetType,
    interpolator_type: InterpolatorType,
    error_type: *const c_char,
    hadron_pid: c_int,
    phys_params: NeoPDFPhysicsParameters,
}

/// Safely converts C string to Rust string
unsafe fn cstr_to_string(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        unsafe { Some(CStr::from_ptr(ptr).to_string_lossy().into_owned()) }
    }
}

/// Safely converts C array to Rust Vec
unsafe fn carray_to_vec<T: Copy>(ptr: *const T, len: usize) -> Option<Vec<T>> {
    if ptr.is_null() {
        None
    } else {
        unsafe { Some(slice::from_raw_parts(ptr, len).to_vec()) }
    }
}

/// Processes metadata from C struct to Rust struct
fn process_metadata(meta: *const NeoPDFMetaData) -> Option<MetaData> {
    if meta.is_null() {
        return None;
    }

    let meta = unsafe { &*meta };

    let set_desc = unsafe { cstr_to_string(meta.set_desc) }?;
    let format = unsafe { cstr_to_string(meta.format) }?;
    let flavors = unsafe { carray_to_vec(meta.flavors, meta.num_flavors) }?;
    let alphas_q_values = unsafe { carray_to_vec(meta.alphas_q_values, meta.num_alphas_q) }?;
    let alphas_vals = unsafe { carray_to_vec(meta.alphas_vals, meta.num_alphas_vals) }?;
    let error_type = unsafe { cstr_to_string(meta.error_type) }?;
    let flavor_scheme = unsafe { cstr_to_string(meta.phys_params.flavor_scheme) }?;
    let alphas_type = unsafe { cstr_to_string(meta.phys_params.alphas_type) }?;

    // Create MetaData (now MetaDataV2) directly
    let metadata = MetaData {
        set_desc,
        set_index: meta.set_index,
        num_members: meta.num_members,
        x_min: meta.x_min,
        x_max: meta.x_max,
        q_min: meta.q_min,
        q_max: meta.q_max,
        flavors,
        format,
        alphas_q_values,
        alphas_vals,
        polarised: meta.polarised,
        set_type: meta.set_type.clone(),
        interpolator_type: meta.interpolator_type.clone(),
        error_type,
        hadron_pid: meta.hadron_pid,
        git_version: String::new(),  // placeholder to be overwritten
        code_version: String::new(), // placeholder to be overwritten
        flavor_scheme,
        order_qcd: meta.phys_params.order_qcd,
        alphas_order_qcd: meta.phys_params.alphas_order_qcd,
        m_w: meta.phys_params.m_w,
        m_z: meta.phys_params.m_z,
        m_up: meta.phys_params.m_up,
        m_down: meta.phys_params.m_down,
        m_strange: meta.phys_params.m_strange,
        m_charm: meta.phys_params.m_charm,
        m_bottom: meta.phys_params.m_bottom,
        m_top: meta.phys_params.m_top,
        alphas_type,
        number_flavors: meta.phys_params.number_flavors,
        // New V2 fields with defaults
        xi_min: 1.0,
        xi_max: 1.0,
        delta_min: 0.0,
        delta_max: 0.0,
    };

    Some(metadata)
}

/// Represents a dynamically-sized collection of `NeoPDFGrid` pointers.
/// This struct is exposed to C and manages memory for the array of pointers.
#[repr(C)]
pub struct NeoPDFGridArrayCollection {
    /// A raw pointer to a C-style array of `NeoPDFGrid` pointers.
    /// This array holds the pointers to the individual grids added to the collection.
    grids: *mut *mut NeoPDFGrid,
    /// The current number of `NeoPDFGrid` pointers stored in the `grids` array.
    num_grids: usize,
    /// The total allocated capacity of the `grids` array.
    /// When `num_grids` reaches `capacity`, the array is reallocated to a larger size.
    capacity: usize,
}

impl NeoPDFGridArrayCollection {
    /// Creates a new, empty `NeoPDFGridArrayCollection`.
    /// Initializes the collection with no grids and zero capacity.
    const fn new() -> Self {
        Self {
            grids: std::ptr::null_mut(),
            num_grids: 0,
            capacity: 0,
        }
    }

    /// Adds a `NeoPDFGrid` pointer to the collection.
    /// This function handles dynamic resizing of the underlying array if needed.
    ///
    /// # Arguments
    /// * `grid` - A raw pointer to the `NeoPDFGrid` to be added.
    ///
    /// # Returns
    /// `NeoPDFResult::Success` if the grid was added successfully, or an error code
    /// (`ErrorNullPointer`, `ErrorMemoryError`) if an issue occurred.
    fn add_grid(&mut self, grid: *mut NeoPDFGrid) -> NeopdfResult {
        // Ensure the provided grid pointer is not null.
        if grid.is_null() {
            return NeopdfResult::ErrorNullPointer;
        }

        if self.num_grids == self.capacity {
            let new_capacity = if self.capacity == 0 {
                4
            } else {
                self.capacity * 2
            };

            let new_ptr = if self.grids.is_null() {
                unsafe {
                    std::alloc::alloc(
                        std::alloc::Layout::array::<*mut NeoPDFGrid>(new_capacity).unwrap(),
                    )
                    // TODO: Would using `libc` better? See commit `54b3044`.
                    .cast::<()>()
                    .cast::<*mut NeoPDFGrid>()
                }
            } else {
                unsafe {
                    std::alloc::realloc(
                        self.grids.cast::<u8>(),
                        std::alloc::Layout::array::<*mut NeoPDFGrid>(self.capacity).unwrap(),
                        new_capacity * std::mem::size_of::<*mut NeoPDFGrid>(),
                    )
                    // TODO: Would using `libc` better? See commit `54b3044`.
                    .cast::<()>()
                    .cast::<*mut NeoPDFGrid>()
                }
            };

            if new_ptr.is_null() {
                return NeopdfResult::ErrorMemoryError;
            }

            self.grids = new_ptr;
            self.capacity = new_capacity;
        }

        unsafe {
            *self.grids.add(self.num_grids) = grid;
        }
        self.num_grids += 1;

        NeopdfResult::Success
    }

    /// Returns the number of grids currently in the collection.
    const fn len(&self) -> usize {
        self.num_grids
    }

    /// Retrieves a reference to the `NeoPDFGrid` at the specified index.
    ///
    /// # Arguments
    /// * `index` - The zero-based index of the grid to retrieve.
    ///
    /// # Returns
    /// An `Option<&NeoPDFGrid>`: `Some` if the index is valid, `None` otherwise.
    fn get(&self, index: usize) -> Option<&NeoPDFGrid> {
        if index >= self.num_grids {
            return None;
        }
        unsafe { (*self.grids.add(index)).as_ref() }
    }
}

impl Drop for NeoPDFGridArrayCollection {
    fn drop(&mut self) {
        if self.grids.is_null() {
            return;
        }
        let grids_slice = unsafe { slice::from_raw_parts(self.grids, self.num_grids) };
        for &grid_ptr in grids_slice {
            if !grid_ptr.is_null() {
                unsafe { drop(Box::from_raw(grid_ptr)) };
            }
        }

        unsafe {
            std::alloc::dealloc(
                self.grids.cast::<u8>(),
                std::alloc::Layout::array::<*mut NeoPDFGrid>(self.capacity).unwrap(),
            );
        }
    }
}

/// Creates a new, empty `NeoPDFGridArrayCollection`.
///
/// # Safety
/// The caller is responsible for freeing the returned collection using
/// `neopdf_gridarray_collection_free` to prevent memory leaks.
#[no_mangle]
pub extern "C" fn neopdf_gridarray_collection_new() -> *mut NeoPDFGridArrayCollection {
    Box::into_raw(Box::new(NeoPDFGridArrayCollection::new()))
}

/// Adds a `NeoPDFGrid` to a `NeoPDFGridArrayCollection`.
///
/// # Safety
/// - `collection` must be a valid, non-null pointer to a `NeoPDFGridArrayCollection`.
/// - `grid` must be a valid, non-null pointer to a `NeoPDFGrid`.
/// - The `grid` pointer is taken ownership of by the collection; it should not be freed separately
///   until the collection itself is freed or the grid is removed from the collection.
#[no_mangle]
pub unsafe extern "C" fn neopdf_gridarray_collection_add_grid(
    collection: *mut NeoPDFGridArrayCollection,
    grid: *mut NeoPDFGrid,
) -> NeopdfResult {
    unsafe {
        collection
            .as_mut()
            .map_or(NeopdfResult::ErrorNullPointer, |collection| {
                collection.add_grid(grid)
            })
    }
}

/// Frees the memory of a `NeoPDFGridArrayCollection` and all the grids it contains.
///
/// # Safety
/// - `collection` must be a valid, non-null pointer to a `NeoPDFGridArrayCollection`
///   that was previously created by `neopdf_gridarray_collection_new`.
/// - After this call, the `collection` pointer and all grids it contained become invalid
///   and should not be used.
#[no_mangle]
pub unsafe extern "C" fn neopdf_gridarray_collection_free(
    collection: *mut NeoPDFGridArrayCollection,
) {
    if !collection.is_null() {
        unsafe { drop(Box::from_raw(collection)) };
    }
}

/// Compresses a collection of `NeoPDFGrid` objects and writes them to a file.
///
/// This function iterates through the grids in the collection, converts them to `GridArray`s,
/// and then uses the `neopdf::writer::GridArrayCollection::compress` function to write them.
///
/// # Safety
/// - `collection` must be a valid, non-null pointer to a `NeoPDFGridArrayCollection`.
/// - `metadata` must be a valid, non-null pointer to a `NeoPDFMetaData` struct.
/// - `output_path` must be a valid, null-terminated C string representing the output file path.
#[no_mangle]
pub unsafe extern "C" fn neopdf_grid_compress(
    collection: *const NeoPDFGridArrayCollection,
    metadata: *const NeoPDFMetaData,
    output_path: *const c_char,
) -> NeopdfResult {
    if collection.is_null() || metadata.is_null() || output_path.is_null() {
        return NeopdfResult::ErrorNullPointer;
    }

    let collection = unsafe { &*collection };

    let Some(meta) = process_metadata(metadata) else {
        return NeopdfResult::ErrorInvalidData;
    };

    let out_path = unsafe { CStr::from_ptr(output_path).to_str() };
    let Ok(out_path) = out_path else {
        return NeopdfResult::ErrorInvalidData;
    };

    let mut grid_arrays = Vec::with_capacity(collection.len());

    for i in 0..collection.len() {
        let Some(grid) = collection.get(i) else {
            return NeopdfResult::ErrorInvalidData;
        };
        let grid_array = GridArray::new(grid.subgrids.clone(), grid.flavors.clone());
        grid_arrays.push(grid_array);
    }

    let grid_refs: Vec<&GridArray> = grid_arrays.iter().collect();

    match GridArrayCollection::compress(&grid_refs, &meta, out_path) {
        Ok(()) => NeopdfResult::Success,
        Err(_) => NeopdfResult::ErrorMemoryError,
    }
}

// LHAPDF C-API drop-in compatibility layer.
///
/// This global state stores the loaded PDF set and the currently selected
/// member index. It is not thread-safe and mirrors the behavior of legacy
/// LHAPDF interfaces.
struct LhapdfState {
    pdf_set: Option<Vec<PDF>>,
    member: usize,
}

static mut LHAPDF_STATE: LhapdfState = LhapdfState {
    pdf_set: None,
    member: 0,
};

/// Sets LHAPDF runtime parameters from a string (no-op).
///
/// This function is provided for compatibility with the LHAPDF C API and does
/// nothing in this implementation.
///
/// # Safety
///
/// The pointer `line` must be a valid, null-terminated C string if non-null.
#[no_mangle]
pub const unsafe extern "C" fn setlhaparm(_line: *const c_char) {}

/// Fortran name-mangled variant of `setlhaparm` (no-op).
///
/// This function is provided for compatibility with the LHAPDF Fortran API and
/// does nothing in this implementation.
///
/// # Safety
///
/// The pointer `line` must be valid for reads of `len` bytes if non-null.
#[no_mangle]
pub const unsafe extern "C" fn setlhaparm_(_line: *const c_char, _len: isize) {}

/// Initializes a PDF set by its name/path and loads all members.
///
/// The loaded set is stored in a global state used by the other LHAPDF-compatible
/// functions, with the current member index reset to 0.
///
/// # Panics
///
/// Panics if `name` is not valid UTF-8 or is not a valid, null-terminated C string.
///
/// # Safety
///
/// `name` must be a valid, null-terminated C string pointing to a UTF-8 path or set name.
#[no_mangle]
pub unsafe extern "C" fn initpdfsetbyname(name: *const c_char) {
    unsafe {
        let c_str = CStr::from_ptr(name);
        let pdf_name = c_str.to_str().expect("Invalid UTF-8 string");
        let pdfs = PDF::load_pdfs(pdf_name);
        LHAPDF_STATE.pdf_set = Some(pdfs);
        LHAPDF_STATE.member = 0;
    }
}

/// Fortran name-mangled variant of `initpdfsetbyname`.
///
/// Reads a fixed-length Fortran character buffer, trims trailing spaces, and loads
/// the corresponding PDF set into the global state. The current member index is
/// reset to 0.
///
/// # Panics
///
/// TODO
///
/// # Safety
///
/// `name` must be valid for reads of `len` bytes. The buffer may not be
/// null-terminated; trailing spaces are trimmed.
#[no_mangle]
#[allow(clippy::cast_sign_loss)]
pub unsafe extern "C" fn initpdfsetbyname_(name: *const c_char, len: c_int) {
    unsafe {
        let name_slice = slice::from_raw_parts(name.cast::<u8>(), len as usize);
        let pdf_name = std::str::from_utf8(name_slice).unwrap().trim_end();
        let pdfs = PDF::load_pdfs(pdf_name);
        LHAPDF_STATE.pdf_set = Some(pdfs);
        LHAPDF_STATE.member = 0;
    }
}

/// Selects the active member of the currently loaded PDF set.
///
/// # Safety
///
/// This function does not perform bounds checking here; subsequent calls that
/// use the member will return early if the index is out of range.
#[no_mangle]
#[allow(clippy::cast_sign_loss)]
pub unsafe extern "C" fn initpdf(member: c_int) {
    unsafe { LHAPDF_STATE.member = member as usize };
}

/// Fortran name-mangled variant of `initpdf`.
///
/// # Safety
///
/// `member` must be a valid pointer to an integer.
#[no_mangle]
#[allow(clippy::cast_sign_loss)]
pub unsafe extern "C" fn initpdf_(member: *const c_int) {
    unsafe { LHAPDF_STATE.member = *member as usize };
}

/// Evaluates parton distribution functions at given `(x, q)` for the active member.
///
/// # Safety
///
/// - `f` must point to writable memory for at least 13 `c_double` values.
/// - Requires that a PDF set has been initialized via `initpdfsetbyname` or its
///   Fortran variant. If no set is loaded or the member index is out of range,
///   the function returns without writing.
#[no_mangle]
pub unsafe extern "C" fn evolvepdf(x: c_double, q: c_double, f: *mut c_double) {
    unsafe {
        let state_ptr = &raw const LHAPDF_STATE;
        let pdf_set_ptr = &raw const (*state_ptr).pdf_set;

        if let Some(pdfs) = &*pdf_set_ptr {
            let member = (*state_ptr).member;
            if member >= pdfs.len() {
                return;
            }
            let pdf = &pdfs[member];
            let q2 = q * q;

            let (available_pids, _) = pdf.pids().clone().into_raw_vec_and_offset();
            let out_slice = slice::from_raw_parts_mut(f, 14);

            for (out, &pid) in out_slice.iter_mut().zip(DEFAULT_PIDS.iter()) {
                *out = if available_pids.contains(&pid) {
                    pdf.xfxq2(pid, &[x, q2])
                } else {
                    0.0
                };
            }
        }
    }
}

/// Fortran name-mangled variant of `evolvepdf`.
///
/// # Safety
///
/// - `x`, `q` must be valid pointers to `c_double` values.
/// - `f` must point to writable memory for at least 13 `c_double` values.
/// - A PDF set must have been initialized and the member index must be valid or
///   the function will return without writing.
#[no_mangle]
pub unsafe extern "C" fn evolvepdf_(x: *const c_double, q: *const c_double, f: *mut c_double) {
    unsafe {
        let pdf_set_ptr = &raw const LHAPDF_STATE.pdf_set;

        if let Some(pdfs) = &*pdf_set_ptr {
            let member = (&raw const LHAPDF_STATE.member).read();
            if member >= pdfs.len() {
                return;
            }
            let pdf = &pdfs[member];
            let q2 = (*q) * (*q);

            let (available_pids, _) = pdf.pids().clone().into_raw_vec_and_offset();
            let out_slice = slice::from_raw_parts_mut(f, 14);

            for (out, &pid) in out_slice.iter_mut().zip(DEFAULT_PIDS.iter()) {
                *out = if available_pids.contains(&pid) {
                    pdf.xfxq2(pid, &[*x, q2])
                } else {
                    0.0
                };
            }
        }
    }
}

/// Evaluates the strong coupling `alpha_s` at scale `q` for the active member.
///
/// # Safety
///
/// - Requires that a PDF set has been initialized via `initpdfsetbyname` or its
///   Fortran variant. If no set is loaded or the member index is out of range,
///   the function returns 0.0.
#[no_mangle]
pub unsafe extern "C" fn alphaspdf(q: c_double) -> c_double {
    unsafe {
        let state_ptr = &raw const LHAPDF_STATE;
        let pdf_set_ptr = &raw const (*state_ptr).pdf_set;

        if let Some(pdfs) = &*pdf_set_ptr {
            let member = (*state_ptr).member;
            if member >= pdfs.len() {
                return 0.0;
            }
            let pdf = &pdfs[member];
            let q2 = q * q;
            pdf.alphas_q2(q2)
        } else {
            0.0
        }
    }
}

/// Fortran name-mangled variant of `alphaspdf`.
///
/// # Safety
///
/// - `q` must be a valid pointer to a `c_double` value.
/// - A PDF set must have been initialized and the member index must be valid or
///   the function will return 0.0.
#[no_mangle]
pub unsafe extern "C" fn alphaspdf_(q: *const c_double) -> c_double {
    unsafe {
        let state_ptr = &raw const LHAPDF_STATE;
        let pdf_set_ptr = &raw const (*state_ptr).pdf_set;

        if let Some(pdfs) = &*pdf_set_ptr {
            let member = (*state_ptr).member;
            if member >= pdfs.len() {
                return 0.0;
            }
            let pdf = &pdfs[member];
            let q2 = (*q) * (*q);
            pdf.alphas_q2(q2)
        } else {
            0.0
        }
    }
}
