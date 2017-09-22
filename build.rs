extern crate bindgen;
use bindgen::Builder;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=AutoItX3_x64_DLL");

    let bindings = Builder::default()
        .header("wrapper.hpp")
        .whitelisted_function("AU3_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
