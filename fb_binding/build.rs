use objectbox::generator as gen;
use std::env;
use std::path::PathBuf;

fn main() {
    // Path to ObjectBox library - can be overridden via OBX_LIB_PATH environment variable
    // First try project-local lib directory, then fallback to Downloads
    let cargo_manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let project_root = cargo_manifest_dir.parent().unwrap();
    let local_lib_path = project_root.join("lib");
    
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
    
    // For macOS, set DYLD_FALLBACK_LIBRARY_PATH for build scripts
    #[cfg(target_os = "macos")]
    {
        let dyld_path = format!("{}:{}", lib_path, env::var("DYLD_FALLBACK_LIBRARY_PATH").unwrap_or_default());
        println!("cargo:rustc-env=DYLD_FALLBACK_LIBRARY_PATH={}", dyld_path);
    }
    
    let cargo_manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    gen::generate_assets(&out_dir, &cargo_manifest_dir);
}
