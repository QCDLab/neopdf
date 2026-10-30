//! Generates the C header of the `capi` feature into `include/neopdf_cuda.h`.

fn main() {
    #[cfg(feature = "capi")]
    {
        println!("cargo:rerun-if-changed=src/capi.rs");
        println!("cargo:rerun-if-changed=src/foreign.rs");
        println!("cargo:rerun-if-changed=cbindgen.toml");
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").expect("set by cargo");
        cbindgen::generate(crate_dir)
            .expect("Unable to generate the C header")
            .write_to_file("include/neopdf_cuda.h");
    }
}
