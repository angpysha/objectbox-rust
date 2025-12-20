extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the objectbox shared library.
    println!("cargo:rustc-link-lib=objectbox");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("src/objectbox.h")
        // Some settings
        .allowlist_function("obx_.*")
        .allowlist_type("OBX_.*")
        .allowlist_var("OBX_.*")
        // .allowlist_recursively(false)
        .prepend_enum_name(false)
        .rustified_enum("OBX_.*")
        .derive_copy(false)
        .derive_debug(false)
        .derive_default(false)
        .rustfmt_bindings(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate_comments(false) // generated comments breaks rust
        .layout_tests(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the src/c_bindings.rs file.
    let cargo_manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_path = cargo_manifest_dir.join("src/c_bindings.rs");
    bindings
        .write_to_file(target_path.as_path())
        .expect("Couldn't write bindings!");
}
