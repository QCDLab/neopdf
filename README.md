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
  <b>NeoPDF</b> is a fast, reliable, and scalable interpolation library for both <b>collinear</b>
  and <b>transverse momentum dependent</b> Parton Distribution Functions with <b>modern features</b>
  designed for both present and future hadron collider experiments:

  <ul>
    <li>
    <p align="justify">
      Beyond interpolations over the kinematic variables (<b>x</b>, <b>kT</b>, <b>Q2</b>), NeoPDF
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

## Quick Links

- [Documentation](https://qcdlab.github.io/neopdf/) | [Rust Crate Documentation](https://docs.rs/neopdf/0.1.1/neopdf/) | [C++ API Reference](https://neopdf.readthedocs.io/en/latest/)
- [Installation](https://qcdlab.github.io/neopdf/installation/)
- [Physics and technical features](https://qcdlab.github.io/neopdf/design-and-features/)
- [NeoPDF Design](https://qcdlab.github.io/neopdf/design/)
- [CLI tutorials](https://qcdlab.github.io/neopdf/cli-tutorials/)
- [Tutorials and examples](https://qcdlab.github.io/neopdf/examples/neopdf-pyapi/)

## Citation

<p align="justify">
  If you use NeoPDF, please cite the NeoPDF <a href="https://arxiv.org/abs/2510.05079">paper</a> and
  <a href="https://zenodo.org/records/17286770">code</a>. If you are also using the
  <a href="https://arxiv.org/abs/2103.09741">TMDlib</a> interface of NeoPDF and/or
  <a href="https://arxiv.org/abs/1412.7420">LHAPDF</a>, please cite them accordingly.
</p>

> [!NOTE]
> As of v0.2.0, NeoPDF (and in particular its APIs) is stable and will fully maintain backward compatibility.
