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
        .header_contents(
            "wrapper.h",
            "#include <nice/agent.h>
             #include <nice/interfaces.h>

             #include <stun/stunagent.h>
             #include <stun/stunmessage.h>
             #include <stun/constants.h>
             #include <stun/usages/bind.h>
             #include <stun/usages/ice.h>
             #include <stun/usages/turn.h>
             #include <stun/usages/timer.h>

             #include <nice/pseudotcp.h>",
        )
        // ICE Library
        .whitelist_function("nice_.+")
        .whitelist_type("NICE.+")
        .whitelist_type("_?Nice.+")
        .whitelist_type("_?TurnServer")
        // STUN Library
        .whitelist_function("stun_.+")
        .whitelist_type("STUN.+")
        .whitelist_type("TURN.+")
        .whitelist_type("_?[Ss]tun.+")
        // contains `va_list` type argument which seems like it might not be handled properly
        .opaque_type("StunDebugHandler")
        // Pseudo TCP Socket implementation
        .whitelist_function("pseudo_tcp_.+")
        .whitelist_type("_?PseudoTcp.+")
        // Disable recursive whitelisting, we're using libc, glib-sys, etc.
        .whitelist_recursively(false)
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
