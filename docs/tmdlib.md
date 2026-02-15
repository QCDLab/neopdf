# TMDlib Interface

`NeoPDF` provides an interface to the `TMDlib` library which allows users for example
to interpolate TMD PDFs from the CLI. Most importantly, it is used to convert TMDlib
sets into `NeoPDF` formats.

## Installing the interface

To install the `TMDlib` interface, first clone the repository and run the following
command:

```bash
cargo install --path neopdf_cli --features=tmdlib
```

Note that this requires `TMDlib` and its dependencies to be installed and available
in the environment paths.

## Interpolating TMDlib sets

To interpolate `TMDlib` sets from the CLI, the command is similar to the one used for
regular sets:

```bash
neopdf compute xfx_q2_kt --pdf-name MAP22_grids_FF_Km_N3LL --member 0 --pid 2 0.00010000000000000009 1.00000e-1 4.14987e3
```

For more details on the required arguments, refer to the helper:

```bash
> neopdf compute xfx_q2_kt --help                                                                                                                                                                                                                                                                                         ─╯

Evaluate TMD PDF for a given set, member, and input values

Usage: neopdf compute xfx_q2_kt --pdf-name <PDF_NAME> --member <MEMBER> --pid <PID> <INPUTS>...

Arguments:
  <INPUTS>...  Input values (kt, x, Q2)

Options:
  -p, --pdf-name <PDF_NAME>  Name of the TMD PDF set
  -m, --member <MEMBER>      Member index (0-based)
  -i, --pid <PID>            PDG flavor ID
  -h, --help                 Print help
```

## Converting TMDlib sets into NeoPDF

To convert a `TMDlib` set into the `NeoPDF` format, one needs to write a configuration file
in the `.toml` format. This configuration file should contain all the information about the
metadata and the grid configurations. The following is an example:

```toml
set_name = "MAP22_grids_FF_Km_N3LL"
set_desc = "MAP22 TMDs for K- fragmentation, converted to NeoPDF"
set_index = 0
n_members = 2

# Inner edges for the subgrids. Leave empty for no subgrids.
x_inner_edges = [0.2]
q_inner_edges = [] # Q, not Q2
kt_inner_edges = [1e-2, 1.0]

# Number of points for (subg)grids.
n_x = [5, 5]
n_q = [6]
n_kt = [5, 5, 4]

# Grid axes that are not part of the TMD interpolation
nucleons = [0.0] # dummy value
alphas = [0.118] # alpha_s(M_Z)

# Metadata
pids = [-3, -2, -1, 21, 1, 2, 3]
polarised = false
set_type = "TimeLike"
interpolator_type = "LogChebyshev" # or LogTricubic
error_type = "replicas"
hadron_pid = 321 # Kaon
alphas_qs = [91.1876] # dummy values
alphas_vals = [0.118] # dummy values

# Physics Parameters
flavor_scheme = "fixed"
order_qcd = 2
alphas_order_qcd = 2
m_w = 80.352
m_z = 91.1876
m_up = 0.0
m_down = 0.0
m_strange = 0.0
m_charm = 1.51
m_bottom = 4.92
m_top = 172.5
alphas_type = "ipol"
number_flavors = 4
```

We can now convert the `MAP22_grids_FF_Km_N3LL` set by running the following command:

```bash
neopdf write convert-tmd --input MAP22_grids_FF_Km_N3LL.toml --output MAP22.neopdf.lz4
```

Once the conversion has been performed successfully (and the converted `NeoPDF` set
available in the `$NEOPDF_DATA_PATH`), we can inspect the content of the subgrids to make
sure that everything is correct.

```bash
> neopdf read subgrid --pdf-name MAP22.neopdf.lz4 --member 0 --subgrid-index 0 --pid 2 --kt-index 0                                                                                                                                                                                                                       ─╯

Displaying grid for kT = 0.00010000000000000009

  [x | Q2]   1.00000e0   2.40972e0   2.40972e1   4.14987e2   4.14987e3   1.00000e4
1.00000e-1 -6.70867e-1 -4.80573e-1 -1.53370e-1 -1.26831e-2  1.49324e-2   0.00000e0
1.10684e-1 -7.26434e-1 -4.81932e-1 -1.40332e-1 -7.62762e-3  1.61799e-2   0.00000e0
1.41421e-1 -7.37791e-1 -3.84220e-1 -8.14145e-2  8.54183e-3  1.92982e-2   0.00000e0
1.80695e-1 -5.23889e-1 -1.84548e-1 -1.28827e-2  2.17623e-2  1.98305e-2   0.00000e0
2.00000e-1 -3.89947e-1 -1.06392e-1  7.02618e-3  2.38316e-2  1.86428e-2   0.00000e0
```

We can verify that the contents of the subgrids are correct by checking one of the entries:

```bash
> neopdf compute xfx_q2_kt --pdf-name MAP22_grids_FF_Km_N3LL --member 0 --pid 2 0.00010000000000000009 1.00000e-1 2.40972e0                                                                                                                                                                                               ─╯

-0.4805722561384916
```
