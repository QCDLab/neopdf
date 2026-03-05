# NeoPDF-HEP

![Static Badge](https://img.shields.io/badge/Python-3.7--3.13-00A800?style=flat-square&logo=python&logoColor=yellow)
![GitHub Tag](https://img.shields.io/github/v/tag/Radonirinaunimi/neopdf?include_prereleases&style=flat-square&logo=github&color=blue)
![Crates.io Version](https://img.shields.io/crates/v/neopdf?style=flat-square&logo=rust&color=blue)
![GitHub License](https://img.shields.io/github/license/Radonirinaunimi/neopdf?style=flat-square&color=red)

This crate uses [PyO3](https://pyo3.rs/v0.25.1/) to provide Python bindings to the
[NeoPDF](https://crates.io/crates/neopdf) Rust's [APIs](https://docs.rs/neopdf/0.1.0/neopdf/index.html).
The `neopdf-hep` package can be installed using a Python Package manager. For example
using [pipx](https://pipx.pypa.io/stable/):

```sh
pipx install neopdf-hep
```

To re-generate the `matplotlib` figures, simply run:

```sh
pytest neopdf_pyapi/tests/test_interpolations.py --mpl-generate-path=neopdf_pyapi/tests/baseline
```

For detailed examples on how to use `neopdf-hep`, please head over the
[tutorials](https://radonirinaunimi.github.io/neopdf/examples/neopdf-pyapi/) section.
