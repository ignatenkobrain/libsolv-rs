extern crate bindgen;
extern crate gcc;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;


fn main() {


    println!("cargo:rustc-link-lib=solv");
    println!("cargo:rustc-link-lib=solvext");


    gcc::compile_library("libsolv-static-functions.a", &["static-functions.c"]);

    //pkg_config::probe_library("libsolv").unwrap();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .ctypes_prefix("libc")
        .whitelisted_type("Solver")
        .whitelisted_function("pool.*")
        .whitelisted_function("stringpool.*")
        .whitelisted_function("transaction.*")
        .whitelisted_function("solv.*")
        .whitelisted_function("selection.*")
        .whitelisted_function("repopagestore.*")
        .whitelisted_function("repo.*")
        .whitelisted_function("queue.*")
        .whitelisted_function("policy.*")
        .whitelisted_function("find.*")
        .whitelisted_function("dirpool.*")
        .whitelisted_function("datamatcher.*")
        .whitelisted_function("dataiterator.*")
        .whitelisted_function("testcase.*")
        .whitelisted_var("SOLVER.*")
        .whitelisted_var("SEARCH.*")
        .whitelisted_var("EVRCMP.*")
        .whitelisted_var("TESTCASE.*")

        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}