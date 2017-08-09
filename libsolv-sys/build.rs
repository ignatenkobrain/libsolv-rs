extern crate bindgen;
extern crate gcc;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;


fn main() {

    // Compile the static inline functions into an archive
    // and directs Cargo to link it
    gcc::Config::new()
        .file("static/queue.c")
        .file("static/bitmap.c")
        .file("static/dirpool.c")
        .file("static/pool.c")
        .file("static/poolarch.c")
        .file("static/repo.c")
        .file("static/repodata.c")
        .file("static/strpool.c")
        .compile("libsolv-static-functions.a");

    //pkg_config::probe_library("libsolvext").unwrap();

    // Direct Cargo to link the libsolv library
    pkg_config::probe_library("libsolv").unwrap();

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")

        // Prefer the libc crate's definitions for libc types
        .ctypes_prefix("libc")

        // Whitelist libsolv's functions, types, and variables,
        // otherwise bindgen will bind all of libc
        .whitelisted_type("Solver")
        .whitelisted_type("Chksum")
        .whitelisted_type("solv.*")
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
        .whitelisted_function("map.*")
        .whitelisted_var("DI.*")
        .whitelisted_var("SOLV.*")
        .whitelisted_var("REPO.*")
        .whitelisted_var("SEARCH.*")
        .whitelisted_var("EVRCMP.*")

        // Hide FILE from bindgen's output
        // Otherwise we get the OS's private file implementation
        .hide_type("FILE")
        .raw_line("use libc::FILE;")

        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
