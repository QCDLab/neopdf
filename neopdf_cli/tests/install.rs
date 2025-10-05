#![allow(missing_docs)]

use assert_cmd::Command;

const HELP_STR: &str = "Install a PDF set from one of the supported repositories

Usage: neopdf install <PDF_NAME>

Arguments:
  <PDF_NAME>  Name of the PDF set to install (e.g. `NNPDF40_nnlo_as_01180`)

Options:
  -h, --help     Print help
  -V, --version  Print version
";

#[test]
fn help() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args(["install", "--help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}
