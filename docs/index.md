---
title: ""
hide:
  - title
---

<style>
.md-content__inner h1:first-of-type { display: none; }
</style>

<div style="display: flex; justify-content: center;">
  <img src="./assets/neopdf.svg" width="600" />
</div>

<div style="display: flex; justify-content: center;">
<p>
    <img src="https://img.shields.io/codecov/c/github/QCDLab/neopdf?style=flat-square&logo=codecov&logoColor=red&color=blue" alt="Codecov">
    <img src="https://img.shields.io/crates/msrv/neopdf?style=flat-square&logo=rust&color=red" alt="MSRV">
    <img src="https://img.shields.io/crates/v/neopdf?style=flat-square&logo=rust&color=blue" alt="Crates.io">
    <img src="https://img.shields.io/pypi/pyversions/neopdf-hep?style=flat-square&logo=python" alt="PyPI - Python Version">
    <img src="https://img.shields.io/pypi/v/neopdf-hep?style=flat-square&logo=python&logoColor=yellow&color=%231d881d" alt="PyPI - Version">
    <img src="https://img.shields.io/github/license/Radonirinaunimi/neopdf?style=flat-square&logo=gplv3&logoColor=red" alt="GitHub License">
</p>
</div>

`NeoPDF` is a fast, reliable, and scalable interpolation library for **Non-Perturbative Distribution Functions**
with **modern features**, designed for both present and future hadron collider experiments. It aims to be a modern,
high-performance alternative to both [LHAPDF](https://www.lhapdf.org/) and [TMDlib](https://tmdlib.hepforge.org/),
focusing on:

<div class="feature-grid">
  <div class="feature-card">
      <strong>🚀 Performance</strong>
      <p>Written in Rust 🦀 for speed and safety, with zero-cost abstractions and efficient memory management.</p>
  </div>
  <div class="feature-card">
      <strong>🧩 Flexibility</strong>
      <p>Easy support for different interpolation strategies, enabling seamless/efficient implementation of new methods.</p>
  </div>
  <div class="feature-card">
      <strong>🌐 Multi-Language Support</strong>
      <p>Native Rust 🦀 API, with bindings for various programming languages such as Python, Fortran, C, C++, Mathematica.</p>
  </div>
  <div class="feature-card">
      <strong>📊 (Physics) Features & Extensibility</strong>
      <p>Very extensible, which makes it easy to introduce new (Physics) features without introducing <b>technical debts</b>.</p>
  </div>
</div>

## Motivation

The need for a fast and reliable PDF interpolation is critical in high-energy physics, especially
for precision calculations at hadron colliders. Existing solutions like LHAPDF or TMDlib, while
widely used, have limitations in terms of extensibility and features. `NeoPDF` addresses these by:

- Providing a modern and modular codebase with efficient file format
- Enabling easy integration into new and existing workflows
- Supporting advanced features such as multi-dimensional interpolations for up to 6D data

## High-Level Architecture

- **Core Library (Rust)**: Implements all the interpolation logics, grid management, and PDF
    metadata handling.
- **FFI Bindings**: Exposes the core functionalities to Python, Fortran, C, C++, and Mathematica, enabling
    easier interoperability with other codes that can link to these programming languages.
- **CLI Tools**: Command-line utilities that allow users to inspect the contents of a gird,
    convert LHAPDF/TMDlib format into `NeoPDF`, and perform interpolations.

## Getting Started

- **[Installation Guide](./installation.md)**: Complete installation instructions for all platforms and APIs
- **[Development with Pixi](./development-with-pixi.md)**: Comprehensive guide for using Pixi environment manager
- **[CLI Tutorials](./cli-tutorials.md)**: Showcase how to use the command-line interface
- **[Examples](./examples/)**: Code examples for Rust, Python, C++, C, Fortran, and Mathematica APIs

---

## Additional Resources

- [Pixi Documentation](https://pixi.sh/latest/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [PyO3 Documentation](https://pyo3.rs/)
- [Cargo-c Documentation](https://crates.io/crates/cargo-c)
- [Maturin Documentation](https://www.maturin.rs/)

For more detailed information about `NeoPDF` development, see the [GitHub repository](https://github.com/QCDLab/neopdf).
