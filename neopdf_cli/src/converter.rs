//! CLI logic for `NeoPDF` conversion utilities.
//!
//! This module defines the command-line interface for converting LHAPDF sets to `NeoPDF` format
//! and for combining multiple nuclear PDFs into a single `NeoPDF` file with explicit A dependence.

use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{BufRead, BufReader};

use neopdf::converter;
use neopdf::metadata::{InterpolatorType, SetType};

/// Command-line interface for `NeoPDF` conversion utilities.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// TODO
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands for the `NeoPDF` CLI.
#[derive(Subcommand)]
pub enum Commands {
    /// Convert a single LHAPDF set to `NeoPDF` format.
    Convert {
        /// Name of the LHAPDF set (e.g. `NNPDF40_nnlo_as_01180`)
        pdf_name: String,
        /// Output path for the `NeoPDF` file.
        #[arg(short, long)]
        output: String,
    },
    /// Combine multiple nuclear PDFs into a single `NeoPDF` with A dependence.
    CombineNpdfs {
        /// List of PDF set names (each with a different A).
        #[arg(
            short = 'n',
            long = "pdf-names",
            required_unless_present = "names_file"
        )]
        pdf_names: Option<Vec<String>>,
        /// Path to a file containing PDF set names, one per line.
        #[arg(short = 'f', long = "names-file", conflicts_with = "pdf_names")]
        names_file: Option<String>,
        /// Output path for the combined `NeoPDF` file.
        #[arg(short, long)]
        output: String,
    },
    /// Combine multiple PDFs with different `alpha_s` values into a single `NeoPDF`.
    CombineAlphas {
        /// List of PDF set names (each with a different `alpha_s`).
        #[arg(
            short = 'n',
            long = "pdf-names",
            required_unless_present = "names_file"
        )]
        pdf_names: Option<Vec<String>>,
        /// Path to a file containing PDF set names, one per line.
        #[arg(short = 'f', long = "names-file", conflicts_with = "pdf_names")]
        names_file: Option<String>,
        /// Output path for the combined `NeoPDF` file.
        #[arg(short, long)]
        output: String,
    },
    /// Convert a TMD set to `NeoPDF` format.
    #[cfg(feature = "tmdlib")]
    ConvertTmd {
        /// Path to the input configuration file.
        #[arg(short, long)]
        input: String,
        /// Output path for the `NeoPDF` file.
        #[arg(short, long)]
        output: String,
    },
    /// Update the metadata of the `NeoPDF` grid
    ///
    /// Here are the metadata keys that can be updated:
    ///
    ///  - `SetDesc`: description of the set
    ///  - `SetIndex`: set index
    ///  - `NumMembers`: number of members
    ///  - `XMin`: minimum value of x
    ///  - `XMax`: maximum value of x
    ///  - `QMin`: minimum value of Q2
    ///  - `QMax`: maximum value of Q2
    ///  - `Format`: format of the set (can be any string)
    ///  - `Polarized`: polarization, takes a boolean
    ///  - `InterpolatorType`: interpolation method (bilinear, logbilinear, logbicubic, logtricubic, chebyshev, interpndlinear)
    ///  - `ErrorType`: set error (replica, hessian, etc.)
    ///  - `Particle`: Hadron PID
    ///  - `FlavorScheme`: flavour scheme (fixed, variable, etc.)
    ///  - `OrderQCD`: perturbative order
    ///  - `AlphaS_OrderQCD`: perturbative order of the strong coupling
    ///  - `MW`: W boson mass
    ///  - `MZ`: Z boson mass
    ///  - `MUp`: mass of the up quark
    ///  - `MDown`: mass of the down quark
    ///  - `MStrange`: mass of the strange quark
    ///  - `MCharm`: mass of the charm quark
    ///  - `MBottom`: mass of the bottom quark
    ///  - `MTop`: mass of the top quark
    ///  - `AlphaS_Type`: type of the `AlphaS` interpolation (ipol, etc.)
    ///  - `NumFlavors`: number of active quark flavours
    ///  - `SetType`: type of the set (spacelike, timelike)
    #[command(verbatim_doc_comment)]
    Metadata {
        /// Path to the `NeoPDF` file.
        #[arg(short, long)]
        path: String,
        /// Key of the metadata to update.
        #[arg(short, long)]
        key: String,
        /// Value to set for the metadata key.
        #[arg(short, long)]
        value: String,
    },
}

// Loads PDF names from either command line arguments or a file.
///
/// This function handles the mutually exclusive options for providing PDF names:
/// either directly via command line arguments or from a file containing one name per line.
/// Exactly one of the two options must be provided.
///
/// # Arguments
///
/// * `pdf_names` - Optional slice of PDF names provided directly via command line
/// * `names_file` - Optional path to a file containing PDF names (one per line)
///
/// # Returns
///
/// * `Ok(Vec<String>)` - Vector of PDF names loaded successfully
/// * `Err(Box<dyn std::error::Error>)` - If:
///   - Both options are provided or both are None
///   - File cannot be opened or read
///   - I/O error occurs while reading the file
fn load_pdf_names(
    pdf_names: Option<&[String]>,
    names_file: Option<&str>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    match (pdf_names, names_file) {
        (Some(names), None) => Ok(names.to_vec()),
        (None, Some(file_path)) => {
            let file = File::open(file_path)?;
            BufReader::new(file)
                .lines()
                .collect::<Result<Vec<_>, _>>()
                .map_err(Into::into)
        }
        _ => Err("Either --pdf-names or --names-file must be provided.".into()),
    }
}

/// Executes the CLI command based on the parsed arguments.
///
/// This function handles the main application logic for both Convert and Combine commands.
/// It delegates to the appropriate converter functions and propagates any errors that occur.
///
/// # Arguments
///
/// * `cli` - The parsed command line interface structure containing the command and its arguments
///
/// # Returns
///
/// * `Ok(())` - If the command executed successfully
/// * `Err(Box<dyn std::error::Error>)` - If any error occurred during execution
///
/// # Errors
///
/// TODO
#[allow(clippy::needless_pass_by_value)]
pub fn run_cli(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match &cli.command {
        Commands::Convert { pdf_name, output } => {
            converter::convert_lhapdf(pdf_name, output)?;
        }
        Commands::CombineNpdfs {
            pdf_names,
            names_file,
            output,
        } => {
            let names = load_pdf_names(pdf_names.as_deref(), names_file.as_deref())?;
            let names_str: Vec<&str> = names.iter().map(String::as_str).collect();
            converter::combine_lhapdf_npdfs(&names_str, output)?;
        }
        Commands::CombineAlphas {
            pdf_names,
            names_file,
            output,
        } => {
            let names = load_pdf_names(pdf_names.as_deref(), names_file.as_deref())?;
            let names_str: Vec<&str> = names.iter().map(String::as_str).collect();
            converter::combine_lhapdf_alphas(&names_str, output)?;
        }
        #[cfg(feature = "tmdlib")]
        Commands::ConvertTmd { input, output } => {
            crate::tmd_converter::convert_tmd(input, output)?;
        }
        Commands::Metadata { path, key, value } => {
            let grids_with_metadata = neopdf::writer::GridArrayCollection::decompress(path)?;
            let mut metadata = grids_with_metadata[0].metadata.as_ref().clone();
            match key.as_str() {
                "SetDesc" => metadata.set_desc = value.to_string(),
                "SetIndex" => metadata.set_index = value.parse()?,
                "NumMembers" => metadata.num_members = value.parse()?,
                "XMin" => metadata.x_min = value.parse()?,
                "XMax" => metadata.x_max = value.parse()?,
                "QMin" => metadata.q_min = value.parse()?,
                "QMax" => metadata.q_max = value.parse()?,
                "Format" => metadata.format = value.to_string(),
                "Polarized" => metadata.polarised = value.parse()?,
                "InterpolatorType" => {
                    metadata.interpolator_type = match value.to_lowercase().as_str() {
                        "bilinear" => InterpolatorType::Bilinear,
                        "logbilinear" => InterpolatorType::LogBilinear,
                        "logbicubic" => InterpolatorType::LogBicubic,
                        "logtricubic" => InterpolatorType::LogTricubic,
                        "interpndlinear" => InterpolatorType::InterpNDLinear,
                        "chebyshev" => InterpolatorType::LogChebyshev,
                        _ => return Err(format!("Unknown InterpolatorType: {value}").into()),
                    }
                }
                "ErrorType" => metadata.error_type = value.to_string(),
                "Particle" => metadata.hadron_pid = value.parse()?,
                "GitVersion" => metadata.git_version = value.to_string(),
                "CodeVersion" => metadata.code_version = value.to_string(),
                "FlavorScheme" => metadata.flavor_scheme = value.to_string(),
                "OrderQCD" => metadata.order_qcd = value.parse()?,
                "AlphaS_OrderQCD" => metadata.alphas_order_qcd = value.parse()?,
                "MW" => metadata.m_w = value.parse()?,
                "MZ" => metadata.m_z = value.parse()?,
                "MUp" => metadata.m_up = value.parse()?,
                "MDown" => metadata.m_down = value.parse()?,
                "MStrange" => metadata.m_strange = value.parse()?,
                "MCharm" => metadata.m_charm = value.parse()?,
                "MBottom" => metadata.m_bottom = value.parse()?,
                "MTop" => metadata.m_top = value.parse()?,
                "AlphaS_Type" => metadata.alphas_type = value.to_string(),
                "NumFlavors" => metadata.number_flavors = value.parse()?,
                "SetType" => {
                    metadata.set_type = match value.to_lowercase().as_str() {
                        "spacelike" => SetType::SpaceLike,
                        "timelike" => SetType::TimeLike,
                        _ => return Err(format!("Unknown SetType: {value}").into()),
                    }
                }
                _ => return Err(format!("Unknown metadata key: {key}").into()),
            }
            let grids_data: Vec<_> = grids_with_metadata.iter().map(|g| &g.grid).collect();
            neopdf::writer::GridArrayCollection::compress(&grids_data, &metadata, path)?;
        }
    }
    Ok(())
}

/// Entry point for the `NeoPDF` CLI.
///
/// Parses command-line arguments and dispatches to the appropriate subcommand handler.
pub fn main(cli: Cli) {
    if let Err(err) = run_cli(cli) {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
