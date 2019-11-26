#[cfg(feature="gen_header")]
extern crate cbindgen;

#[cfg(feature="gen_header")]
fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(crate_dir)
        .expect("Unable to generate C bindings.")
        .write_to_file("./bls-embedded.h");
}

#[cfg(not(feature="gen_header"))]
fn main() {
}
