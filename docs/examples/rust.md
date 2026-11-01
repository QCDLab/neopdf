---
title: "Rust API Examples"
---

# Rust API Examples

This page provides a hands-on tutorial on how to use the **native Rust API** of `NeoPDF`
to load, inspect, and interpolate PDF sets.

## Prerequisites

In order to start using the native Rust API, first add the crate dependency in your
`Cargo.toml`:

```toml
[dependencies]
neopdf = "0.3.0"
ndarray = "0.16.1"
```

---

## Example 1: Loading and Evaluating a Single PDF Member

In this first example, we load a **single PDF member** from an LHAPDF or NeoPDF grid
and evaluate the gluon distribution \(x f(x, Q^2)\) for a range of \(x\) and \(Q^2\).

```rust linenums="1"
use neopdf::pdf::PDF;

fn main() {
    // Name of the PDF set: can be a standard LHAPDF name
    // or a NeoPDF grid such as "NNPDF40_nnlo_as_01180.neopdf.lz4".
    let pdf_name = "NNPDF40_nnlo_as_01180";
    let member = 0usize; // central replica

    // Load the PDF member.
    let pdf = PDF::load(pdf_name, member);

    // Parton ID (PDG): 21 = gluon.
    let pid = 21;

    // Example kinematics.
    let x_values = vec![5e-2, 1.5e-1, 2.5e-1, 3.5e-1, 4.5e-1];
    let q2 = 100.0;

    for x in x_values {
        let xf = pdf.xfxq2(pid, &[x, q2]);
        println!("{:10.3e} {:20.8e}", x, xf);
    }
}
```

**Key points:**

- `PDF::load` automatically detects whether the set is LHAPDF (`NNPDF40_nnlo_as_01180`)
  or NeoPDF (`NNPDF40_nnlo_as_01180.neopdf.lz4`) and uses the appropriate backend.
- `xfxq2(pid, &[x, q2])` evaluates \(x f(x, Q^2)\) at the requested kinematic point.

---

## Example 2: Evaluating Multiple Flavors at Once

When evaluating PDFs for **several flavors** at the same kinematic point, it is more
efficient to reuse the subgrid lookup and interpolation. The `xfxq2_allpids` method
does exactly this.

```rust linenums="1"
use neopdf::pdf::PDF;

fn main() {
    let pdf_name = "NNPDF40_nnlo_as_01180";
    let member = 0usize;
    let pdf = PDF::load(pdf_name, member);

    // A typical list of PDG IDs: anti-quarks, gluon, quarks.
    // TODO: Check if this is consistent with the internal definition.
    let pids = [-5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5];

    // Kinematic point (x, Q2).
    let x = 1e-3;
    let q2 = 10.0;
    let point = [x, q2];

    // Output buffer, one entry per PID.
    let mut results = vec![0.0_f64; pids.len()];
    pdf.xfxq2_allpids(&pids, &point, &mut results);

    for (pid, value) in pids.iter().zip(results.iter()) {
        println!("{:8} {:12.3e} {:12.3e} {:20.8e}", pid, x, q2, value);
    }
}
```

---

## Example 3: Batch Evaluation with `xfxq2s`

For various analyses, one often needs values at many kinematic points. The `xfxq2s`
method computes a full **2D array** of values for multiple flavors and many points in
a single call.

```rust linenums="1"
use ndarray::Array2;
use neopdf::pdf::PDF;

fn main() {
    let pdf_name = "NNPDF40_nnlo_as_01180";
    let member = 0usize;
    let pdf = PDF::load(pdf_name, member);

    // Select a subset of flavors.
    let pids = vec![21, 1, 2, 3, 4, 5];

    // Build a list of knots (x, Q2). Each "knot" is a slice &[x, Q2].
    let xs = vec![1e-4, 1e-3, 1e-2, 1e-1];
    let q2s = vec![10.0, 100.0];

    let mut points: Vec<[f64; 2]> = Vec::new();
    for &x in &xs {
        for &q2 in &q2s {
            points.push([x, q2]);
        }
    }

    // Slice-of-slices view required by xfxq2s.
    let slice_points: Vec<&[f64]> = points.iter().map(|p| &p[..]).collect();

    // Shape: [flavors, N_knots].
    let xf_grid: Array2<f64> = pdf.xfxq2s(pids.clone(), &slice_points);

    println!("{:-^80}", " xfxQ2 grid ");
    for (iflav, pid) in pids.iter().enumerate() {
        println!("Flavor pid = {}", pid);
        for (iknot, values) in slice_points.iter().enumerate() {
            let (x, q2) = (values[0], values[1]);
            let val = xf_grid[[iflav, iknot]];
            println!("  x = {:10.3e}, Q2 = {:10.3e} -> xf = {:14.8e}", x, q2, val);
        }
    }
}
```

---

## Example 4: Lazy Loading of NeoPDF Grids

For large `.neopdf.lz4` grids with many members, you may not want to load everything
into memory at once. The `load_pdfs_lazy` constructor exposes an iterator that yields
`PDF` members on demand.

```rust linenums="1"
use neopdf::pdf::PDF;

fn main() {
    let pdf_name = "NNPDF40_nnlo_as_01180.neopdf.lz4";
    let pid = 21;
    let x = 1e-3;
    let q2 = 10.0;
    let point = [x, q2];

    println!("{:-^60}", " Lazy loading of PDF members ");
    println!("{:>8} {:>12} {:>12} {:>20}", "member", "x", "Q2", "NeoPDF");
    println!("{:-^60}", "");

    for (member_idx, pdf_result) in PDF::load_pdfs_lazy(pdf_name).enumerate() {
        let pdf = match pdf_result {
            Ok(pdf) => pdf,
            Err(err) => {
                eprintln!("Failed to load member {}: {}", member_idx, err);
                continue;
            }
        };

        let xf = pdf.xfxq2(pid, &point);
        println!("{:8} {:12.3e} {:12.3e} {:20.8e}", member_idx, x, q2, xf);
    }
}
```

---

## Example 5: Inspecting Metadata and Subgrids

Beyond simple interpolation, the Rust API allows you to inspect the **metadata** and
the internal **subgrid structure**, similar to what the CLI `read` commands expose.

```rust linenums="1"
use neopdf::pdf::PDF;

fn main() {
    let pdf_name = "NNPDF40_nnlo_as_01180";
    let member = 0usize;
    let pdf = PDF::load(pdf_name, member);

    // --- Metadata inspection ---
    let meta = pdf.metadata();
    println!("Set description: {}", meta.set_desc);
    println!("Set index: {}", meta.set_index);
    println!("Number of members: {}", meta.num_members);
    println!("x range: [{}, {}]", meta.x_min, meta.x_max);
    println!("Q range: [{}, {}]", meta.q_min, meta.q_max);
    println!("Flavors (PIDs): {:?}", meta.flavors);
    println!("Format: {}", meta.format);
    println!("Set type: {:?}", meta.set_type);
    println!("Interpolator type: {:?}", meta.interpolator_type);

    // --- Subgrid information ---
    let num_subgrids = pdf.num_subgrids();
    println!("\nNumber of subgrids: {}", num_subgrids);

    for subgrid_idx in 0..num_subgrids {
        let sg = pdf.subgrid(subgrid_idx);
        println!("Subgrid {}:", subgrid_idx);
        println!("  A values:   {:?}", sg.nucleons);
        println!("  alphas:     {:?}", sg.alphas);
        println!("  kT values:  {:?}", sg.kts);
        println!("  x knots:    len = {}", sg.xs.len());
        println!("  Q2 knots:   len = {}", sg.q2s.len());
    }
}
```

This is the Rust equivalent of running commands such as:

```bash
neopdf read metadata NNPDF40_nnlo_as_01180
neopdf read num_subgrids NNPDF40_nnlo_as_01180
neopdf read subgrid-info NNPDF40_nnlo_as_01180 --member=0 --subgrid-index=0
```

---

## Example 6: Controlling Positivity Clipping

The Rust API also exposes a method for **positivity clipping** of a given grid.

```rust linenums="1"
use neopdf::gridpdf::ForcePositive;
use neopdf::pdf::PDF;

fn main() {
    let pdf_name = "NNPDF40_nnlo_as_01180";
    let mut pdf = PDF::load(pdf_name, 0);

    // Set positivity clipping for a single member.
    pdf.set_force_positive(ForcePositive::ClipNegative);
    println!("Current clipping mode: {:?}", pdf.is_force_positive());

    // Set clipping for all members at once.
    let mut all_pdfs = PDF::load_pdfs(pdf_name);
    PDF::set_force_positive_members(&mut all_pdfs, ForcePositive::ClipSmall);
    println!(
        "Clipping mode for member 4: {:?}",
        all_pdfs[4].is_force_positive()
    );
}
```

---

## Example 7: Filling and Writing a NeoPDF Grid (LHAPDF-like)

The following example illustrates how one can create a `NeoPDF` grid from scratch.
We do so by loading the values from an existing LHAPDF set and create a genuine
compressed `NeoPDF` grid with the extension `.neopdf.lz4`.

!!! tip "NOTE"

    As in the C/C++/Fortran tutorials, this example recomputes the values of the subgrids
    from an existing set to validate the writer. This makes the code verbose; the lines
    that actually fill the grid are highlighted.

```rust linenums="1"
use neopdf::gridpdf::GridArray;
use neopdf::metadata::{InterpolatorType, MetaData, SetType};
use neopdf::pdf::PDF;
use neopdf::subgrid::{GridData, ParamRange, RangeParameters, SubGrid};
use neopdf::writer::GridArrayCollection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Name of the input LHAPDF set (can also be a NeoPDF grid).
    let pdf_name = "NNPDF40_nnlo_as_01180";

    // Load all PDF members (eager mode).
    let pdfs = PDF::load_pdfs(pdf_name);
    if pdfs.is_empty() {
        eprintln!("Failed to load any PDF members!");
        std::process::exit(1);
    }
    println!("Loaded {} PDF members", pdfs.len());

    // Use the first member as a reference for metadata and knot structure.
    let ref_pdf = &pdfs[0];

    // Extract the flavor PIDs.
    let pids = ref_pdf.pids().clone();

    // Extract subgrid information from the reference member.
    let num_subgrids = ref_pdf.num_subgrids();

    // Collection of GridArray references that will be compressed.
    let mut grids: Vec<GridArray> = Vec::with_capacity(pdfs.len());

    // For each member, build a GridArray.
    for (m, pdf) in pdfs.iter().enumerate() {
        println!("Building grid for member {}", m);

        let mut subgrids: Vec<SubGrid> = Vec::with_capacity(num_subgrids);

        // Loop over subgrids and reconstruct the values on each knot.
        for subgrid_idx in 0..num_subgrids {
            let sg = pdf.subgrid(subgrid_idx);

            let nucleons = sg.nucleons.clone();
            let alphas = sg.alphas.clone();
            let kts = sg.kts.clone();
            let xs = sg.xs.clone();
            let q2s = sg.q2s.clone();

            // Compute grid_data as [nucleon][alpha][pid][kt][x][q2].
            // Here we assume a standard LHAPDF-like grid: no explicit
            // A or alpha_s dependence.
            let n_nuc = nucleons.len().max(1);
            let n_alp = alphas.len().max(1);
            let n_kt = kts.len().max(1);
            let n_x = xs.len();
            let n_q2 = q2s.len();
            let n_pid = pids.len();

            let mut data = vec![0.0_f64; n_nuc * n_alp * n_pid * n_kt * n_x * n_q2];
            let mut idx = 0;

            data.iter_mut()
                .zip(
                    (0..n_nuc)
                        .flat_map(|_| (0..n_alp))
                        .flat_map(|_| pids.iter())
                        .flat_map(|pid| (0..n_kt).map(move |_| pid))
                        .flat_map(|pid| xs.iter().map(move |&x| (pid, x)))
                        .flat_map(|(pid, x)| q2s.iter().map(move |&q2| (pid, x, q2))),
                )
                .for_each(|(slot, (pid, x, q2))| {
                    *slot = pdf.xfxq2(*pid, &[x, q2]);
                });

            let grid = GridData::Grid6D(ndarray::Array::from_shape_vec(
                (n_nuc, n_alp, n_pid, n_kt, n_x, n_q2),
                data,
            )?);

            // Build parameter ranges for this subgrid.
            let nucleons_range = ParamRange::new(
                nucleons.first().copied().unwrap_or(0.0),
                nucleons.last().copied().unwrap_or(0.0),
            );
            let alphas_range = ParamRange::new(
                alphas.first().copied().unwrap_or(0.0),
                alphas.last().copied().unwrap_or(0.0),
            );
            let kt_range = ParamRange::new(
                kts.first().copied().unwrap_or(0.0),
                kts.last().copied().unwrap_or(0.0),
            );
            let x_range =
                ParamRange::new(xs.first().copied().unwrap(), xs.last().copied().unwrap());
            let q2_range =
                ParamRange::new(q2s.first().copied().unwrap(), q2s.last().copied().unwrap());

            let subgrid = SubGrid {
                xs,
                q2s,
                kts,
                xis: ndarray::Array1::from_vec(vec![0.0]),
                deltas: ndarray::Array1::from_vec(vec![0.0]),
                grid,
                nucleons,
                alphas,
                nucleons_range,
                alphas_range,
                xi_range: ParamRange::new(0.0, 0.0),
                delta_range: ParamRange::new(0.0, 0.0),
                kt_range,
                x_range,
                q2_range,
            };

            subgrids.push(subgrid);
        }

        // Finalize GridArray for this member.
        let grid_array = GridArray::from_parts(pids.clone(), subgrids);
        grids.push(grid_array);
    }

    // Construct metadata for the collection.
    let ranges: RangeParameters = ref_pdf.param_ranges();
    let meta = MetaData {
        set_desc: "NNPDF40_nnlo_as_01180 collection (Rust writer example)".into(),
        set_index: 0,
        num_members: grids.len() as u32,
        x_min: ranges.x.min,
        x_max: ranges.x.max,
        q_min: ranges.q2.min.sqrt(),
        q_max: ranges.q2.max.sqrt(),
        flavors: pids.to_vec(),
        format: "neopdf".into(),
        alphas_q_values: vec![2.0],
        alphas_vals: vec![0.118],
        polarised: false,
        set_type: SetType::SpaceLike,
        interpolator_type: InterpolatorType::LogBicubic,
        error_type: "replicas".into(),
        hadron_pid: 2212,
        git_version: String::new(),
        code_version: String::new(),
        flavor_scheme: "variable".into(),
        order_qcd: 2,
        alphas_order_qcd: 2,
        m_w: 80.352,
        m_z: 91.1876,
        m_up: 0.0,
        m_down: 0.0,
        m_strange: 0.0,
        m_charm: 1.51,
        m_bottom: 4.92,
        m_top: 172.5,
        alphas_type: "ipol".into(),
        number_flavors: 4,
        xi_min: 0.0,
        xi_max: 0.0,
        delta_min: 0.0,
        delta_max: 0.0,
    };

    // Decide where to write the output grid.
    let filename = "check-writer-rust.neopdf.lz4";
    let output_path = match env::var("NEOPDF_DATA_PATH") {
        Ok(root) => {
            let mut path = std::path::PathBuf::from(root);
            path.push(filename);
            path
        }
        Err(_) => std::path::PathBuf::from(filename),
    };

    // Prepare references for compression.
    let grid_refs: Vec<&GridArray> = grids.iter().collect();

    // Compress and write the GridArray collection to disk.
    GridArrayCollection::compress(&grid_refs, &meta, &output_path)?;
    println!("Compression succeeded: {}", output_path.display());

    Ok(())
}
```

---

## Example 8: Interpolating in a Caller-Chosen Arithmetic

`PDF::xfxq2` and `PDF::alphas_q2` compute in `f64`. With `PDF::with_arithmetic`,
the same interpolation runs in **an arithmetic one chooses**. That can be a
multiprecision float (MPFR through `rug`, `astro-float`, ...), a double-double,
interval arithmetic, or plain `f64` again.

### Why it is needed

A PDF grid stores its values as `f64`, and `PDF::xfxq2` turns them into
\(x f(x, Q^2)\) with a sequence of `f64` operations:

- the logarithms of \(x\), \(Q^2\) and of the grid knots;
- the finite-difference slopes at the knots;
- the Hermite polynomial in the two variables.

Each operation rounds, so the result carries an error of a few units in the last
place, \(\mathcal{O}(10^{-16})\) relative and sometimes more. For a typical
prediction this error is irrelevant. It matters when the rest of the computation
is done beyond `f64`. This is relevant for example for **High-precision calculations**
(quadruple precision or with MPFR).

### How it works

`with_arithmetic` builds a `PrecisionPDF<A>`, a **view** of one PDF member in the
arithmetic `A`. When the view is created:

1. every grid value and knot is converted **exactly** from `f64` into `A` (`lift`);
2. the logarithms of the knots, the slopes and the interpolation coefficients are
   computed **in `A`**, not in `f64`.

The view then evaluates `xfxq2` and `alphas_q2` entirely in `A`. It uses the same
subgrid selection, the same flavour lookup and the same positivity clipping as the
`PDF` it came from. It owns its data, so it does not borrow the `PDF`: the `PDF`
can be changed or dropped while the view is in use.

An arithmetic is any type that implements the `neopdf::precision::Arithmetic` trait:

```rust
pub trait Arithmetic {
    /// A finite real scalar; it need not implement `Copy`.
    type Scalar: Clone;
    /// Convert fixed binary64 input data at the context's precision.
    fn lift(&self, value: f64) -> Self::Scalar;
    fn add(&self, a: &Self::Scalar, b: &Self::Scalar) -> Self::Scalar;
    fn sub(&self, a: &Self::Scalar, b: &Self::Scalar) -> Self::Scalar;
    fn mul(&self, a: &Self::Scalar, b: &Self::Scalar) -> Self::Scalar;
    fn div(&self, a: &Self::Scalar, b: &Self::Scalar) -> Self::Scalar;
    /// Natural logarithm in the context's arithmetic.
    fn ln(&self, a: &Self::Scalar) -> Self::Scalar;
    /// Compare without conversion to binary64. Unordered values return `None`.
    fn compare(&self, a: &Self::Scalar, b: &Self::Scalar) -> Option<std::cmp::Ordering>;
    /// Whether the scalar is finite.
    fn is_finite(&self, a: &Self::Scalar) -> bool;
}
```

The trait is a **context**: `self` can carry the precision, the rounding mode or any
other setting, and `Scalar` is the number type. `neopdf` itself depends on no
multiprecision crate. It ships one implementation, `neopdf::precision::Binary64`,
which is plain `f64`.

### Usage with `f64`

With `Binary64`, the view reproduces the standard path, which makes it a convenient
check:

```rust linenums="1"
use neopdf::pdf::PDF;
use neopdf::precision::Binary64;
use neopdf::precision_pdf::PrecisionError;

fn main() -> Result<(), PrecisionError> {
    let pdf = PDF::load("NNPDF40_nnlo_as_01180", 0);
    let view = pdf.with_arithmetic(Binary64)?;

    let (x, q2) = (1e-3, 1e4);
    println!("PDF::xfxq2           {:.17e}", pdf.xfxq2(21, &[x, q2]));
    println!("PrecisionPDF::xfxq2  {:.17e}", view.xfxq2(21, &x, &q2)?);
    println!("PrecisionPDF::alphas {:.17e}", view.alphas_q2(&q2)?);
    Ok(())
}
```

Note the signature of `PrecisionPDF::xfxq2`: it takes `x` and `q2` **by reference as scalars
of the arithmetic**, `xfxq2(pid, &x, &q2)`, instead of the slice `&[x, q2]` taken
by `PDF::xfxq2`. It returns a `Result`, because the view reports failures instead
of returning a value.

### Usage with MPFR

To evaluate with, say, 256-bit MPFR (Multiple Precision Floating-Point Reliable
Library) floats, implement `Arithmetic` for a small context type. This example
uses the [`rug`](https://crates.io/crates/rug) crate:

```toml
[dependencies]
neopdf = "0.4.0"
rug = { version = "1.28", default-features = false, features = ["float"] }
```

```rust linenums="1"
use std::cmp::Ordering;

use neopdf::pdf::PDF;
use neopdf::precision::Arithmetic;
use neopdf::precision_pdf::PrecisionError;
use rug::Float;

/// MPFR floating-point numbers with a fixed number of mantissa bits.
#[derive(Clone, Copy)]
struct Mpfr {
    bits: u32,
}

impl Arithmetic for Mpfr {
    type Scalar = Float;

    fn lift(&self, value: f64) -> Float {
        // Exact: every f64 is representable with 53 bits or more.
        Float::with_val(self.bits, value)
    }
    fn add(&self, a: &Float, b: &Float) -> Float {
        Float::with_val(self.bits, a + b)
    }
    fn sub(&self, a: &Float, b: &Float) -> Float {
        Float::with_val(self.bits, a - b)
    }
    fn mul(&self, a: &Float, b: &Float) -> Float {
        Float::with_val(self.bits, a * b)
    }
    fn div(&self, a: &Float, b: &Float) -> Float {
        Float::with_val(self.bits, a / b)
    }
    fn ln(&self, a: &Float) -> Float {
        Float::with_val(self.bits, a.ln_ref())
    }
    fn compare(&self, a: &Float, b: &Float) -> Option<Ordering> {
        a.partial_cmp(b)
    }
    fn is_finite(&self, a: &Float) -> bool {
        a.is_finite()
    }
}

fn main() -> Result<(), PrecisionError> {
    let pdf = PDF::load("NNPDF40_nnlo_as_01180", 0);

    let mpfr = Mpfr { bits: 256 };
    let view = pdf.with_arithmetic(mpfr)?;

    let (x, q2) = (mpfr.lift(1e-3), mpfr.lift(1e4));
    let gluon = view.xfxq2(21, &x, &q2)?;
    let alphas = view.alphas_q2(&mpfr.lift(8315.0))?;

    println!("x g(x, Q2) = {gluon:.40}");
    println!("alpha_s    = {alphas:.40}");
    Ok(())
}
```

This prints:

```
x g(x, Q2) = 29.59155878119592248046416566474969528643
alpha_s    = 0.1180023495562482395598442571216052032962
```

The `f64` path gives `2.95915587811959142e1` for the same gluon: it agrees with
the 256-bit value to about \(2 \times 10^{-16}\) relative, which is the rounding
error that `with_arithmetic` removes.

### Supported sets

The view supports **collinear `LogBicubic` sets**. Every subgrid must be a plain
\((x, Q^2)\) grid, with no nucleon number, \(\alpha_s\), \(k_T\), \(\xi\) or
\(\Delta\) dimension. This covers the usual LHAPDF sets, read from LHAPDF `.dat`
files or from NeoPDF grids.

Sets outside this class are rejected when the view is created, with
`PrecisionError::Unsupported`. Examples are Chebyshev grids (`LogChebyshev`), grids
that combine several nuclei (`LogTricubic`, as in `nNNPDF30_nlo_as_0118.neopdf.lz4`)
and TMD grids. A set whose \(\alpha_s\) is analytic rather than tabulated
gets a working `xfxq2`, but its `alphas_q2` returns `PrecisionError::Unsupported`.

### Differences from `PDF::xfxq2`

The view is deliberately stricter than the standard path:

| | `PDF::xfxq2` / `PDF::alphas_q2` | `PrecisionPDF::xfxq2` / `PrecisionPDF::alphas_q2` |
|---|---|---|
| Point outside the grid | Clamps or extrapolates | `Err(OutOfRange)` |
| Non-finite input | Panics | `Err(NonFinite)` |
| Flavour not in the set | `0.0` | `lift(0.0)` |
| Positivity clipping | The current `ForcePositive` | The `ForcePositive` when the view was created |
| Subgrid bounds in \(Q^2\) | The stored `f64` \(Q^2\) knots | For LHAPDF `.dat` sets, the original \(Q\) knots squared in `A` |

Two of these rows need more explanation:

- **No extrapolation.** Extrapolating would be a modelling choice, so the view refuses to
  make it. Check the point with `pdf.param_ranges()` first, or handle `OutOfRange`.
- **Thresholds squared in `A`.** LHAPDF `.dat` files list \(Q\), not \(Q^2\). The standard
  path squares each \(Q\) knot in `f64`; the view squares it in `A`. That way, a flavour
  threshold given by one \(Q\) stays a single point, shared by the subgrids on both sides
  and by the \(\alpha_s\) table. As a consequence, the lowest \(Q^2\) of the view can lie a
  hair above `pdf.param_ranges().q2.min`. For `NNPDF40_nnlo_as_01180`, the lowest \(Q\) is
  1.65:

    ```
    f64 q2 min:          2.72249999999999969801...
    1.65 squared in A:   2.72249999999999970690...
    ```

    `lift(pdf.param_ranges().q2.min)` is then just outside the grid. To evaluate at a
    threshold, square \(Q\) in the arithmetic, `a.mul(&q, &q)`, instead of lifting an `f64`
    \(Q^2\).

The positivity row means that a later `pdf.set_force_positive(...)` does not affect a view
that already exists. Create the view after setting the clipping you want.

### Cost

Measured on `NNPDF40_nnlo_as_01180` (for the gluon PDF) over 2000 points spread across the
grid:

| Arithmetic | Building the view | Per `xfxq2` |
|---|---|---|
| `PDF::xfxq2` (reference) | | 93 ns |
| `Binary64` | 3 ms | 81 ns |
| MPFR, 64 bits | 134 ms | 5.1 µs |
| MPFR, 128 bits | 164 ms | 8.6 µs |
| MPFR, 256 bits | 221 ms | 13 µs |
| MPFR, 512 bits | 275 ms | 19 µs |

The cost is set by the arithmetic: the view performs the same number of operations whatever
`A` is. Across those 2000 points, the 256-bit and the `f64` results differ by at most
\(1.1 \times 10^{-15}\) relative.
