//! The C-language interface to `PDF::with_arithmetic`: the collinear `xfxq2` and `alphas_q2` in
//! an arithmetic that the caller provides as a table of callbacks.

use std::cell::Cell;
use std::cmp::Ordering;
use std::mem;
use std::ops::Deref;
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::sync::Arc;

use neopdf::precision::{Arithmetic, InterpolationError};
use neopdf::precision_pdf::{PrecisionError, PrecisionPDF};

use super::{NeoPDFWrapper, NeopdfResult};

/// The outcome of comparing two scalars.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeopdfOrdering {
    /// The first scalar is smaller.
    Less = -1,
    /// The scalars are equal.
    Equal = 0,
    /// The first scalar is larger.
    Greater = 1,
    /// The scalars cannot be ordered (e.g. one of them is a NaN).
    Unordered = 2,
}

/// A scalar of the caller's arithmetic, created by `lift`, `clone` or an operation and owned by
/// whoever receives it until it is passed to `free`.
type Scalar = *mut c_void;

/// An arithmetic given by callbacks on opaque scalars (`void *`).
///
/// Every callback receives `context` as its first argument and returns `0` on success. Any other
/// value reports a failure, which aborts the evaluation with `NEOPDF_RESULT_ERROR_ARITHMETIC`.
/// The callbacks that create a scalar (`lift`, `clone`, `add`, `sub`, `mul`, `div` and `ln`)
/// store a new scalar in `out`; its ownership passes to `NeoPDF`, which releases it with `free`.
///
/// `lift` must convert the binary64 value exactly, not through a decimal string, and every
/// operation must round to the arithmetic's own precision. The table is copied by
/// `neopdf_pdf_with_arithmetic`; `context` must stay valid while the view exists. A view may be
/// used from several threads at once only if the callbacks are thread-safe.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NeoPDFArithmetic {
    /// Caller data passed to every callback, e.g. the precision; may be `NULL`.
    pub context: *mut c_void,
    /// Converts a binary64 value exactly into a new scalar.
    pub lift: Option<unsafe extern "C" fn(*mut c_void, f64, *mut *mut c_void) -> c_int>,
    /// Copies a scalar into a new scalar.
    pub clone: Option<unsafe extern "C" fn(*mut c_void, *const c_void, *mut *mut c_void) -> c_int>,
    /// Releases a scalar.
    pub free: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    /// Stores `a + b` in a new scalar.
    pub add: Option<
        unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void, *mut *mut c_void) -> c_int,
    >,
    /// Stores `a - b` in a new scalar.
    pub sub: Option<
        unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void, *mut *mut c_void) -> c_int,
    >,
    /// Stores `a * b` in a new scalar.
    pub mul: Option<
        unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void, *mut *mut c_void) -> c_int,
    >,
    /// Stores `a / b` in a new scalar.
    pub div: Option<
        unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void, *mut *mut c_void) -> c_int,
    >,
    /// Stores the natural logarithm of `a` in a new scalar.
    pub ln: Option<unsafe extern "C" fn(*mut c_void, *const c_void, *mut *mut c_void) -> c_int>,
    /// Stores the ordering of `a` relative to `b`.
    pub compare: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const c_void,
            *const c_void,
            *mut NeopdfOrdering,
        ) -> c_int,
    >,
    /// Stores whether `a` is finite.
    pub is_finite: Option<unsafe extern "C" fn(*mut c_void, *const c_void, *mut bool) -> c_int>,
}

type Unary = unsafe extern "C" fn(*mut c_void, *const c_void, *mut Scalar) -> c_int;
type Binary = unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void, *mut Scalar) -> c_int;

/// The callbacks of a `NeoPDFArithmetic`, all present.
struct Callbacks {
    context: *mut c_void,
    lift: unsafe extern "C" fn(*mut c_void, f64, *mut Scalar) -> c_int,
    clone: Unary,
    free: unsafe extern "C" fn(*mut c_void, Scalar),
    add: Binary,
    sub: Binary,
    mul: Binary,
    div: Binary,
    ln: Unary,
    compare: unsafe extern "C" fn(
        *mut c_void,
        *const c_void,
        *const c_void,
        *mut NeopdfOrdering,
    ) -> c_int,
    is_finite: unsafe extern "C" fn(*mut c_void, *const c_void, *mut bool) -> c_int,
}

// The scalars of a view hold `Arc<Callbacks>`, so that one view can be used from several threads
// at once, which is sound on the caller's side when the callbacks are thread-safe.
unsafe impl Send for Callbacks {}
unsafe impl Sync for Callbacks {}

impl Callbacks {
    fn new(table: &NeoPDFArithmetic) -> Option<Self> {
        Some(Self {
            context: table.context,
            lift: table.lift?,
            clone: table.clone?,
            free: table.free?,
            add: table.add?,
            sub: table.sub?,
            mul: table.mul?,
            div: table.div?,
            ln: table.ln?,
            compare: table.compare?,
            is_finite: table.is_finite?,
        })
    }
}

thread_local! {
    /// Whether a callback failed during the current call into `NeoPDF` on this thread.
    static FAILED: Cell<bool> = const { Cell::new(false) };
}

/// Runs a callback that creates a scalar; a failure gives the `NULL` scalar, which every later
/// operation propagates without calling back.
fn create(callback: impl FnOnce(*mut Scalar) -> c_int) -> Scalar {
    let mut out = ptr::null_mut();
    if callback(&raw mut out) != 0 || out.is_null() {
        FAILED.with(|failed| failed.set(true));
        return ptr::null_mut();
    }
    out
}

/// A scalar of the caller's arithmetic, released with the arithmetic's `free` when dropped.
pub struct CScalar {
    ptr: Scalar,
    callbacks: Arc<Callbacks>,
}

impl CScalar {
    /// A scalar that the caller keeps owning: it is not released when dropped.
    fn borrowed(ptr: *const c_void, arithmetic: &CArithmetic) -> Borrowed {
        Borrowed(arithmetic.scalar(ptr.cast_mut()))
    }

    /// Hands the scalar over to the caller.
    fn into_raw(mut self) -> *mut c_void {
        mem::replace(&mut self.ptr, ptr::null_mut())
    }
}

/// A scalar of the caller, which `NeoPDF` only reads.
struct Borrowed(CScalar);

impl Drop for Borrowed {
    fn drop(&mut self) {
        // The caller keeps the scalar; dropping the `CScalar` then only releases its callbacks.
        self.0.ptr = ptr::null_mut();
    }
}

impl Deref for Borrowed {
    type Target = CScalar;

    fn deref(&self) -> &CScalar {
        &self.0
    }
}

impl Clone for CScalar {
    fn clone(&self) -> Self {
        let c = &self.callbacks;
        let ptr = if self.ptr.is_null() {
            ptr::null_mut()
        } else {
            create(|out| unsafe { (c.clone)(c.context, self.ptr, out) })
        };
        Self {
            ptr,
            callbacks: Arc::clone(c),
        }
    }
}

impl Drop for CScalar {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { (self.callbacks.free)(self.callbacks.context, self.ptr) };
        }
    }
}

/// The `Arithmetic` of a `NeoPDFArithmetic`.
#[derive(Clone)]
struct CArithmetic(Arc<Callbacks>);

impl CArithmetic {
    fn scalar(&self, ptr: Scalar) -> CScalar {
        CScalar {
            ptr,
            callbacks: Arc::clone(&self.0),
        }
    }

    fn unary(&self, op: Unary, a: &CScalar) -> CScalar {
        let c = &self.0;
        if a.ptr.is_null() {
            return self.scalar(ptr::null_mut());
        }
        self.scalar(create(|out| unsafe { op(c.context, a.ptr, out) }))
    }

    fn binary(&self, op: Binary, a: &CScalar, b: &CScalar) -> CScalar {
        let c = &self.0;
        if a.ptr.is_null() || b.ptr.is_null() {
            return self.scalar(ptr::null_mut());
        }
        self.scalar(create(|out| unsafe { op(c.context, a.ptr, b.ptr, out) }))
    }
}

impl Arithmetic for CArithmetic {
    type Scalar = CScalar;

    fn lift(&self, value: f64) -> CScalar {
        let c = &self.0;
        self.scalar(create(|out| unsafe { (c.lift)(c.context, value, out) }))
    }

    fn add(&self, a: &CScalar, b: &CScalar) -> CScalar {
        self.binary(self.0.add, a, b)
    }

    fn sub(&self, a: &CScalar, b: &CScalar) -> CScalar {
        self.binary(self.0.sub, a, b)
    }

    fn mul(&self, a: &CScalar, b: &CScalar) -> CScalar {
        self.binary(self.0.mul, a, b)
    }

    fn div(&self, a: &CScalar, b: &CScalar) -> CScalar {
        self.binary(self.0.div, a, b)
    }

    fn ln(&self, a: &CScalar) -> CScalar {
        self.unary(self.0.ln, a)
    }

    fn compare(&self, a: &CScalar, b: &CScalar) -> Option<Ordering> {
        let c = &self.0;
        if a.ptr.is_null() || b.ptr.is_null() {
            return None;
        }
        let mut ordering = NeopdfOrdering::Unordered;
        if unsafe { (c.compare)(c.context, a.ptr, b.ptr, &raw mut ordering) } != 0 {
            FAILED.with(|failed| failed.set(true));
            return None;
        }
        match ordering {
            NeopdfOrdering::Less => Some(Ordering::Less),
            NeopdfOrdering::Equal => Some(Ordering::Equal),
            NeopdfOrdering::Greater => Some(Ordering::Greater),
            NeopdfOrdering::Unordered => None,
        }
    }

    fn is_finite(&self, a: &CScalar) -> bool {
        let c = &self.0;
        if a.ptr.is_null() {
            return false;
        }
        let mut finite = false;
        if unsafe { (c.is_finite)(c.context, a.ptr, &raw mut finite) } != 0 {
            FAILED.with(|failed| failed.set(true));
            return false;
        }
        finite
    }
}

/// Opaque pointer to a PDF member prepared in a caller-chosen arithmetic.
pub struct NeoPDFPrecisionPDF {
    view: PrecisionPDF<CArithmetic>,
    arithmetic: CArithmetic,
}

/// Runs `op` with the failure flag of this thread cleared, and reports a failed callback first,
/// before the error that the failure may have caused.
fn checked<T>(op: impl FnOnce() -> Result<T, PrecisionError>) -> Result<T, NeopdfResult> {
    FAILED.with(|failed| failed.set(false));
    let result = op();
    if FAILED.with(Cell::take) {
        return Err(NeopdfResult::ErrorArithmetic);
    }
    result.map_err(|err| match err {
        PrecisionError::Unsupported(_) => NeopdfResult::ErrorUnsupported,
        PrecisionError::Interpolation(InterpolationError::OutOfRange) => {
            NeopdfResult::ErrorOutOfRange
        }
        PrecisionError::Interpolation(InterpolationError::NonFinite) => {
            NeopdfResult::ErrorNonFinite
        }
        PrecisionError::Interpolation(InterpolationError::InsufficientPrecision) => {
            NeopdfResult::ErrorInsufficientPrecision
        }
        PrecisionError::Interpolation(_) => NeopdfResult::ErrorInvalidData,
    })
}

/// Prepares `xfxq2` and `alphas_q2` of a PDF member in the arithmetic `arithmetic`.
///
/// The view uses the same data, subgrid selection, flavour lookup and positivity policy as `pdf`
/// (the policy as it is now), but never extrapolates. It is supported for collinear `LogBicubic`
/// sets; see `PDF::with_arithmetic` in the Rust API for the details. The view does not refer to
/// `pdf` afterwards, so `pdf` may be freed first.
///
/// On success, stores the view in `view`, to be released with `neopdf_precision_pdf_free`, and
/// returns `NEOPDF_RESULT_SUCCESS`. Otherwise stores `NULL` and returns the error:
/// `NEOPDF_RESULT_ERROR_NULL_POINTER` for a missing pointer or callback,
/// `NEOPDF_RESULT_ERROR_UNSUPPORTED` for a set outside the supported class,
/// `NEOPDF_RESULT_ERROR_ARITHMETIC` if a callback failed, or another code if the grid cannot be
/// prepared in this arithmetic.
///
/// # Safety
///
/// `pdf` must be a valid pointer to a `NeoPDF` object, `arithmetic` a valid pointer to a
/// `NeoPDFArithmetic` whose callbacks follow its contract, and `view` valid for writing.
#[no_mangle]
pub unsafe extern "C" fn neopdf_pdf_with_arithmetic(
    pdf: *const NeoPDFWrapper,
    arithmetic: *const NeoPDFArithmetic,
    view: *mut *mut NeoPDFPrecisionPDF,
) -> NeopdfResult {
    if view.is_null() {
        return NeopdfResult::ErrorNullPointer;
    }
    unsafe { *view = ptr::null_mut() };
    if pdf.is_null() || arithmetic.is_null() {
        return NeopdfResult::ErrorNullPointer;
    }
    let Some(callbacks) = Callbacks::new(unsafe { &*arithmetic }) else {
        return NeopdfResult::ErrorNullPointer;
    };
    let arithmetic = CArithmetic(Arc::new(callbacks));
    let pdf = unsafe { &(*pdf).0 };
    match checked(|| pdf.with_arithmetic(arithmetic.clone())) {
        Ok(prepared) => {
            let prepared = NeoPDFPrecisionPDF {
                view: prepared,
                arithmetic,
            };
            unsafe { *view = Box::into_raw(Box::new(prepared)) };
            NeopdfResult::Success
        }
        Err(code) => code,
    }
}

/// Interpolates `x f(x, Q2)` of the flavour `id` in the view's arithmetic.
///
/// `x` and `q2` are scalars of the view's arithmetic, which the caller keeps owning. On success,
/// stores a new scalar in `value`, which the caller owns and releases with the arithmetic's
/// `free`, and returns `NEOPDF_RESULT_SUCCESS`; a flavour that the set does not contain gives
/// `lift(0.0)`. Otherwise stores `NULL` and returns the error:
/// `NEOPDF_RESULT_ERROR_OUT_OF_RANGE` outside every subgrid (there is no extrapolation),
/// `NEOPDF_RESULT_ERROR_NON_FINITE` for a non-finite input or result, or
/// `NEOPDF_RESULT_ERROR_ARITHMETIC` if a callback failed.
///
/// # Safety
///
/// `view` must be a valid pointer returned by `neopdf_pdf_with_arithmetic`, `x` and `q2` valid
/// scalars of its arithmetic, and `value` valid for writing.
#[no_mangle]
pub unsafe extern "C" fn neopdf_precision_pdf_xfxq2(
    view: *const NeoPDFPrecisionPDF,
    id: i32,
    x: *const c_void,
    q2: *const c_void,
    value: *mut *mut c_void,
) -> NeopdfResult {
    if value.is_null() {
        return NeopdfResult::ErrorNullPointer;
    }
    unsafe { *value = ptr::null_mut() };
    if view.is_null() || x.is_null() || q2.is_null() {
        return NeopdfResult::ErrorNullPointer;
    }
    let view = unsafe { &*view };
    let x = CScalar::borrowed(x, &view.arithmetic);
    let q2 = CScalar::borrowed(q2, &view.arithmetic);
    match checked(|| view.view.xfxq2(id, &x, &q2)) {
        Ok(result) => {
            unsafe { *value = result.into_raw() };
            NeopdfResult::Success
        }
        Err(code) => code,
    }
}

/// Interpolates the strong coupling `alpha_s(Q2)` from the set's table, in the view's arithmetic.
///
/// `q2` is a scalar of the view's arithmetic, which the caller keeps owning. On success, stores a
/// new scalar in `value`, which the caller owns and releases with the arithmetic's `free`, and
/// returns `NEOPDF_RESULT_SUCCESS`. Otherwise stores `NULL` and returns the error:
/// `NEOPDF_RESULT_ERROR_OUT_OF_RANGE` outside the table,
/// `NEOPDF_RESULT_ERROR_UNSUPPORTED` for a set with an analytic `alpha_s`,
/// `NEOPDF_RESULT_ERROR_NON_FINITE` for a non-finite input or result, or
/// `NEOPDF_RESULT_ERROR_ARITHMETIC` if a callback failed.
///
/// # Safety
///
/// `view` must be a valid pointer returned by `neopdf_pdf_with_arithmetic`, `q2` a valid scalar
/// of its arithmetic, and `value` valid for writing.
#[no_mangle]
pub unsafe extern "C" fn neopdf_precision_pdf_alphas_q2(
    view: *const NeoPDFPrecisionPDF,
    q2: *const c_void,
    value: *mut *mut c_void,
) -> NeopdfResult {
    if value.is_null() {
        return NeopdfResult::ErrorNullPointer;
    }
    unsafe { *value = ptr::null_mut() };
    if view.is_null() || q2.is_null() {
        return NeopdfResult::ErrorNullPointer;
    }
    let view = unsafe { &*view };
    let q2 = CScalar::borrowed(q2, &view.arithmetic);
    match checked(|| view.view.alphas_q2(&q2)) {
        Ok(result) => {
            unsafe { *value = result.into_raw() };
            NeopdfResult::Success
        }
        Err(code) => code,
    }
}

/// Frees a view, releasing all its scalars with the arithmetic's `free`.
///
/// # Safety
///
/// `view` must be `NULL` or a valid pointer returned by `neopdf_pdf_with_arithmetic`, and must not
/// be used afterwards.
#[no_mangle]
pub unsafe extern "C" fn neopdf_precision_pdf_free(view: *mut NeoPDFPrecisionPDF) {
    if !view.is_null() {
        drop(unsafe { Box::from_raw(view) });
    }
}
