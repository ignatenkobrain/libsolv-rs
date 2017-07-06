extern crate bindgen;
extern crate gcc;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;


fn main() {
/*    gcc::Config::new()
        .file("static/queue.c")
        .file("static/bitmap.c")
        .file("static/dirpool.c")
        .file("static/pool.c")
        .file("static/poolarch.c")
        .file("static/repo.c")
        .file("static/repodata.c")
        .file("static/strpool.c")
        .compile("libsolv-static-functions.a");*/

    println!("cargo:rustc-link-lib=solvext");


    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")

        // Finish the builder and generate the bindings.
        .ctypes_prefix("libc")

        // <solv/testcase.h>
        .whitelisted_var("TESTCASE.*")
        .whitelisted_function("testcase.*")

        // <solv/solv_xfopen.h>
        .whitelisted_function("solv_xfopen.*")

        // <solv/pool_fileconflicts.h>
        .whitelisted_var("FINDFILECONFLICTS.*")
        .whitelisted_function("pool_findfileconflicts")

        // <solv/repo_rpmdb.h>
        // TODO: VARS
        .whitelisted_function("repo_add_rpm.*")
        .whitelisted_function("rpm_state_.*")
        .whitelisted_function("rpm_installedrpmdbids")
        .whitelisted_function("rpm_by.*")
        .whitelisted_function("rpm_query.*")
        .whitelisted_function("rpm_iterate_filelist")

        // <solv/repo_repomdxml.h>
        .whitelisted_function("repo_add_repo.*")
        // <solv/repo_rpmmd.h>
        .whitelisted_function("repo_add_rpm.*")
        // <solv/repo_deltainfoxml.h>
        .whitelisted_function("repo_add_delta.*")
        // <solv/repo_updateinfoxml.h>
        .whitelisted_function("repo_add_update.*")

        // As defined by libsolv-sys
        .hide_type("Chksum")
        .hide_type("DUChanges")
        .hide_type("Dataiterator")
        .hide_type("Datamatcher")
        .hide_type("_Datapos")
        .hide_type("Datapos")
        .hide_type("_Dirpool")
        .hide_type("Dirpool")
        .hide_type("Hashtable")
        .hide_type("Hashval")
        .hide_type("Id")
        .hide_type("KeyValue")
        .hide_type("_Map")
        .hide_type("Map")
        .hide_type("Offset")
        .hide_type("_Pool")
        .hide_type("Pool")
        .hide_type("_Queue")
        .hide_type("Queue")
        .hide_type("_Reldep")
        .hide_type("Reldep")
        .hide_type("_Repo")
        .hide_type("Repo")
        .hide_type("_Repodata")
        .hide_type("Repodata")
        .hide_type("_Repokey")
        .hide_type("Repokey")
        .hide_type("Rule")
        .hide_type("_Solvable")
        .hide_type("Solvable")
        .hide_type("_Solver")
        .hide_type("Solver")
        .hide_type("_Stringpool")
        .hide_type("Stringpool")
        .hide_type("Transaction")
        // As defined by libc
        .hide_type("FILE")
        .raw_line("use libc::FILE;")
        .raw_line("use libsolv_sys::{Chksum, DUChanges, Dataiterator, Datamatcher, Datapos, Dirpool};")
        .raw_line("use libsolv_sys::{Hashtable, Hashval, Id, KeyValue, Map, Offset, Pool, Queue};")
        .raw_line("use libsolv_sys::{Reldep, _Repo, Repo, Repodata, Repokey, Rule, Solvable, Solver, Stringpool};")
        .raw_line("use libsolv_sys::{Transaction};")

        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
