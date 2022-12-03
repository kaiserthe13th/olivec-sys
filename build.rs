use bindgen;
use cc;

use std::path::PathBuf;
use std::env;

const FILE: &str = "src/olivec.c";

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=olive-c/olive.c");
    println!("cargo:rustc-search-lib=static={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=olivec");
    
    cc::Build::new()
        .flag("-std=c99")
        .warnings(false)
        .extra_warnings(false)
        .file(FILE)
        .compile("olivec");
    
    let bindings = bindgen::builder()
        .whitelist_function("olivec_.*")
        .whitelist_type("Olivec_.*")
        .whitelist_var("olivec_.*")
        .header(FILE)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}