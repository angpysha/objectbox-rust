extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Path to ObjectBox library - can be overridden via OBX_LIB_PATH environment variable
    // First try project-local lib directory, then fallback to Downloads
    let cargo_manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let local_lib_path = cargo_manifest_dir.join("lib");
    
    let lib_path = env::var("OBX_LIB_PATH")
        .unwrap_or_else(|_| {
            if local_lib_path.exists() && local_lib_path.join("libobjectbox.dylib").exists() {
                local_lib_path.to_string_lossy().to_string()
            } else {
                "/Users/andrii/Downloads/objectbox-macos-universal (1)/lib".to_string()
            }
        });
    
    // Tell cargo to tell rustc where to find the objectbox shared library.
    println!("cargo:rustc-link-search=native={}", lib_path);
    
    // Tell cargo to tell rustc to link the objectbox shared library.
    println!("cargo:rustc-link-lib=dylib=objectbox");

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
