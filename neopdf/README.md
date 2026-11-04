# NeoPDF

![Crates.io MSRV](https://img.shields.io/crates/msrv/neopdf?style=flat-square&logo=rust&color=orange)
![Crates.io Version](https://img.shields.io/crates/v/neopdf?style=flat-square&logo=rust&color=blue)
![GitHub Tag](https://img.shields.io/github/v/tag/QCDLab/neopdf?include_prereleases&style=flat-square&logo=github&color=blue)
![GitHub License](https://img.shields.io/github/license/QCDLab/neopdf?style=flat-square&color=red)

**NeoPDF** is a fast, reliable, and scalable interpolation library for both **Collinear** Parton
Distribution Functions (PDFs) and **Transverse Momentum Dependent** Distributions (TMDs) with
**modern features**, designed for both present and future hadron collider experiments. It aims
to be a modern, high-performance alternative to both [LHAPDF](https://www.lhapdf.org/) and
[TMDlib](https://tmdlib.hepforge.org/), focusing on:

- **Performance**: Written in Rust for speed and safety, with zero-cost abstractions and efficient
    memory management.
- **Flexibility**: Supports multiple interpolation strategies and is easily extensible. The
    abstraction of the interpolation crate makes it easier and efficient to implement custom
    interpolation methods.
- **Multi-language Support**: Native Rust API, with bindings for Python, Fortran, C, and C++.
- **Features and Extensibility**: `NeoPDF` is very extensible and therefore makes it easier
    to introduce new (Physics) features without introducing **technical debts**.
