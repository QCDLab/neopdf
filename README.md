<p align="center">
  <img src="https://github.com/Radonirinaunimi/neopdf/blob/master/logo/neopdf.svg" alt="NeoPDF Logo" width="450">
</p>
<div align="center">
    <a href="https://app.codecov.io/gh/Radonirinaunimi/neopdf/tree/master"><img
        alt="Codecov"
        src="https://img.shields.io/codecov/c/github/Radonirinaunimi/neopdf?style=for-the-badge&logo=codecov&logoColor=red&color=blue"
        height="22"
    /></a>
    <a href="https://gribnau.dev/cargo-msrv/"><img
        alt="MSRV"
        src="https://img.shields.io/crates/msrv/neopdf?style=for-the-badge&logo=rust&color=red"
        height="22"
    /></a>
    <a href="https://crates.io/crates/neopdf"><img
        alt="Crates.io"
        src="https://img.shields.io/crates/v/neopdf?style=for-the-badge&logo=rust&color=blue"
        height="22"
    /></a>
    <a href="https://pypi.org/project/neopdf-hep/"><img
        alt="PyPI - Version"
        src="https://img.shields.io/pypi/v/neopdf-hep?style=for-the-badge&logo=python&logoColor=yellow&color=%1d881d"
        height="22"
    /></a>
    <a href="https://github.com/qcdlab/neopdf?tab=GPL-3.0-1-ov-file"><img
        alt="GitHub License"
        src="https://img.shields.io/github/license/qcdlab/neopdf?style=for-the-badge&logo=gplv3&logoColor=red"
        height="22"
    /></a>
</div>

<p align="justify">
  <b>NeoPDF</b> is a fast, reliable, and scalable interpolation library for <b>Non-Perturbative Distribution Functions</b>
  with <b>modern features</b> designed for both present and future hadron collider experiments:

  <ul>
    <li>
    <p align="justify">
      Beyond interpolations over the kinematic variables (<b>x</b>, <b>ξ</b>, <b>Δ</b>, <b>kT</b>, <b>Q²</b>), NeoPDF
      also supports interpolations along the nucleon numbers <b>A</b> (relevant for <b>nuclear</b> PDFs
      and TMDs) and the strong coupling <b>αs(MZ)</b>.
    </p>
    </li>
    <li>
    <p align="justify">
      NeoPDF implements its own file format using binary serialization and <a href="https://lz4.org/">LZ4</a>
      compression, prioritizing speed and efficiency over human-readable formats. A command Line
      Interface (CLI) is provided to easily inspect and perform various operations on NeoPDF grids.
    </p>
    </li>
    <li>
    <p align="justify">
      NeoPDF is esigned as much as possible with a “no-code migration” philosophy across the various API
      interfaces (Fortran, C/C++, Python, Mathematica). It thus preserves naming conventions and method
      signatures in close alignment with LHAPDF, ensuring that existing codes can switch to NeoPDF with
      minimal or no modifications.
    </p>
    </li>
  </ul>
</p>

## Supported distributions

<p align="justify">
  NeoPDF supports generic classes of distributions that are functions of different combinations of the kinematic
  variables. Examples of distribution functions are given in the diagram below with their simplified relationships.
</p>

```mermaid
graph TD

A["**Generalized Tranverse Momentum Distribution** <br> GTMD(x, ξ, Δ, kT​​, Q²)"]

A -->|∫ dkT| B["**Generalized Parton Distributions** <br> GPD(x, ξ, Δ, Q²)"]
A -->|ξ → 0, Δ → 0| C["**Transverse Momentum Distributions** <br> TMD(x, kT, Q²)"]

B -->|ξ → 0, Δ → 0| D["**Collinear Parton Distribution Functions** <br> PDF(x, Q²)"]
C -->|∫ dkT| D

style A fill:#ffd580,stroke:#b8860b,stroke-width:2px
style B fill:#add8e6,stroke:#1e90ff,stroke-width:2px
style C fill:#98fb98,stroke:#228b22,stroke-width:2px
style D fill:#f08080,stroke:#8b0000,stroke-width:2px
```

## Quick Links

- [Documentation](https://qcdlab.github.io/neopdf/) | [Rust Crate Documentation](https://docs.rs/neopdf/0.1.1/neopdf/) | [C++ API Reference](https://neopdf.readthedocs.io/en/latest/)
- [Installation](https://qcdlab.github.io/neopdf/installation/)
- [Physics and technical features](https://qcdlab.github.io/neopdf/design-and-features/)
- [NeoPDF Design](https://qcdlab.github.io/neopdf/design/)
- [CLI tutorials](https://qcdlab.github.io/neopdf/cli-tutorials/)
- [Tutorials and examples](https://qcdlab.github.io/neopdf/examples/neopdf-pyapi/)

## Citation

<p align="justify">
  If you use NeoPDF, please cite the <a href="https://zenodo.org/records/17286770">software</a>
  itself and the <a href="https://arxiv.org/abs/2510.05079">paper</a>. If you are also using the
  <a href="https://arxiv.org/abs/2103.09741">TMDlib</a> interface of NeoPDF and/or
  <a href="https://arxiv.org/abs/1412.7420">LHAPDF</a>, please cite them accordingly.
</p>

> [!NOTE]
> As of v0.2.0, NeoPDF (and in particular its APIs) is stable and will fully maintain backward compatibility.
