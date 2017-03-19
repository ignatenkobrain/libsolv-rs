extern crate gcc;
extern crate pkg_config;

fn main() {
    pkg_config::probe_library("libsolv").unwrap();
    gcc::compile_library("libsolv-static-functions.a", &["static-functions.c"]);
}
