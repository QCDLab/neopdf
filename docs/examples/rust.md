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
