extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let libnice = pkg_config::Config::new()
        .atleast_version("0.1.0")
        .probe("nice")
        .expect("Failed to find (lib)nice using pkg-config!");

    let bindings = bindgen::Builder::default()
        .header_contents("wrapper.h", "#include <nice/agent.h>")
        .clang_args(
            libnice
                .include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy())),
        )
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
