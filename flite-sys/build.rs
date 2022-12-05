extern crate bindgen;

use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=LIBRARY_PATH");

    // shared library.
    println!("cargo:rustc-link-lib=flite_cmu_us_kal");
    println!("cargo:rustc-link-lib=flite_cmu_us_kal16");
    println!("cargo:rustc-link-lib=flite_cmu_us_slt");
    println!("cargo:rustc-link-lib=flite_usenglish");
    println!("cargo:rustc-link-lib=flite_cmulex");
    println!("cargo:rustc-link-lib=flite");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // use unsafe u128 so removed them for now.
        .blocklist_type("_Float64x")
        .blocklist_function("strtold")
        .blocklist_function("qfcvt")
        .blocklist_function("qfcvt_r")
        .blocklist_function("qecvt")
        .blocklist_function("qecvt_r")
        .blocklist_function("qgcvt")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src/flite_bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
