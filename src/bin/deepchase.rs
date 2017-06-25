extern crate libc;
extern crate libsolv;

#[macro_use]
extern crate clap;

use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use std::collections::HashMap;
use clap::App;
use libsolv::pool::PoolContext;


struct BaseRepo {
    name: String,
    base_url: String,
}

impl BaseRepo {
    fn new<T: Into<String>, U: Into<String>>(name: T, base_url: U) -> Self {
        BaseRepo{name: name.into(), base_url: base_url.into()}
    }
}

struct OsRepo {
    repo: BaseRepo,
    src_repo: Option<SourceRepo>
}

impl OsRepo {
    fn new<T: Into<String>, U: Into<String>>(name: T, base_url: U) -> Self {
        OsRepo{repo: BaseRepo::new(name.into(), base_url.into()), src_repo: None}
    }

    fn set_src(&mut self, src_repo: SourceRepo) {
        self.src_repo = Some(src_repo);
    }
}

struct SourceRepo {
    repo: BaseRepo
}

impl SourceRepo {
    fn new<T: Into<String>, U: Into<String>>(name: T, base_url: U) -> Self {
        SourceRepo{repo: BaseRepo::new(name.into(), base_url.into())}
    }
}


// Substantially simplified for speed
fn setup_repos(conf_file: &str) -> Vec<OsRepo> {
    let base_dir = "~/Projects/fedora-modularity/depchase/repos";
    let os_base = base_dir.to_owned() + "/rawhide/x86_64/os";
    let source_base = base_dir.to_owned() + "/rawhide/x86_64/os";

    let source = SourceRepo::new("base-source", source_base);
    let mut base = OsRepo::new("base", os_base);
    base.set_src(source);

    vec![base]
}

fn main() {
    let matches = clap_app!(deepchase =>
        (version: "0.1")
        (author: "Igor Gnatenko, Stephen Gallagher, Adam Baxter")
        (about: "Chase down runtime/buildtime requirements")
        (@arg ARCH: -a --arch +takes_value +required "Specify the CPU architecture.")
        (@arg CONFIG: -c --config +takes_value +required "Path to configuration.")
        (@arg VERBOSE: -v --verbose +multiple)
        (@subcommand resolve =>
            (about: "resolve command")
            (@arg PACKAGE: +required +multiple)
        )

    ).get_matches();

    let config = matches.value_of("CONFIG").unwrap();
    println!("Value for config: {}", config);

    if let Some(sub_m) = matches.subcommand_matches("resolve") {
        for value in sub_m.values_of("PACKAGE").unwrap() {
            println!("Package: {}", value);
        }
    }
}