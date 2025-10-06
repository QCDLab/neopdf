# Design

## PDF Object Construction: Multi-dimensional Grid Representation

The core of `NeoPDF`'s data model is the representation of Parton Distribution Functions (PDFs)
and Transverse Momentum Distributions (TMDs) as multi-dimensional arrays. This is implemented
via the `GridArray` and `SubGrid` structures in the `neopdf::gridpdf` and `neopdf::subgrid` modules
respectively.

- **GridArray**: Stores the full set of subgrids and flavor IDs. Each subgrid represents a region
  of phase space with a consistent grid of variables ($A$, $\alpha_s$, $k_T$, $x$, $Q^2$).
- **SubGrid**: In the most general case, contains a **6-dimensional** array: `[nucleons, alphas, pids, kT, x, Q²]`.
  For the standard case of proton PDF, for example, `SubGrid` is a **3-dimensional** array of `[pids, x, Q²]`
  that requires a 2D interpolation. This allows for efficient storage and interpolation across all
  relevant physical parameters.
- **Interpolation**: The library supports up to 5D interpolation strategies, automatically
  selecting the appropriate method based on the grid structure and metadata. Interpolators are
  built for each subgrid and flavor, supporting log-space and linear strategies for high accuracy.

Therefore, using the notations from [[2112.09703](https://arxiv.org/pdf/2112.09703)], the full
`GridArray` (for 1D grid), which is a conjunction of $k$ subgrids, can be represented as:

$$ \left[ z_0, z_1, \cdots, z_{\mathrm{max}} \right]_{\left( n_1, n_2, \cdots, n_k \right)} \qquad \text{with} \qquad z=A, \alpha_s, k_T, x, Q^2 $$

where the $z_i$ are the subinterval boundaries and $n_i = N_i + 1$ is the number of points for
subgrid $i$. Note that adjacent subgrids share their end points so that the total number of grid
points is $n_{\mathrm{pts}} = \sum_i n_i - (k - i)$.

The diagram below schematically summarizes the `NeoPDF` data structure:

![grid](https://github.com/user-attachments/assets/0637f61f-348f-4507-a6f5-08fac127368f){ .center-medium }

where the `GridArray` object represents an instance of a set member. The subgrid $S_k$ is a
**hyperrectangle** given by the **Cartesian** product of the dependent variables.

This design enables:

- Efficient access to PDF/TMD values at arbitrary kinematic points for a given member.
- Support for advanced use cases, such as nuclear PDFs and strong coupling $\alpha_s(M_Z)$.
- Modular extension to new interpolation strategies or additional dimensions.

## NeoPDF File Format: Compression, Metadata, and Lazy Loading

The `NeoPDF` file format is designed for efficient storage and fast, random access to large
collections of grids. For technical reasons (see below), the format is **not human-readable**.
However, `NeoPDF` provides a [CLI tool](/docs/cli-tutorials.md) that allows the user to easily and
quickly inspect the contents of a given PDF/TMD set. The implementation of the logics is found in
`neopdf::writer`.

- **Serialization & Compression**: All grid data (`GridArray`), along with shared metadata, are
  serialized using `bincode` and compressed with LZ4. This results in compact files that are quick
  to read and write.
- **Metadata**: Metadata is stored at the beginning of the file, allowing extraction without
  decompressing the entire file.
- **Offset Table**: An offset table is written after the metadata, enabling random access to any
  grid member without reading the whole file.
- **Grid Data**: Each grid is stored with its size and data, allowing for efficient deserialization.

Such a choice of format allows `NeoPDF` grids to be efficiently stored on disk, reducing at least
by half the size of a given LHAPDF PDF set:

| PDF Set            | Nb. Members | LHAPDF/TMDlib | NeoPDF |
|--------------------|-------------|---------------|--------|
| PDF4LHC21          | 40          | 31 MB         | 16 MB  |
| NNPDF4.0 NNLO      | 100         | 158 MB        | 85 MB  |
| NNPDF4.0 NNLO      | 1000        | 1.55 GB       | 830 MB |
| Combined nNNPDF3.0 | 19*200      | 19*160 MB     | 1.43 GB|
| MAP22 FF @N3LL     | 250         | 2.50 GB       | 950 MB |

!!! info "Note"

    The size of the `NeoPDF` sets could be reduced further (at least by half) by using the Chebyshev
    grid spacing and interpolation.

### Access Patterns
- **Eager Loading**: The entire collection of grids can be decompressed and loaded into memory for
  batch operations.
- **Random Access**: The `GridArrayReader` provides random access to individual grids using the offset
  table, without loading all data.
- **Lazy Iteration**: The `LazyGridArrayIterator` enables sequential, memory-efficient iteration over
  grid members, suitable for processing very large sets.

### Advantages
- **Performance**: LZ4 compression and binary serialization provide fast read/write speeds and small
  file sizes.
- **Scalability**: Lazy and random access patterns allow working with very large PDF sets without
  high memory usage.
- **Extensibility**: The format is designed to accommodate future extensions, such as new metadata
  fields or additional grid dimensions.
