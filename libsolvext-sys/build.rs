extern crate bindgen;
extern crate cc;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;


fn main() {

    //pkg_config::probe_library("libsolvext").unwrap();

    pkg_config::probe_library("libsolvext").unwrap();


    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")

        // Prefer the libc crate's definitions for libc types
        .ctypes_prefix("libc")

        // <solv/testcase.h>
        .whitelist_var("TESTCASE.*")
        .whitelist_function("testcase.*")

        // <solv/solv_xfopen.h>
        .whitelist_function("solv_xfopen.*")

        // <solv/pool_fileconflicts.h>
        .whitelist_var("FINDFILECONFLICTS.*")
        .whitelist_function("pool_findfileconflicts")

        // <solv/repo_rpmdb.h>
        // TODO: VARS
        .whitelist_function("repo_add_rpm.*")
        .whitelist_function("rpm_state_.*")
        .whitelist_function("rpm_installedrpmdbids")
        .whitelist_function("rpm_by.*")
        .whitelist_function("rpm_query.*")
        .whitelist_function("rpm_iterate_filelist")

        // <solv/repo_repomdxml.h>
        .whitelist_function("repo_add_repo.*")
        // <solv/repo_rpmmd.h>
        .whitelist_function("repo_add_rpm.*")
        // <solv/repo_deltainfoxml.h>
        .whitelist_function("repo_add_delta.*")
        // <solv/repo_updateinfoxml.h>
        .whitelist_function("repo_add_update.*")

        // Don't let bindgen recreate libsolv's types
        .blacklist_type("Chksum")
        .blacklist_type("DUChanges")
        .blacklist_type("Dataiterator")
        .blacklist_type("Datamatcher")
        .blacklist_type("s_Datapos")
        .blacklist_type("Datapos")
        .blacklist_type("s_Dirpool")
        .blacklist_type("Dirpool")
        .blacklist_type("Hashtable")
        .blacklist_type("Hashval")
        .blacklist_type("Id")
        .blacklist_type("KeyValue")
        .blacklist_type("s_Map")
        .blacklist_type("Map")
        .blacklist_type("Offset")
        .blacklist_type("s_Pool")
        .blacklist_type("Pool")
        .blacklist_type("s_Queue")
        .blacklist_type("Queue")
        .blacklist_type("s_Reldep")
        .blacklist_type("Reldep")
        .blacklist_type("s_Repo")
        .blacklist_type("Repo")
        .blacklist_type("s_Repodata")
        .blacklist_type("Repodata")
        .blacklist_type("s_Repokey")
        .blacklist_type("Repokey")
        .blacklist_type("Rule")
        .blacklist_type("s_Solvable")
        .blacklist_type("Solvable")
        .blacklist_type("s_Solver")
        .blacklist_type("Solver")
        .blacklist_type("s_Stringpool")
        .blacklist_type("Stringpool")
        .blacklist_type("Transaction")

        // Hide FILE from bindgen's output
        // Otherwise we get the OS's private file implementation
        .blacklist_type("FILE")
        .raw_line("use libc::FILE;")

        // Import necessary structs from libsolv_sys
        .raw_line("use libsolv_sys::{Chksum, DUChanges, Dataiterator, Datamatcher, Datapos, Dirpool};")
        .raw_line("use libsolv_sys::{Hashtable, Hashval, Id, KeyValue, Map, Offset, Pool, Queue};")
        .raw_line("use libsolv_sys::{Reldep, Repo, s_Repo, Repodata, Repokey, Rule, Solvable, Solver, Stringpool};")
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
