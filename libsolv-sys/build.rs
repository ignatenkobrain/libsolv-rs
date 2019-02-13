extern crate bindgen;
extern crate cc;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;


fn main() {

    // Compile the static inline functions into an archive
    // and directs Cargo to link it
    cc::Build::new()
        .file("static/bitmap.c")
        .file("static/dirpool.c")
        .file("static/pool.c")
        .file("static/poolarch.c")
        .file("static/queue.c")
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
        .whitelist_type("Solver")
        .whitelist_type("Chksum")
        .whitelist_type("solv.*")
        .whitelist_function("pool.*")
        .whitelist_function("stringpool.*")
        .whitelist_function("transaction.*")
        .whitelist_function("solv.*")
        .whitelist_function("selection.*")
        .whitelist_function("repopagestore.*")
        .whitelist_function("repo.*")
        .whitelist_function("queue.*")
        .whitelist_function("policy.*")
        .whitelist_function("find.*")
        .whitelist_function("dirpool.*")
        .whitelist_function("datamatcher.*")
        .whitelist_function("dataiterator.*")
        .whitelist_function("map.*")
        .whitelist_var("DI.*")
        .whitelist_var("SOLV.*")
        .whitelist_var("REPO.*")
        .whitelist_var("SEARCH.*")
        .whitelist_var("EVRCMP.*")

        // Hide FILE from bindgen's output
        // Otherwise we get the OS's private file implementation
        .blacklist_type("FILE")
        .raw_line("use libc::FILE;")

        .rustified_enum("solv_knownid")

        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
