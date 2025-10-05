#![allow(missing_docs)]

use assert_cmd::Command;
use predicates::str;
use std::io::Write;

const HELP_STR: &str = "Conversion and combination of PDF sets

Usage: neopdf write <COMMAND>

Commands:
  convert         Convert a single LHAPDF set to `NeoPDF` format
  combine-npdfs   Combine multiple nuclear PDFs into a single `NeoPDF` with A dependence
  combine-alphas  Combine multiple PDFs with different `alpha_s` values into a single `NeoPDF`
  metadata        Update the metadata of the `NeoPDF` grid
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
";

const MODIFIED_METADATA: &str = "Set Description: NNPDF4.0 NNLO global fit, alphas(MZ)=0.1180. mem=0 => average on replicas; mem=1-100 => PDF replicas
Set Index: 331100
Number of Members: 101
XMin: 0.000000001
XMax: 1
QMin: 1.65
QMax: 100000
Flavors: [-5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5]
Format: lhagrid1
AlphaS Q Values: [1.65, 1.7874388, 1.9429053, 2.1193749, 2.32041, 2.5502944, 2.8142025, 3.1184122, 3.4705775, 3.8800751, 4.3584516, 4.92, 4.92, 5.5493622, 6.2897452, 7.1650687, 8.2052867, 9.4481248, 10.941378, 12.745972, 14.940062, 17.624572, 20.930715, 25.030298, 30.149928, 36.590777, 44.756282, 55.191298, 68.63794, 86.115921, 109.03923, 139.38725, 179.95815, 234.7482, 309.52544, 412.70732, 556.71861, 760.11795, 1050.9694, 1472.2574, 2090.6996, 3011.2909, 4401.6501, 6533.3918, 9853.5186, 15109.614, 23573.066, 37444.017, 60599.32, 100000.0]
AlphaS Values: [0.33074891, 0.3176246, 0.30507081, 0.29305875, 0.28156114, 0.27055221, 0.26000761, 0.24990438, 0.24022086, 0.23093662, 0.22203241, 0.21377883, 0.21377883, 0.20671584, 0.19986334, 0.19321629, 0.1867697, 0.1805186, 0.17445809, 0.16858329, 0.16288942, 0.15737175, 0.15202563, 0.14684647, 0.14182979, 0.13697116, 0.13226626, 0.12771084, 0.12330074, 0.11903189, 0.11490031, 0.1109021, 0.10703345, 0.10329062, 0.099669997, 0.096168008, 0.092781186, 0.089506143, 0.086339571, 0.083278245, 0.080319017, 0.077458816, 0.074694649, 0.0720236, 0.069442824, 0.066949551, 0.064541082, 0.062214786, 0.059968105, 0.057798546]
Polarized: false
Set Type: TimeLike
Interpolator Type: LogBicubic
Error Type: replicas
Particle: 2212
Flavor Scheme: variable
Order QCD: 2
AlphaS Order QCD: 2
MW: 0
MZ: 91.1876
MUp: 0
MDown: 0
MStrange: 0
MCharm: 1.51
MBottom: 4.92
MTop: 172.5
AlphaS Type: ipol
Number of PDF flavors: 5

";

#[test]
fn help() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args(["write", "--help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}

#[test]
fn convert_lhapdf() {
    let output = assert_fs::NamedTempFile::new("nnpdf40.neopdf.lz4").unwrap();

    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "write",
            "convert",
            "NNPDF40_nnlo_as_01180",
            "--output",
            output.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "compute",
            "xfx_q2",
            output.path().to_str().unwrap(),
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

    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "write",
            "metadata",
            "--path",
            output.path().to_str().unwrap(),
            "--key",
            "SetType",
            "--value",
            "TimeLike",
        ])
        .assert()
        .success();

    Command::cargo_bin("neopdf")
        .unwrap()
        .args(["read", "metadata", output.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(MODIFIED_METADATA);
}

#[test]
fn combine_nuclear_pdfs() {
    let output = assert_fs::NamedTempFile::new("nnnpdf30.neopdf.lz4").unwrap();
    let npdfs_list = [
        "nNNPDF30_nlo_as_0118_p",
        "nNNPDF30_nlo_as_0118_A2_Z1",
        "nNNPDF30_nlo_as_0118_A4_Z2",
        "nNNPDF30_nlo_as_0118_A6_Z3",
        "nNNPDF30_nlo_as_0118_A9_Z4",
    ];
    let npdfs_str = npdfs_list.join("\n");
    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
    write!(temp_file, "{npdfs_str}").unwrap();

    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "write",
            "combine-npdfs",
            "--names-file",
            temp_file.path().to_str().unwrap(),
            "--output",
            output.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "compute",
            "xfx_q2",
            output.path().to_str().unwrap(),
            "--member",
            "10",
            "--pid",
            "21",
            "4",
            "1e-5",
            "50",
        ])
        .assert()
        .success()
        .stdout("63.389564472386645\n");
}

#[test]
fn combine_alphas_pdfs() {
    let output = assert_fs::NamedTempFile::new("nnpdf40-alphas.neopdf.lz4").unwrap();
    let alphas_pdfs = [
        "NNPDF40_nnlo_as_01160",
        "NNPDF40_nnlo_as_01170",
        "NNPDF40_nnlo_as_01175",
        "NNPDF40_nnlo_as_01185",
        "NNPDF40_nnlo_as_01190",
    ];
    let alphas_pdfs_str = alphas_pdfs.join("\n");
    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
    write!(temp_file, "{alphas_pdfs_str}").unwrap();

    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "write",
            "combine-alphas",
            "--names-file",
            temp_file.path().to_str().unwrap(),
            "--output",
            output.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "compute",
            "xfx_q2",
            output.path().to_str().unwrap(),
            "--member",
            "10",
            "--pid",
            "21",
            "0.1175",
            "1e-5",
            "50",
        ])
        .assert()
        .success()
        .stdout("34.352827400641736\n");
}

#[test]
#[cfg(feature = "tmdlib")]
fn convert_tmd() {
    let config_content = r#"
set_name = "MAP22_grids_FF_Km_N3LL"
set_desc = "MAP22 TMDs for K- fragmentation, converted to NeoPDF"
set_index = 42
n_members = 2

# Inner edges for the subgrids. Leave empty for no subgrids.
x_inner_edges = [0.2]
q_inner_edges = [] # Q, not Q2
kt_inner_edges = [1e-2, 1.0]

# Number of points for (subg)grids.
n_x = [5, 5]
n_q = [6]
n_kt = [5, 5, 4]

# Grid axes that are not part of the TMD interpolation
nucleons = [0.0] # dummy value
alphas = [0.118]

# Metadata
pids = [-3, -2, -1, 21, 1, 2, 3] # smaller set for testing
polarised = false
set_type = "TimeLike"
interpolator_type = "LogChebyshev"
error_type = "replicas"
hadron_pid = 321 # Kaon

alphas_qs = [91.1876]
alphas_vals = [0.118]

# Physics Parameters
flavor_scheme = "fixed"
order_qcd = 2
alphas_order_qcd = 2
m_w = 80.352
m_z = 91.1876
m_up = 0.0
m_down = 0.0
m_strange = 0.0
m_charm = 1.51
m_bottom = 4.92
m_top = 172.5
alphas_type = "ipol"
number_flavors = 4
"#;
    let mut config_file = tempfile::NamedTempFile::new().unwrap();
    write!(config_file, "{config_content}").unwrap();

    let output = assert_fs::NamedTempFile::new("map22.neopdf.lz4").unwrap();

    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "write",
            "convert-tmd",
            "--input",
            config_file.path().to_str().unwrap(),
            "--output",
            output.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    Command::cargo_bin("neopdf")
        .unwrap()
        .args(["read", "metadata", output.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicates::str::contains("MAP22 TMDs"));
}
