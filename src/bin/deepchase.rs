extern crate libc;
extern crate libsolv;

#[macro_use]
extern crate clap;

use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use std::collections::HashMap;
use clap::App;
use libsolv::pool::PoolContext;
use libsolv::repo::Repo;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::ffi::CString;
use std::ptr;
use std::io::{Cursor, Read};
use libsolv::chksum::Chksum;
use libsolv::ext::solvfile::SolvFile;

use libsolv::errors::*;

struct BaseRepo {
    name: String,
    base_url: String,
}

impl BaseRepo {

    fn new<T: Into<String>, U: Into<String>>(name: T, base_url: U) -> Result<Self> {
        let name = name.into();
        let base_url = base_url.into();

        let mut path = PathBuf::from(&base_url);
        path.push("repodata/repomd.xml");
        let mut repomd = SolvFile::open(&path)?;


        Ok(BaseRepo{name: name, base_url: base_url})
    }

    fn calc_cookie<R: Read>(r: &mut R) -> Box<[u8]> {
        let mut chksum = Chksum::new_sha256().unwrap();
        chksum.add("1.1");
        chksum.read_in(r);
        chksum.into_boxed_slice()
    }

    fn calc_cookie_ext(cookie: &[u8], file: &File) -> Box<[u8]> {
        let mut chksum = Chksum::new_sha256().unwrap();
        chksum.add("1.1");
        chksum.add(cookie);
        chksum.add_fstat(file);
        chksum.into_boxed_slice()
    }

}

struct OsRepo {
    repo: BaseRepo,
    src_repo: Option<SourceRepo>
}

impl OsRepo {
    fn new<T: Into<String>, U: Into<String>>(name: T, base_url: U, src_repo: Option<SourceRepo>) -> Result<Self> {
        BaseRepo::new(name.into(), base_url.into())
            .map(|base| OsRepo{repo: base , src_repo: src_repo})

    }

    fn has_src(&self) -> bool {
        self.src_repo.is_some()
    }

    fn src(&self) -> Option<&SourceRepo> {
        self.src_repo.as_ref()
    }

    fn src_mut(&mut self) -> Option<&mut SourceRepo> {
        self.src_repo.as_mut()
    }
}

struct SourceRepo {
    repo: BaseRepo
}

impl SourceRepo {
    fn new<T: Into<String>, U: Into<String>>(name: T, base_url: U) -> Result<Self> {
        BaseRepo::new(name.into(), base_url.into())
            .map(|base| SourceRepo{repo: base})
    }
}


// Skip reading config for now.
fn setup_repos(arch: &str, conf_file: &str, pool_context: &PoolContext) -> Result<Vec<OsRepo>> {

    let base_dir = "~/Projects/fedora-modularity/depchase/repos";
    let os_base = base_dir.to_owned() + "/rawhide/{arch}/os";
    let source_base = base_dir.to_owned() + "/rawhide/{arch}/os";
    let m: HashMap<String, String> = vec![
        ("base".to_owned(), os_base),
        ("base-source".to_owned(), source_base)
    ].into_iter()
        .map(|(k, v)| (k, v.replace("{arch}", arch)))
        .collect();

    {
        let mut pool = pool_context.borrow_mut();
        pool.set_arch(arch);
        //pool.set_loadcallback(load_stub)
    }

    m.iter()
        .filter(|&(k, _)| !k.ends_with("-source"))
        .map(|(k, v)| {
            let source_key = k.clone() + "-source";
            let mut source_result= m
                .get(&source_key)
                .map(|base| SourceRepo::new(source_key, base.clone()));

            let source_repo = match source_result {
                Some(Ok(repo)) => Ok(Some(repo)),
                Some(Err(e)) => Err(e),
                None => Ok(None)
            };

            OsRepo::new(k.clone(), v.clone(), source_repo?)
        }).collect()
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

    let arch = matches.value_of("ARCH").unwrap();
    let config = matches.value_of("CONFIG").unwrap();

    if let Some(resolve) = matches.subcommand_matches("resolve") {
        let mut pool_context = PoolContext::new();
        let mut repos = setup_repos(&arch, &config, &pool_context);
        for value in resolve.values_of("PACKAGE").unwrap() {
            println!("Package: {}", value);
        }
    }
}