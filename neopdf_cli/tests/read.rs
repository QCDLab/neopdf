#![allow(missing_docs)]

use assert_cmd::Command;
use predicates::str;

const HELP_STR: &str = "Commands for reading PDF set information

Usage: neopdf read <COMMAND>

Commands:
  metadata      Print the metadata of a PDF set
  num_subgrids  Print the number of subgrids in a PDF set
  subgrid-info  Print the subgrid info (nucleons, alphas, x, Q2) for a given subgrid index
  subgrid       Print the contents of a subgrid
  git-version   Print the git version of the code that generated the PDF
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
";

const METADATA: &str = "Set Description: NNPDF4.0 NNLO global fit, alphas(MZ)=0.1180. mem=0 => average on replicas; mem=1-100 => PDF replicas
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
Set Type: SpaceLike
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

const SUBGRID_INFO_LHAPDF: &str = "Nucleon Numbers A: [0.0], shape=[1], strides=[1], layout=CFcf (0xf), const ndim=1
Alphas values: [0.0], shape=[1], strides=[1], layout=CFcf (0xf), const ndim=1
kT values: [0.0], shape=[1], strides=[1], layout=CFcf (0xf), const ndim=1
x values: [1e-9, 1.2970848e-9, 1.682429e-9, 2.1822532e-9, 2.8305674e-9, 3.671486e-9, 4.7622286e-9, 6.1770143e-9, 8.0121111e-9, 1.0392387e-8, 1.3479806e-8, 1.748445e-8, 2.2678812e-8, 2.9416337e-8, 3.8155475e-8, 4.9490871e-8, 6.419383e-8, 8.3264795e-8, 1.0800142e-7, 1.4008687e-7, 1.8170433e-7, 2.3568555e-7, 3.0570351e-7, 3.9652231e-7, 5.1432126e-7, 6.6711525e-7, 8.6529992e-7, 1.1223588e-6, 1.45578e-6, 1.8882456e-6, 2.4491735e-6, 3.1767165e-6, 4.1203542e-6, 5.3442527e-6, 6.931619e-6, 8.9903426e-6, 1.1660303e-5, 1.5122831e-5, 1.9612953e-5, 2.5435221e-5, 3.2984168e-5, 4.2770705e-5, 5.5456125e-5, 7.1895831e-5, 9.3195423e-5, 0.00012078237, 0.00015649721, 0.00020270894, 0.0002624598, 0.00033964524, 0.00043923444, 0.00056753566, 0.00073250762, 0.00094411211, 0.0012146932, 0.0015593531, 0.0019962745, 0.0025469149, 0.0032359751, 0.0040910344, 0.0051417598, 0.006418651, 0.0079513794, 0.0097669, 0.011887614, 0.014329895, 0.017103228, 0.020210073, 0.023646397, 0.027402692, 0.031465251, 0.035817483, 0.040441106, 0.045317134, 0.050426635, 0.055751261, 0.061273602, 0.066977383, 0.072847559, 0.078870332, 0.08503312, 0.091324491, 0.097734088, 0.10425254, 0.11087137, 0.11758291, 0.12438023, 0.13125706, 0.13820771, 0.14522701, 0.15231026, 0.15945321, 0.16665195, 0.17390294, 0.18120291, 0.18854889, 0.19593815, 0.20336816, 0.21083662, 0.21834138, 0.22588049, 0.2334521, 0.24105454, 0.24868622, 0.2563457, 0.26403161, 0.2717427, 0.27947777, 0.28723573, 0.29501555, 0.30281625, 0.31063694, 0.31847676, 0.32633492, 0.33421065, 0.34210326, 0.35001207, 0.35793645, 0.36587581, 0.37382958, 0.38179724, 0.38977827, 0.3977722, 0.40577857, 0.41379696, 0.42182694, 0.42986814, 0.43792018, 0.44598271, 0.45405538, 0.46213789, 0.47022992, 0.47833118, 0.48644138, 0.49456027, 0.50268759, 0.51082309, 0.51896653, 0.52711769, 0.53527636, 0.54344232, 0.55161538, 0.55979535, 0.56798204, 0.57617528, 0.5843749, 0.59258073, 0.60079262, 0.60901041, 0.61723396, 0.62546313, 0.63369778, 0.64193778, 0.65018301, 0.65843334, 0.66668866, 0.67494884, 0.68321379, 0.69148339, 0.69975754, 0.70803614, 0.7163191, 0.72460632, 0.73289771, 0.74119318, 0.74949265, 0.75779603, 0.76610325, 0.77441423, 0.78272889, 0.79104716, 0.79936897, 0.80769425, 0.81602293, 0.82435495, 0.83269024, 0.84102875, 0.84937041, 0.85771516, 0.86606296, 0.87441373, 0.88276744, 0.89112402, 0.89948343, 0.90784562, 0.91621053, 0.92457813, 0.93294836, 0.94132119, 0.94969656, 0.95807444, 0.96645478, 0.97483755, 0.9832227, 0.9916102, 1.0], shape=[196], strides=[1], layout=CFcf (0xf), const ndim=1
Q2 values: [2.7224999999999997, 3.19493746374544, 3.77488100476809, 4.491749966750009, 5.384302568099999, 6.50400152667136, 7.91973571100625, 9.72449464910884, 12.044908183506251, 15.05498278164001, 18.996100349542562, 24.2064], shape=[12], strides=[1], layout=CFcf (0xf), const ndim=1
";

#[test]
fn help() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args(["read", "--help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}

#[test]
fn read_metadata_lhapdf() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args(["read", "metadata", "NNPDF40_nnlo_as_01180"])
        .assert()
        .success()
        .stdout(METADATA);
}

#[test]
fn read_metadata_neopdf() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args(["read", "metadata", "NNPDF40_nnlo_as_01180.neopdf.lz4"])
        .assert()
        .success()
        .stdout(METADATA);
}

#[test]
fn read_gitversion_neopdf() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args(["read", "git-version", "NNPDF40_nnlo_as_01180.neopdf.lz4"])
        .assert()
        .success()
        .stdout("v0.2.0-alpha1-22-gfb6af13-dirty\n");
}

#[test]
fn read_num_subgrids_lhapdf() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args(["read", "num_subgrids", "NNPDF40_nnlo_as_01180"])
        .assert()
        .success()
        .stdout("2\n");
}

#[test]
fn read_num_subgrids_neopdf() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args(["read", "num_subgrids", "NNPDF40_nnlo_as_01180.neopdf.lz4"])
        .assert()
        .success()
        .stdout("2\n");
}

#[test]
fn read_num_subgrid_info_lhapdf() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "read",
            "subgrid-info",
            "NNPDF40_nnlo_as_01180",
            "--member",
            "0",
            "--subgrid-index",
            "0",
        ])
        .assert()
        .success()
        .stdout(SUBGRID_INFO_LHAPDF);
}

#[test]
fn read_num_subgrid_info_neopdf() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "read",
            "subgrid-info",
            "NNPDF40_nnlo_as_01180.neopdf.lz4",
            "--member",
            "0",
            "--subgrid-index",
            "0",
        ])
        .assert()
        .success()
        .stdout(SUBGRID_INFO_LHAPDF);
}
