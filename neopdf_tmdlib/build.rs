#![allow(missing_docs)]

use std::process::Command;

fn main() {
    let cxx_flags: Vec<String> = String::from_utf8(
        Command::new("TMDlib-config")
            .arg("--cppflags")
            .output()
            .expect("Could not find `TMDlib-config`, please install TMDlib and make sure `TMDlib-config` is in your PATH")
            .stdout,
    )
    .unwrap()
    .split_whitespace()
    .map(ToOwned::to_owned)
    .collect();

    let include_dirs: Vec<_> = cxx_flags
        .iter()
        .filter_map(|token| token.strip_prefix("-I"))
        .collect();

    let libs = String::from_utf8(
        Command::new("TMDlib-config")
            .arg("--ldflags")
            .output()
            .expect("Could not find `TMDlib-config`, please install TMDlib and make sure `TMDlib-config` is in your PATH")
            .stdout,
    )
    .unwrap();

    for lib_path in libs
        .split_whitespace()
        .filter_map(|token| token.strip_prefix("-L"))
    {
        println!("cargo:rustc-link-search={lib_path}");
    }

    for lib in libs
        .split_whitespace()
        .filter_map(|token| token.strip_prefix("-l"))
    {
        println!("cargo:rustc-link-lib={lib}");
    }

    let mut build = cxx_build::bridge("src/lib.rs");
    build.file("src/tmdlib.cpp").cpp(true);

    for dir in &include_dirs {
        build.include(*dir);
    }

    build.compile("tmd-bridge");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/tmdlib.cpp");
    println!("cargo:rerun-if-changed=src/tmdlib.hpp");

    // Manually link to different libraries as TMDlib need them.
    println!("cargo:rustc-link-lib=gsl");
    println!("cargo:rustc-link-lib=gslcblas");
    println!("cargo:rustc-link-lib=LHAPDF");
}
