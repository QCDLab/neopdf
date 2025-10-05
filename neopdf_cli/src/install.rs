//! CLI logic for installing PDF sets.

use clap::Parser;

use neopdf::manage::{ManageData, PdfSetFormat};

/// Command-line interface for installing PDF sets.
#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Name of the PDF set to install (e.g. `NNPDF40_nnlo_as_01180`)
    pub pdf_name: String,
}

/// Entry point for the `neopdf install` CLI.
#[allow(clippy::needless_pass_by_value)]
pub fn main(cli: Cli) {
    let manager = if cli.pdf_name.ends_with(".neopdf.lz4") {
        ManageData::new(&cli.pdf_name, PdfSetFormat::Lhapdf)
    } else {
        ManageData::new(&cli.pdf_name, PdfSetFormat::Neopdf)
    };

    println!(
        "PDF set '{}' installed in {}",
        cli.pdf_name,
        manager.set_path().display()
    );
}
