#![allow(missing_docs)]

use assert_cmd::Command;
use predicates::str;

const HELP_STR: &str = "Evaluate PDF values and `alpha_s` at given kinematics

Usage: neopdf compute <COMMAND>

Commands:
  xfx_q2     Evaluate xf(x, Q2, pid, ...) for a given set, member, and input values
  alphas_q2  Evaluate `alphasQ2` for a given set, member, and Q2 value
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
";

#[test]
fn help() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args(["compute", "--help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}

#[test]
fn xfxq2_lhapdf() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "compute",
            "xfx_q2",
            "NNPDF40_nnlo_as_01180",
            "--member",
            "0",
            "--pid",
            "21",
            "1e-3",
            "10.0",
        ])
        .assert()
        .success()
        .stdout("7.1276606679158565\n");
}

#[test]
fn xfxq2_neopdf() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "compute",
            "xfx_q2",
            "NNPDF40_nnlo_as_01180.neopdf.lz4",
            "--member",
            "0",
            "--pid",
            "21",
            "1e-3",
            "10.0",
        ])
        .assert()
        .success()
        .stdout("7.1276606679158565\n");
}

#[test]
fn xfxq2_neopdf_combined_npdfs() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "compute",
            "xfx_q2",
            "nNNPDF30_nlo_as_0118.neopdf.lz4",
            "--member",
            "0",
            "--pid",
            "21",
            "27",
            "1e-3",
            "10.0",
        ])
        .assert()
        .success()
        .stdout("8.204642526146479\n");
}

#[test]
fn xfxq2_neopdf_combined_npdfs_interpolation() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "compute",
            "xfx_q2",
            "nNNPDF30_nlo_as_0118.neopdf.lz4",
            "--member",
            "0",
            "--pid",
            "21",
            "45",
            "1e-3",
            "10.0",
        ])
        .assert()
        .success()
        .stdout("7.994425939656785\n");
}

#[test]
fn alphasq2_lhapdf() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "compute",
            "alphas_q2",
            "NNPDF40_nnlo_as_01180",
            "--member",
            "0",
            "--q2",
            "10",
        ])
        .assert()
        .success()
        .stdout("0.2485925816007479\n");
}

#[test]
fn alphasq2_neopdf() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "compute",
            "alphas_q2",
            "NNPDF40_nnlo_as_01180.neopdf.lz4",
            "--member",
            "0",
            "--q2",
            "10",
        ])
        .assert()
        .success()
        .stdout("0.2485925816007479\n");
}

#[test]
#[cfg(feature = "tmdlib")]
fn xfxq2_kt_tmdlib() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "compute",
            "xfx_q2_kt",
            "MAP22_grids_FF_Km_N3LL",
            "--member",
            "0",
            "--pid",
            "2",
            "1.0",
            "0.1",
            "10.0",
        ])
        .assert()
        .success()
        .stdout("0.07892252798564643\n");
}

#[test]
#[cfg(feature = "tmdlib")]
fn xfxq2_kt_neopdf_tmdlib() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "compute",
            "xfx_q2",
            "MAP22_grids_FF_Km_N3LL.neopdf.lz4",
            "--member",
            "0",
            "--pid",
            "2",
            "1.0",
            "0.1",
            "10.0",
        ])
        .assert()
        .success()
        .stdout("0.07899136744063368\n");
}
