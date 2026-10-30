//! Device/GPU batch evaluation for `NeoPDF`'s collinear PDF grids.
//!
//! - [`grid_prep`] flattens every member of a set into one device-ready layout.
//! - [`eval`] evaluates one entry of the `[events, members, flavors + 1]` batch
//!   output from that layout; it is shared by the device kernels and the host.
//! - `device` (feature `cuda`) uploads a layout once and launches the batch kernel.
//! - `capi` (feature `capi`) exposes `device` as a C interface.

#[cfg(feature = "capi")]
pub mod capi;
#[cfg(feature = "cuda")]
pub mod device;
pub mod eval;
pub mod grid_prep;
