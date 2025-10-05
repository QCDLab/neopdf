//! CLI logic for reading PDF set information.

use clap::{Args, Parser, Subcommand};
use ndarray::{s, Axis};
use terminal_size::{terminal_size, Width};

/// Command-line interface for reading PDF set information.
#[derive(Parser, Clone)]
pub struct ReadCli {
    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: ReadCommands,
}

/// Subcommands for reading PDF set information.
#[derive(Subcommand, Clone)]
pub enum ReadCommands {
    /// Print the metadata of a PDF set.
    #[command(name = "metadata")]
    Metadata(MetadataArgs),
    /// Print the number of subgrids in a PDF set.
    #[command(name = "num_subgrids")]
    NumSubgrids(PdfNameArgs),
    /// Print the subgrid info (nucleons, alphas, x, Q2) for a given subgrid index.
    #[command(name = "subgrid-info")]
    SubgridInfo(SubgridInfoArgs),
    /// Print the contents of a subgrid.
    #[command(name = "subgrid")]
    Subgrid(SubgridArgs),
    /// Print the git version of the code that generated the PDF.
    #[command(name = "git-version")]
    GitVersion(PdfNameArgs),
}

/// Arguments for the metadata subcommand.
#[derive(Args, Clone)]
pub struct MetadataArgs {
    /// Name of the PDF set (LHAPDF or `NeoPDF` file)
    pub pdf_name: String,
}

/// Arguments for commands that only require a PDF name.
#[derive(Args, Clone)]
pub struct PdfNameArgs {
    /// Name of the PDF set (LHAPDF or `NeoPDF` file)
    pub pdf_name: String,
}

/// Arguments for the `subgrid-info` subcommand.
#[derive(Args, Clone)]
pub struct SubgridInfoArgs {
    /// Name of the PDF set (LHAPDF or `NeoPDF` file)
    pub pdf_name: String,
    /// Member index (0-based)
    #[arg(short, long)]
    pub member: usize,
    /// Subgrid index
    #[arg(short, long)]
    pub subgrid_index: usize,
}

/// Arguments for the `subgrid` subcommand.
#[derive(Args, Clone)]
pub struct SubgridArgs {
    /// Name of the PDF set (LHAPDF or `NeoPDF` file)
    pub pdf_name: String,
    /// Member index (0-based)
    #[arg(short, long, default_value_t = 0)]
    pub member: usize,
    /// Subgrid index
    #[arg(short, long)]
    pub subgrid_index: usize,
    /// PDG flavor ID
    #[arg(short = 'i', long)]
    pub pid: i32,
    /// Nucleon index
    #[arg(long, default_value_t = 0)]
    pub nucleon_index: usize,
    /// `Alpha_s` index
    #[arg(long, default_value_t = 0)]
    pub alphas_index: usize,
    /// kT index
    #[arg(long, default_value_t = 0)]
    pub kt_index: usize,
}

/// Entry point for the read CLI.
///
/// # Panics
///
/// This function panics when a PID not present in the Grid is requested.
#[allow(clippy::needless_pass_by_value)]
pub fn main(cli: ReadCli) {
    match &cli.command {
        ReadCommands::Metadata(args) => {
            let pdf = neopdf::pdf::PDF::load(&args.pdf_name, 0);
            println!("{}", pdf.metadata());
        }
        ReadCommands::NumSubgrids(args) => {
            let pdf = neopdf::pdf::PDF::load(&args.pdf_name, 0);
            println!("{}", pdf.num_subgrids());
        }
        ReadCommands::SubgridInfo(args) => {
            let pdf = neopdf::pdf::PDF::load(&args.pdf_name, args.member);
            let subgrid = pdf.subgrid(args.subgrid_index);
            println!("Nucleon Numbers A: {:?}", subgrid.nucleons);
            println!("Alphas values: {:?}", subgrid.alphas);
            println!("kT values: {:?}", subgrid.kts);
            println!("x values: {:?}", subgrid.xs);
            println!("Q2 values: {:?}", subgrid.q2s);
        }
        ReadCommands::Subgrid(args) => {
            let pdf = neopdf::pdf::PDF::load(&args.pdf_name, args.member);
            let subgrid = pdf.subgrid(args.subgrid_index);
            let pids = pdf.pids();
            let pid_to_find = if args.pid == 0 { 21 } else { args.pid };
            let pid_idx = pids
                .iter()
                .position(|&p| p == pid_to_find)
                .unwrap_or_else(|| panic!("PID {} not found in the grid.", args.pid));

            if subgrid.nucleons.len() > 1 {
                println!(
                    "Displaying grid for Nucleon A = {}",
                    subgrid.nucleons[args.nucleon_index]
                );
            }
            if subgrid.alphas.len() > 1 {
                println!(
                    "Displaying grid for alpha_s = {}",
                    subgrid.alphas[args.alphas_index]
                );
            }
            if subgrid.kts.len() > 1 {
                println!("Displaying grid for kT = {}", subgrid.kts[args.kt_index]);
            }
            println!();

            let grid_slice = subgrid.grid.slice(s![
                args.nucleon_index,
                args.alphas_index,
                pid_idx,
                args.kt_index,
                ..,
                ..
            ]);

            let width = if let Some((Width(w), _)) = terminal_size() {
                w as usize
            } else {
                80 // default terminal width
            };

            let col_width = 13;
            let first_col_width = 13;
            let num_cols = (width.saturating_sub(first_col_width)) / col_width;

            if num_cols == 0 {
                println!("Terminal too narrow to display the table.");
                return;
            }

            let q2_chunks = subgrid.q2s.as_slice().unwrap().chunks(num_cols);
            let grid_chunks = grid_slice.axis_chunks_iter(Axis(1), num_cols);

            for (q2_chunk, grid_chunk) in q2_chunks.zip(grid_chunks) {
                print!("{:>12}", "[x | Q2]");
                for q2 in q2_chunk {
                    print!("{q2:>12.5e}");
                }
                println!();

                for (ix, x) in subgrid.xs.iter().enumerate() {
                    print!("{x:>12.5e}");
                    for iq2 in 0..q2_chunk.len() {
                        print!("{:>12.5e}", grid_chunk[[ix, iq2]]);
                    }
                    println!();
                }
                println!();
            }
        }
        ReadCommands::GitVersion(args) => {
            let pdf = neopdf::pdf::PDF::load(&args.pdf_name, 0);
            println!("{}", pdf.metadata().git_version);
        }
    }
}
