//! Device/GPU batch evaluation for `NeoPDF`'s collinear PDF grids.
//!
//! - [`grid_prep`] flattens every member of a set into one device-ready layout.
//! - [`eval`] evaluates one entry of the `[events, members, flavors + 1]` batch
//!   output from that layout; it is shared by the device kernels and the host.
//! - `device` (feature `cuda`) uploads a layout once and launches the batch kernel.
//! - `foreign` (features `capi` and `python`) drives a device grid built from a set
//!   name, with raw device pointers and streams.
//! - `capi` (feature `capi`) and `pyapi` (feature `python`) expose `foreign` as C and
//!   Python interfaces.

#[cfg(feature = "capi")]
pub mod capi;
#[cfg(feature = "cuda")]
pub mod device;
pub mod eval;
#[cfg(any(feature = "capi", feature = "python"))]
pub mod foreign;
pub mod grid_prep;
#[cfg(feature = "python")]
pub mod pyapi;
