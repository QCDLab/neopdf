//! CLI logic for evaluating PDF values (xfxQ2) and (alphasQ2) for a given set, member, and kinematics.
//!
//! This module provides subcommands for evaluating PDF values and `alpha_s` at specified kinematic points.

use clap::{Args, Parser, Subcommand};
use std::process;

/// Command-line interface for PDF and `alpha_s` evaluation.
#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct PdfCli {
    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: PdfCommands,
}

/// Subcommands for PDF and `alpha_s` evaluation.
#[derive(Subcommand, Clone)]
pub enum PdfCommands {
    /// Evaluate xf(x, Q2, pid, ...) for a given set, member, and input values.
    #[command(name = "xfx_q2")]
    XfxQ2(XfxQ2Args),
    /// Evaluate `alphasQ2` for a given set, member, and Q2 value.
    #[command(name = "alphas_q2")]
    AlphasQ2(AlphasQ2Args),
    /// Evaluate TMD PDF for a given set, member, and input values.
    #[cfg(feature = "tmdlib")]
    #[command(name = "xfx_q2_kt")]
    XfxQ2Kt(XfxQ2KtArgs),
}

/// Arguments for the xfxQ2 subcommand.
#[derive(Args, Clone)]
pub struct XfxQ2Args {
    /// Name of the PDF set (`LHAPDF` or `NeoPDF` file)
    pub pdf_name: String,
    /// Member index (0-based)
    #[arg(short, long)]
    pub member: usize,
    /// PDG flavor ID
    #[arg(short = 'i', long)]
    pub pid: i32,
    /// Input values (e.g. x Q2 or A x Q2, ...)
    #[arg(required = true)]
    pub inputs: Vec<f64>,
}

/// Arguments for the `alphas_q2` subcommand.
#[derive(Args, Clone)]
pub struct AlphasQ2Args {
    /// Name of the PDF set (LHAPDF or `NeoPDF` file)
    pub pdf_name: String,
    /// Member index (0-based)
    #[arg(short, long)]
    pub member: usize,
    /// Q^2 value
    #[arg(short, long)]
    pub q2: f64,
}

/// Arguments for the `xfxQ2_kt` subcommand.
#[cfg(feature = "tmdlib")]
#[derive(Args, Clone)]
pub struct XfxQ2KtArgs {
    /// Name of the TMD PDF set
    pub pdf_name: String,
    /// Member index (0-based)
    #[arg(short, long)]
    pub member: usize,
    /// PDG flavor ID
    #[arg(short = 'i', long)]
    pub pid: i32,
    /// Input values (kt, x, Q2)
    #[arg(required = true)]
    pub inputs: Vec<f64>,
}

/// Entry point for the pdf CLI.
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn main(cli: PdfCli) {
    match &cli.command {
        PdfCommands::XfxQ2(args) => {
            let pdf = neopdf::pdf::PDF::load(&args.pdf_name, args.member);
            if args.inputs.len() < 2 {
                eprintln!("Error: At least [x, Q2] must be provided as input.");
                process::exit(1);
            }
            let val = pdf.xfxq2(args.pid, &args.inputs);
            println!("{val}");
        }
        PdfCommands::AlphasQ2(args) => {
            let pdf = neopdf::pdf::PDF::load(&args.pdf_name, args.member);
            let val = pdf.alphas_q2(args.q2);
            println!("{val}");
        }
        #[cfg(feature = "tmdlib")]
        PdfCommands::XfxQ2Kt(args) => {
            use neopdf_tmdlib::Tmd;
            let mut tmd = Tmd::new();
            tmd.init(&args.pdf_name, args.member as i32);

            if args.inputs.len() < 3 {
                eprintln!("Error: [kt, x, Q2] must be provided as input.");
                process::exit(1);
            }
            let kt = args.inputs[0];
            let x = args.inputs[1];
            let q2 = args.inputs[2];

            // The list of PIDs returned by TMDlib
            let pids = [-6, -5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5, 6];
            let pid_idx = pids.iter().position(|&p| p == args.pid).unwrap_or_else(|| {
                eprintln!("Error: PID {} not supported by TMDlib interface.", args.pid);
                process::exit(1);
            });

            let pdfs = tmd.xfxq2kt(x, kt, q2.sqrt());
            println!("{}", pdfs[pid_idx]);
        }
    }
}
