extern crate libc;
extern crate libsolv;
extern crate libsolv_sys;
extern crate libsolvext_sys;

use std::ffi::{CStr, CString};
use std::fs::File;
use std::os::unix::io::*;
use std::ptr;
use std::mem;
use std::slice;
use std::path::{Path, PathBuf};
use libsolv::chksum::Chksum;

use libc::FILE;

use libsolv_sys::{_Pool, Pool, Dataiterator};
use libsolv_sys::{pool_create, pool_setdebuglevel, pool_setloadcallback};
use libsolv_sys::pool_free;
use libsolv_sys::pool_setarch;
use libsolvext_sys::solv_xfopen_fd;
use libsolv_sys::Repo;
use libsolv_sys::_Chksum;
use libsolv_sys::repo_create;
use libsolv_sys::repo_free;
use libsolvext_sys::{repo_add_repomdxml, repo_add_rpmmd, repo_add_repodata};
use libsolv_sys::{SEARCH_STRING, SOLVID_META};
use libsolv_sys::{solv_knownid, Id};
use libsolv_sys::dataiterator_free;
use libsolv_sys::dataiterator_init;
use libsolv_sys::dataiterator_prepend_keyname;
use libsolv_sys::dataiterator_step;
use libsolv_sys::{dataiterator_init_clone, dataiterator_strdup};
use libsolv_sys::dataiterator_setpos_parent;
use libsolv_sys::{SOLVID_POS, pool_lookup_str, pool_lookup_bin_checksum, solv_chksum_create_from_bin};
use libsolv_sys::repodata_set_str;
use libsolv_sys::repodata_set_bin_checksum;
use libsolv_sys::repodata_new_handle;
use libsolv_sys::repo_id2repodata;
use libsolv_sys::repodata_set_poolstr;
use libsolv_sys::solv_chksum_get_type;
use libsolv_sys::solv_chksum_get;
use libsolv_sys::repodata_add_flexarray;
use libsolv_sys::repodata_internalize;
use libsolv_sys::repodata_add_idarray;
use libsolv_sys::repodata_create_stubs;
use libsolv_sys::pool_addfileprovides_queue;
use libsolv_sys::queue_init;
use libsolv_sys::Queue;
use libsolv_sys::{_Repodata, Repodata};
use libsolv_sys::pool_createwhatprovides;
use libsolv_sys::queue_free;
use libsolv_sys::solver_create;
use libsolv_sys::solver_free;
use libsolv_sys::solv_chksum_free;

pub type LoadCallback = Option<Box<Fn(_Repodata)>>;

fn xf_open(path: &CStr, file: &File) -> *mut FILE {
    let fd = file.as_raw_fd();
    unsafe {
        let new_fd = libc::dup(fd);
        if fd == -1 {
            panic!("unable to dup {:?}", file);
        }
        libc::fcntl(new_fd, libc::F_SETFD, libc::FD_CLOEXEC);
        let fp = solv_xfopen_fd(path.as_ptr(), fd, ptr::null());
        if fp.is_null() {
            libc::close(fd);
            panic!("Unable to open fd {:?}", file);
        }
        fp
    }
}

fn find(pool: *mut Pool, repo: *mut Repo, what: &str) -> (Option<CString>, Option<*mut _Chksum>){
    let what = CString::new(what)
        .unwrap();
    let mut lookup_cstr = None;
    let mut lookup_chksum = None;

    let mut di = unsafe {
        let mut di = mem::uninitialized();
        dataiterator_init(&mut di, pool, repo,
                          SOLVID_META as Id, solv_knownid::REPOSITORY_REPOMD_TYPE as Id, what.as_ptr(), SEARCH_STRING as Id);
        dataiterator_prepend_keyname(&mut di, solv_knownid::REPOSITORY_REPOMD as Id);
        di
    };

    while unsafe{dataiterator_step(&mut di)} != 0 {
        println!("loop!");
        let mut ndi = unsafe {
            let mut ndi = mem::uninitialized();
            dataiterator_init_clone(&mut ndi, &mut di);
            dataiterator_strdup(&mut ndi);
            ndi
        };

        let pos = {
            let _pool: &mut _Pool = unsafe { &mut *ndi.pool };
            let old_pos = _pool.pos;
            unsafe { dataiterator_setpos_parent(&mut ndi) };
            let pos = _pool.pos;
            _pool.pos = old_pos;

            println!("pos: {:?}", &pos);
            pos
        };
        lookup_cstr = {
            let repo: &mut Repo = unsafe {&mut *pos.repo};
            let _pool: &mut _Pool = unsafe{&mut *repo.pool};
            let old_pos = _pool.pos;
            _pool.pos = pos;
            let cstr = unsafe {pool_lookup_str(_pool, SOLVID_POS, solv_knownid::REPOSITORY_REPOMD_LOCATION as Id)};
            _pool.pos = old_pos;
            if cstr.is_null() {
                None
            } else {
                unsafe {
                    let len = libc::strlen(cstr);
                    let slice = slice::from_raw_parts(cstr as *const libc::c_uchar, len as usize);
                    CString::new(slice).ok()
                }
            }
        };
        println!("cstr: {:?}", lookup_cstr);

        lookup_chksum = {
            let repo: &mut Repo = unsafe {&mut *pos.repo};
            let _pool: &mut _Pool = unsafe{&mut *repo.pool};
            let old_pos = _pool.pos;
            _pool.pos = pos;
            let mut type_id = 0;
            let b = unsafe {pool_lookup_bin_checksum(_pool, SOLVID_POS, solv_knownid::REPOSITORY_REPOMD_CHECKSUM as Id, &mut type_id)};
            _pool.pos = old_pos;
            let _c = unsafe {solv_chksum_create_from_bin(type_id, b)};
            if _c.is_null() {
                None
            } else {
                Some(_c)
            }
        };
        println!("chksum: {:?}", lookup_chksum);

        unsafe{dataiterator_free(&mut ndi)};

        if lookup_cstr.is_some() {
            break;
        }

    }
    unsafe{dataiterator_free(&mut di)};

    (lookup_cstr, lookup_chksum)
}

fn updateaddedprovides(pool: *mut Pool, repo: *mut Repo, addwhatprovies: &mut Queue) {
    let repo: &mut Repo = unsafe {&mut *repo};
    let _pool: &mut _Pool = unsafe{&mut *repo.pool};
    if repo.nsolvables == 0 {
        println!("0 nsolvables");
        return;
    }
    //first_repodata()
    if repo.nsolvables < 2 {
        println!("too few nsolvables");
        return;
    }
    let mut repodate: &mut Repodata = unsafe{&mut *repo_id2repodata(repo, 1)};
    if repodate.loadcallback.is_some() {
        println!("loadcallback found on 1st repodata");
        return
    }

    for i in 2..repo.nsolvables {
        repodate = unsafe{&mut *repo_id2repodata(repo, i)};
        if repodate.loadcallback.is_none() {
            println!("loadcallback not found on repodata {}", i);
            return
        }
    }
    // we don't get any farther on this function.

}

fn load_repo<P: AsRef<str>>(pool: *mut Pool, base_path: P) {

    let mut repomdstr = base_path.as_ref().to_owned();
    repomdstr.push_str("/repodata/repomd.xml");

    // Load the repomd.xml
    let repo_cstr_path = CString::new(repomdstr.clone())
        .unwrap();
    let repomd_file = File::open(repomdstr)
        .unwrap();
    // Get the file pointer for repomd.xml
    let repomd_fp = xf_open(&repo_cstr_path,&repomd_file);

    // Create the repo & load the repomd
    let repo_name = CString::new("reproducer")
        .unwrap();
    let repo = unsafe{repo_create(pool, repo_name.as_ptr())};
    unsafe {repo_add_repomdxml(repo, repomd_fp, 0)};

    // Close repomd_fp
    unsafe{libc::fclose(repomd_fp)};

    // Search for the primary entry

    let(option_cstr, option_chksum) = find(pool, repo, "primary");

    let repo_md_chksum = option_chksum
        .expect("Expected checksum");
    let repo_md_name = option_cstr
        .expect("Expected name");

    // Load the primary entry
    let mut repo_file_buf = PathBuf::new();
    repo_file_buf.push(base_path.as_ref());
    repo_file_buf.push(repo_md_name.to_str().unwrap());
    let repo_file = File::open(repo_file_buf)
        .unwrap();

    let repo_fd = xf_open(&repo_md_name, &repo_file);
    unsafe{repo_add_rpmmd(repo, repo_fd, ptr::null(), 0)};
    unsafe{libc::fclose(repo_fd)};

    // add_exts
    let mut repodata_id = {
        let rd = unsafe {& *repo_add_repodata(repo, 0)};
        rd.repodataid
    };

    let(option_cstr, option_chksum) = find(pool, repo, "filelists");
    let filelists_chksum = option_chksum
        .expect("Expected checksum");
    let filelists_name = option_cstr
        .expect("Expected name");

    let filelists_cstr = CString::new("filelists").unwrap();
    unsafe {
        let repomd_handle = repodata_new_handle(repo_id2repodata(repo, repodata_id));
        repodata_set_poolstr(repo_id2repodata(repo, repodata_id), repomd_handle, solv_knownid::REPOSITORY_REPOMD_TYPE as Id, filelists_cstr.as_ptr());
        repodata_set_str(repo_id2repodata(repo, repodata_id), repomd_handle, solv_knownid::REPOSITORY_REPOMD_LOCATION as Id, filelists_name.as_ptr());
        let chksum_buf = solv_chksum_get(filelists_chksum, ptr::null_mut());
        repodata_set_bin_checksum(repo_id2repodata(repo, repodata_id), repomd_handle, solv_knownid::REPOSITORY_REPOMD_CHECKSUM as Id, solv_chksum_get_type(filelists_chksum), chksum_buf);
        repodata_add_idarray(repo_id2repodata(repo, repodata_id), repomd_handle, solv_knownid::REPOSITORY_KEYS as Id, solv_knownid::SOLVABLE_FILELIST as Id);
        repodata_add_idarray(repo_id2repodata(repo, repodata_id), repomd_handle, solv_knownid::REPOSITORY_KEYS as Id, solv_knownid::REPOKEY_TYPE_DIRSTRARRAY as Id);
        repodata_add_flexarray(repo_id2repodata(repo, repodata_id), SOLVID_META, solv_knownid::REPOSITORY_EXTERNAL as Id, repomd_handle);
        repodata_internalize(repo_id2repodata(repo, repodata_id));
        let mut data = repo_id2repodata(repo, repodata_id);
        let ref_data = &*repodata_create_stubs(data);
        repodata_id = ref_data.repodataid;
    }

    unsafe {
        solv_chksum_free(filelists_chksum, ptr::null_mut());
    }

    //addedprovides = pool.addfileprovides_queue()
    let mut whatprovides_queue = unsafe{
        let mut queue =  mem::uninitialized();
        queue_init(&mut queue);
        pool_addfileprovides_queue(pool, &mut queue, ptr::null_mut());
        queue
    };

    updateaddedprovides(pool, repo, &mut whatprovides_queue);
    unsafe {queue_free(&mut whatprovides_queue)};

    unsafe{pool_createwhatprovides(pool)};

    unsafe{repo_free(repo, 0)};
}

fn box_callback<F: 'static + Fn(Repodata)>(cb: F) -> Box<LoadCallback> {
    Box::new(Some(Box::new(cb)))
}

unsafe extern "C" fn loadcallback(_p: *mut _Pool, _rd: *mut _Repodata, _d: *mut libc::c_void) -> libc::c_int {
    let cb = _d as *const LoadCallback;
    println!("Entering callback function");
    if let Some(ref function) = *cb {
        function(*_rd);
    };
    0
}

fn main() {
    // Create the pool
    let pool = unsafe{pool_create()};
    unsafe{pool_setdebuglevel(pool, 2)};
    // Set the pool arch
    let arch = CString::new("x86_64").unwrap();
    let mut callback = box_callback(|_| println!("loadsuccess."));
    unsafe{
        pool_setarch(pool, arch.as_ptr());
        let cb_ptr = &mut *callback as *mut LoadCallback as *mut libc::c_void;
        pool_setloadcallback(pool, Some(loadcallback), cb_ptr)

    };

    load_repo(pool, "/Users/abaxter/Projects/fedora-modularity/depchase/repos/rawhide/x86_64/os");

    let solver = unsafe {solver_create(pool)};
    // left off at creating solver
    unsafe{solver_free(solver)};
    unsafe{pool_free(pool)};
}