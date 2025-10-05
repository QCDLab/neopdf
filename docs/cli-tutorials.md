# CLI Tutorials

The `neopdf` CLI tool provides a command-line interface to the `NeoPDF` library, enabling users
to perform PDF interpolation, alpha_s evaluation, file format conversion, and metadata inspection
directly from the terminal. Below are detailed usage instructions and examples for each major
feature.

!!! note "How to use the new NeoPDF format?"

    - To use the `NeoPDF` grid format, if available, simply append `.neopdf.lz4` to the name of the PDF set; LHAPDF
      sets can be converted into the NeoPDF format using the CLI (see below)
    - The CLI supports both eager and lazy loading of grids, and can operate on both standard and compressed formats
    - For more advanced usage, see the help output:

    ```bash
    neopdf --help
    ```

---

## Reading PDF Metadata and Grid Information

### Read Metadata for a PDF Set

To read the metadata for a given PDF set:

```bash
neopdf read metadata NNPDF40_nnlo_as_01180
```

- `--pdf-name`: Name of the PDF set (can be a `NeoPDF` or `LHAPDF` set).

The above command will print out the following output:

```yaml
Set Description: NNPDF4.0 NNLO global fit, alphas(MZ)=0.1180. mem=0 => average on replicas; mem=1-100 => PDF replicas
Set Index: 331100
Number of Members: 101
XMin: 0.000000001
XMax: 1
QMin: 1.65
QMax: 100000
Flavors: [-5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5]
Format: lhagrid1
...
Polarized: false
Set Type: SpaceLike
Interpolator Type: LogBicubic
...
```

### Get the Number of Subgrids

To get the number of subgrids a PDF contains:

```bash
neopdf read num_subgrids NNPDF40_nnlo_as_01180
```

- `--pdf-name`: Name of the PDF set.

### Get Knot Values for a Subgrid

To get the knot values for a given subgrid:

```bash
neopdf read subgrid-info NNPDF40_nnlo_as_01180 --member=0 --subgrid-index=1
```

- `--pdf-name`: Name of the PDF set.
- `--member`: The member index (0-based).
- `--subgrid-index`: The index of the subgrid to inspect.

The above command will print out the following output:

```yaml
Nucleon Numbers A: [0.0], shape=[1], strides=[1], layout=CFcf (0xf), const ndim=1
Alphas values: [0.0], shape=[1], strides=[1], layout=CFcf (0xf), const ndim=1
kT values: [0.0], shape=[1], strides=[1], layout=CFcf (0xf), const ndim=1
x values: [1e-9, 1.2970848e-9, ...], shape=[196], strides=[1], layout=CFcf (0xf), const ndim=1
Q2 values: [2.7224999999999997, ...], shape=[12], strides=[1], layout=CFcf (0xf), const ndim=1
```

!!! note

    Note that the values of `A` and `alphas` are set to zero by default as there is no
    proper way to extract their values from LHAPDF.

### Updating Grid Metadata

The metadata of a given `NeoPDF` grid can be updated by running the following command:

```bash
neopdf write metadata --path NNPDF40_nnlo_as_01180.neopdf.lz4 --key SetType --value TimeLike
```

- `--path`: Path to the `NeoPDF` set.
- `--key`: The key value of the metadata to be updated, see the `rename` field of the
  [MetaData](https://github.com/Radonirinaunimi/neopdf/blob/master/neopdf/src/metadata.rs#L33-L121)
  struct for the available keys.
- `--value`: The updated value of the metadata.

---

## Computing Interpolated Values

### Compute interpolated PDF values $xf(x, Q^2)$

To compute the interpolated function $xf(x, Q^2)$ for a given PDF set, member, and flavor:

```bash
neopdf compute xfx_q2 NNPDF40_nnlo_as_01180 --member=0 --pid=21 1e-3 10.0
```

- `--pdf-name`: Name of the PDF set (can be a `NeoPDF` or `LHAPDF` set).
- `--member`: The member index (0-based).
- `--pid`: The PDG ID of the parton flavor (e.g., 21 for gluon).
- If the PDF set doesn't have the $\alpha_s$ and $A$ dependence, then the last two arguments
  are simply the $x$ and $Q^2$ values at which to evaluate the PDF.

!!! note "More about the last argument"

    In the case where the PDF set contains an $\alpha_s$ and/or $A$ dependence, the the
    argument should be either $(A, x, Q^2)$, $(\alpha_s, x, Q^2)$, $(k_T, x, Q^2)$,
    $(A, \alpha_s, x, Q^2)$, $(A, k_T, x, Q^2)$, $(\alpha_s, k_T, x, Q^2)$, or
    $(A, \alpha_s, k_T, x, Q^2)$ depending on what the grid contains. See the section below
    for an example where the grid contains an $A$-dependence.

### Compute strong coupling $\alpha_s$ at a given $Q^2$

To compute the strong coupling constant $\alpha_s(Q^2)$ for a given PDF set and member:

```bash
neopdf compute alphas_q2 NNPDF40_nnlo_as_01180 --member=0 --q2=10
```

- `--q2`: The $Q^2$ value at which to evaluate $\alpha_s$.

---

## Converting LHAPDF Sets into NeoPDF Format

### Convert standard LHAPDF to NeoPDF format

To convert a standard LHAPDF set into the new, compressed `NeoPDF` format:

```bash
neopdf write convert NNPDF40_nnlo_as_01180 --output NNPDF40_nnlo_as_01180.neopdf.lz4
```

- `--output`: The output file name for the `NeoPDF` grid (should end with `.neopdf.lz4`).

### Combine multiple LHAPDF Nuclear PDFs

To combine several LHAPDF nuclear PDF sets into a single `NeoPDF` grid:

```bash
neopdf write combine-npdfs --pdf-names nNNPDF30_nlo_as_0118_p nNNPDF30_nlo_as_0118_A12 nNNPDF30_nlo_as_0118_A40 --output nNNPDF30_nlo_as_0118.neopdf.lz4
```

- `--pdf-names`: List of PDF set names to combine.
- `--output`: Output file for the combined grid.

Alternatively, one can pass the names of the PDFs via an input file where each line
contains a PDF name. Taking the example of nuclear PDFs above but using all the $A$
sets available,

```yaml title="nuclearpdfs.txt"
nNNPDF30_nlo_as_0118_p
nNNPDF30_nlo_as_0118_A2_Z1
nNNPDF30_nlo_as_0118_A4_Z2
nNNPDF30_nlo_as_0118_A6_Z3
nNNPDF30_nlo_as_0118_A9_Z4
nNNPDF30_nlo_as_0118_A12_Z6
nNNPDF30_nlo_as_0118_A14_Z7
nNNPDF30_nlo_as_0118_A16_Z8
nNNPDF30_nlo_as_0118_A27_Z13
nNNPDF30_nlo_as_0118_A31_Z15
nNNPDF30_nlo_as_0118_A40_Z20
nNNPDF30_nlo_as_0118_A56_Z26
nNNPDF30_nlo_as_0118_A64_Z29
nNNPDF30_nlo_as_0118_A108_Z54
nNNPDF30_nlo_as_0118_A119_Z59
nNNPDF30_nlo_as_0118_A131_Z54
nNNPDF30_nlo_as_0118_A184_Z74
nNNPDF30_nlo_as_0118_A197_Z79
nNNPDF30_nlo_as_0118_A208_Z82
```

the command now becomes:

```bash
neopdf write combine-npdfs --names-file nuclearpdfs.txt --output nNNPDF30_nlo_as_0118.neopdf.lz4
```

This will generate a `nNNPDF30_nlo_as_0118.neopdf.lz4` PDF grid that also contains the $A$
dependence. One can now check the value of $xf(A, x, Q^2)$ for the Iron $^{56}_{26}\mathrm{Fe}$:

```bash
neopdf compute xfx_q2 nNNPDF30_nlo_as_0118.neopdf.lz4 --member=0 --pid=21 56 1e-3 10.0
```

### Combine multiple  $\alpha_s$ LHAPDF sets

To combine multiple $\alpha_s$ LHAPDF sets into a single NeoPDF grid with an explicit dependence
on $\alpha_s$, the procedure is the same as when combining multiple nuclear PDFs with the option
`combine-npdfs` replaced with `combine-alphas`. That is, given an input file with the names of the
PDF sets:

```yaml title="alphaspdfs.txt"
NNPDF40_nnlo_as_01160
NNPDF40_nnlo_as_01170
NNPDF40_nnlo_as_01175
NNPDF40_nnlo_as_01185
NNPDF40_nnlo_as_01190
```

The command to run is:

```bash
neopdf write combine-alphas --names-file alphaspdfs.txt --output NNPDF40_nnlo.neopdf.lz4
```

Note that the names of the PDF sets can be passed via the command line using the option
`--pdf--name`. We can then interpolate the $\alpha_s = 0.1180$ value:

```bash
neopdf compute xfx_q2 NNPDF40_nnlo.neopdf.lz4 --member=0 --pid=21 0.1180 1e-3 10.0
```

!!! danger "Warning"

    Given that LHAPDF doesn't provide specific attributes to extract the values of $A$ and $\alpha_s$,
    their values are inferred from the set name. The extraction of these values are therefore set
    dependent and we try to support as many sets as possible while keeping the implementation modular.
    The current extraction of these values rely on the following regex:

    ```rust
    // Regexes to extract A from the PDF set name
    let re_nnpdf = Regex::new(r"_A(\d+)").unwrap();
    let re_ncteq = Regex::new(r"_(\d+)_(\d+)$").unwrap();
    let re_epps = Regex::new(r"[a-zA-Z]+(\d+)$").unwrap();

    // Regexes to extract alpha_s from the PDF set name
    let re_nnpdf_ct = Regex::new(r"_as_(\d+)_?").unwrap();
    let re_msht = Regex::new(r"_as(\d+)").unwrap();
    ```

    This approach is not fullproof and some PDF sets might not be supported at all.

---

## Inspecting subgrid contents

The contents of the subgrids can be printed into human-readable format. Given that a set might contain
multiple subgrids (as we have seen before using the `subgrid-info` command), it is instructive to check
first how many subgrids a given set member contains.

```bash
> neopdf read num_subgrids NNPDF40_nnlo_as_01180.neopdf.lz4

2
```

We can look at the contents of the first subgrid for the **gluon** PDF:

```bash
> neopdf read subgrid NNPDF40_nnlo_as_01180.neopdf.lz4 --member=0 --subgrid-index=0 --pid=21

  [x | Q2]   2.72250e0   3.19494e0   3.77488e0   4.49175e0   5.38430e0   6.50400e0   7.91974e0
1.00000e-9  1.48441e-1  -1.47266e0  -3.42816e0  -5.57841e0  -7.73893e0  -9.65268e0  -1.10375e1
1.29708e-9  1.53954e-1  -1.36579e0  -3.16487e0  -5.10066e0  -6.98714e0  -8.57386e0  -9.57904e0
1.68243e-9  1.59670e-1  -1.26122e0  -2.90961e0  -4.64076e0  -6.26809e0  -7.54804e0  -8.20073e0
2.18225e-9  1.65601e-1  -1.15891e0  -2.66220e0  -4.19823e0  -5.58080e0  -6.57342e0  -6.89951e0
2.83057e-9  1.71754e-1  -1.05882e0  -2.42248e0  -3.77263e0  -4.92430e0  -5.64825e0  -5.67242e0
3.67149e-9  1.78142e-1 -9.60911e-1  -2.19028e0  -3.36352e0  -4.29764e0  -4.77083e0  -4.51658e0
...
  [x | Q2]   9.72449e0   1.20449e1   1.50550e1   1.89961e1   2.42064e1
1.00000e-9  -1.15252e1  -1.06011e1  -7.88260e0  -2.66303e0   5.44604e0
1.29708e-9  -9.64489e0  -8.29955e0  -5.12278e0  5.21491e-1   8.89615e0
1.68243e-9  -7.87939e0  -6.15103e0  -2.56742e0   3.44402e0   1.20386e1
2.18225e-9  -6.22381e0  -4.14868e0 -2.06304e-1   6.11881e0   1.48905e1
2.83057e-9  -4.67345e0  -2.28589e0   1.97036e0   8.55949e0   1.74680e1
...
```
