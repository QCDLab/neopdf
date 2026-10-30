//! Device/GPU batch evaluation for `NeoPDF`'s collinear PDF grids.
//!
//! See [`grid_prep`] for the host-only data layout this crate builds its device
//! kernels on top of. The `cuda`-feature device/kernel layer lands in a later
//! phase; see the crate's `Cargo.toml` for why it isn't a dependency yet.

pub mod grid_prep;
