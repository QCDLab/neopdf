# Mathematica Interface Example

The following provides a short example on how to use the `Mathematica` interface
of `NeoPDF`. More examples can be found in
[neopdf_wolfram](https://github.com/QCDLab/neopdf/tree/master/neopdf_wolfram).

## Prerequisites

In order to use the interface, we first need to install the bindings to the Wolfram
LibraryLink and the Wolfram Language. To do so, clone the repository and go into the
`neopdf_wolfram` directory. Then run the following command:

```bash
cargo build --release --manifest-path neopdf_wolfram/Cargo.toml
```

This will generate a `libneopdf_wolfram.dylib` in the `$PWD/target/release/` directory
which can be loaded in Mathematica using the `LibraryFunctionLoad` function.

## Example interpolating PDFs

The following example illustrates how to load and interpolate PDFs from `Mathematica`.

```Mathematica linenums="1"
(* ::Package:: *)

(* Mathematica example for neopdf_wolfram *)

(* Step 1: Load the library *)
(* Replace with the actual path to the compiled library *)
libPath = "/path/to/neopdf/target/release/libneopdf_wolfram.dylib";

loadPDF = LibraryFunctionLoad[libPath, "NeoPDF_Load", {"UTF8String", Integer}, Integer];
xfxq2 = LibraryFunctionLoad[libPath, "NeoPDF_XFXQ2", {Integer, Integer, {Real, 1}}, Real];
alphasQ2 = LibraryFunctionLoad[libPath, "NeoPDF_AlphasQ2", {Integer, Real}, Real];
clearPDFs = LibraryFunctionLoad[libPath, "NeoPDF_Clear", {}, "Void"];

(* Step 2: Load a PDF set *)
pdfName = "NNPDF40_nnlo_as_01180";
member = 0;
pdfIndex = loadPDF[pdfName, member];
Print["Loaded PDF set ", pdfName, " with member ", member, " at index ", pdfIndex];

(* Step 3: Calculate xfxq2 *)
pid = 21; (* gluon *)
x = 0.01;
q2 = 100.0;
points = {x, q2};
resultXFXQ2 = xfxq2[pdfIndex, pid, points];
Print["xfxq2 for pid=", pid, ", x=", x, ", q2=", q2, " is ", resultXFXQ2];

(* Step 4: Calculate alpha_s *)
q2AlphaS = 100.0;
resultAlphaS = alphasQ2[pdfIndex, q2AlphaS];
Print["alpha_s for q2=", q2AlphaS, " is ", resultAlphaS];

(* Step 5: Clear the loaded PDFs *)
clearPDFs[];
Print["Cleared loaded PDFs."];
```
