
use libsolv;

#[macro_use]
extern crate clap;

use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use std::collections::HashMap;
use clap::App;
use libsolv::pool::PoolContext;
use libsolv::repo::{Repo, SEARCH_STRING, SOLVID_META};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::ffi::CString;
use std::ptr;
use std::io::{Cursor, Read};
use libsolv::chksum::Chksum;
use libsolv::ext::solvfile::*;
use libsolv::ext::rpmmd::*;
use libsolv::sys::{s_Pool, s_Repodata, s_Repo};
use libsolv::errors::*;
use libsolv::{solv_knownid, Id};


#[derive(Debug)]
struct BaseRepo {
    name: String,
    base_url: PathBuf,
}

impl BaseRepo {

    // change into AsRef<Path>
    fn new<T: Into<String>, U: AsRef<Path>>(pool_context: &PoolContext, name: T, base_url: U) -> Result<Self> {
        let name = name.into();
        let base_url = base_url.as_ref();

        //REPRODUCER EDIT

        let mut repomd_path = Path::new("./reproducer_files/repomd.xml");

        // Analyze the repomd.xml
        let mut repomd = SolvFile::open(&repomd_path)?;
        println!("Opened repomd.xml");

        let cookie = Self::calc_cookie(&mut repomd);
        println!("calc_cookie finished. {:?}", &cookie);
        repomd.rewind();

        println!("Rewind succeedd.");

        // Create repo in the pool
        let mut repo = pool_context.create_repo(&name);

        println!("Created repo.");

        repo.add_repomdxml(&mut repomd);
        // skip cached repo for now

        println!("Added repo");

        // TODO: stopped at find function
        // Need to hand data iterator in a sane fashion

        let mut di = repo.iter_mut_with_string(SOLVID_META as Id, solv_knownid::REPOSITORY_REPOMD_TYPE as Id, "primary", libsolv::repo::SEARCH_STRING as Id);
        di.prepend_keyname(solv_knownid::REPOSITORY_REPOMD);
        println!("Created di");
        for mut d in di {
            println!(" Iter di: {}", &d);
            //TODO: about to try regular pos?
            println!("Looking up parent pos");
            let mut dp = d.parent_pos();
            println!("DP after return: {:?}", dp);
            let chksum = dp.lookup_checksum(solv_knownid::REPOSITORY_REPOMD_CHECKSUM as Id);
                        //println!("Looked up checksum");
            /*
                let filename = dp.lookup_str(solv_knownid::REPOSITORY_REPOMD_LOCATION as Id);

                println!("Looked up str");

                println!("How'd it go? {:?}, {:?}", filename, chksum.map(|c| c.into_boxed_slice()) );*/
        }

        Ok(BaseRepo{name: name, base_url: base_url.to_path_buf()})
    }

    fn calc_cookie<R: Read>(r: &mut R) -> Box<[u8]> {
        let mut chksum = Chksum::new_sha256();
        chksum.add("1.1");
        chksum.read_in(r);
        chksum.into_boxed_slice()
    }

    fn calc_cookie_ext(cookie: &[u8], file: &File) -> Box<[u8]> {
        let mut chksum = Chksum::new_sha256();
        chksum.add("1.1");
        chksum.add(cookie);
        chksum.add_fstat(file);
        chksum.into_boxed_slice()
    }

}

#[derive(Debug)]
struct OsRepo {
    repo: BaseRepo,
    src_repo: Option<SourceRepo>
}

impl OsRepo {
    fn new<T: Into<String>, U: Into<String>>(pool_context: &PoolContext, name: T, base_url: U, src_repo: Option<SourceRepo>) -> Result<Self> {
        BaseRepo::new(pool_context, name.into(), base_url.into())
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

#[derive(Debug)]
struct SourceRepo {
    repo: BaseRepo
}

impl SourceRepo {
    fn new<T: Into<String>, U: Into<String>>(pool_context: &PoolContext, name: T, base_url: U) -> Result<Self> {
        BaseRepo::new(pool_context, name.into(), base_url.into())
            .map(|base| SourceRepo{repo: base})
    }
}


// Skip reading config for now.
fn setup_repos(arch: &str, conf_file: &str, pool_context: &PoolContext) -> Result<Vec<OsRepo>> {

    // Can't handle ~/ ?
    let base_dir = "/Users/abaxter/Projects/fedora-modularity/depchase/repos";
    let os_base = base_dir.to_owned() + "/rawhide/{arch}/os";
    let source_base = base_dir.to_owned() + "/rawhide/{arch}/sources";
    let m: HashMap<String, String> = vec![
        ("base".to_owned(), os_base),
        ("base-source".to_owned(), source_base)
    ].into_iter()
        .map(|(k, v)| (k, v.replace("{arch}", arch)))
        .collect();

    {
        let mut pool = pool_context.borrow_mut();
        pool.set_arch(arch);
        pool.set_loadcallback(|d| println!("Callback working!"));
    }

    println!("{:?}", &m);

    m.iter()
        .filter(|&(k, _)| !k.ends_with("-source"))
        .map(|(k, v)| {
            let source_key = k.clone() + "-source";
            let source_result= m
                .get(&source_key)
                .map(|base| SourceRepo::new(pool_context, source_key, base.clone()));

            println!("{:?}", &source_result);

            let source_repo = match source_result {
                Some(Ok(repo)) => Ok(Some(repo)),
                Some(Err(e)) => Err(e),
                None => Ok(None)
            };

            OsRepo::new(pool_context, k.clone(), v.clone(), source_repo?)
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
