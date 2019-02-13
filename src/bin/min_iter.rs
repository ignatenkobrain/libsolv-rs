extern crate libc;
extern crate libsolv_sys;
extern crate libsolvext_sys;

use std::ffi::{CStr, CString};
use std::ptr;
use std::mem;
use std::slice;

use libsolv_sys::{s_Pool, Pool, Dataiterator, Datapos};
use libsolv_sys::{pool_create, pool_setdebuglevel};
use libsolv_sys::pool_free;
use libsolv_sys::pool_setarch;
use libsolvext_sys::solv_xfopen;
use libsolv_sys::Repo;
use libsolv_sys::s_Chksum;
use libsolv_sys::repo_create;
use libsolv_sys::repo_free;
use libsolvext_sys::repo_add_repomdxml;
use libsolv_sys::{SEARCH_STRING, SOLVID_META};
use libsolv_sys::{solv_knownid, Id};
use libsolv_sys::dataiterator_free;
use libsolv_sys::dataiterator_init;
use libsolv_sys::dataiterator_prepend_keyname;
use libsolv_sys::dataiterator_step;
use libsolv_sys::{dataiterator_init_clone, dataiterator_strdup};
use libsolv_sys::dataiterator_setpos_parent;
use libsolv_sys::{SOLVID_POS, pool_lookup_str, pool_lookup_bin_checksum, solv_chksum_create_from_bin};
use libsolv_sys::solv_chksum_free;

#[derive(Debug)]
struct RepoDataIterator {
    di: Box<Dataiterator>,
}

impl RepoDataIterator {
    fn new(pool: *mut Pool, repo: *mut Repo, what: &CStr) -> RepoDataIterator {
        let mut di = unsafe {
            let mut di = Box::new(mem::zeroed());
            dataiterator_init(&mut *di, pool, repo,
                              SOLVID_META as Id, solv_knownid::REPOSITORY_REPOMD_TYPE as Id, what.as_ptr(), SEARCH_STRING as Id);
            dataiterator_prepend_keyname(&mut *di, solv_knownid::REPOSITORY_REPOMD as Id);
            di
        };
        RepoDataIterator{di: di}
    }
}

impl Iterator for RepoDataIterator {
    type Item = RepoDataMatch;

    fn next(&mut self) -> Option<Self::Item> {
        if unsafe {dataiterator_step(&mut *self.di) } == 0 {
            None
        } else {
            println!("loop!");
            let ndi = unsafe {
                let mut ndi = Box::new(mem::zeroed());
                dataiterator_init_clone(&mut *ndi, &mut *self.di);
                dataiterator_strdup(&mut *ndi);
                ndi
            };
            Some(RepoDataMatch{ndi: ndi})
        }
    }
}

impl Drop for RepoDataIterator {
    fn drop(&mut self) {
        unsafe{dataiterator_free(&mut *self.di)};
    }
}

#[derive(Debug)]
struct RepoDataMatch {
    ndi: Box<Dataiterator>,
}

impl RepoDataMatch {
    fn parent_pos(&mut self) -> RepoDataPos {
        let _pool: &mut s_Pool = unsafe { &mut *self.ndi.pool };
        let old_pos = _pool.pos;
        unsafe { dataiterator_setpos_parent(&mut *self.ndi) };
        let pos = _pool.pos;
        _pool.pos = old_pos;
        RepoDataPos{pos: pos}
    }
}

impl Drop for RepoDataMatch {
    fn drop(&mut self) {
        unsafe{dataiterator_free(&mut *self.ndi)};
    }
}

#[derive(Debug)]
struct RepoDataPos {
    pos: Datapos
}

impl RepoDataPos {

    pub fn location(&self) -> Option<CString> {
        let repo: &mut Repo = unsafe {&mut *self.pos.repo};
        let _pool: &mut s_Pool = unsafe{&mut *repo.pool};
        let old_pos = _pool.pos;
        _pool.pos = self.pos;
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
    }

    pub fn checksum(&self) -> Option<*mut s_Chksum> {
        let repo: &mut Repo = unsafe {&mut *self.pos.repo};
        let _pool: &mut _Pool = unsafe{&mut *repo.pool};
        let old_pos = _pool.pos;
        _pool.pos = self.pos;
        let mut type_id = 0;
        let b = unsafe {pool_lookup_bin_checksum(_pool, SOLVID_POS, solv_knownid::REPOSITORY_REPOMD_CHECKSUM as Id, &mut type_id)};
        _pool.pos = old_pos;
        let _c = unsafe {solv_chksum_create_from_bin(type_id, b)};
        if _c.is_null() {
            None
        } else {
            Some(_c)
        }
    }
}

fn find(pool: *mut Pool, repo: *mut Repo, what: &CStr) -> (Option<CString>, Option<*mut s_Chksum>) {
    let mut lookup_cstr = None;
    let mut lookup_chksum = None;

    for mut step in RepoDataIterator::new(pool, repo, what) {
        let parent_pos = step.parent_pos();
        lookup_cstr = parent_pos.location();
        lookup_chksum = parent_pos.checksum();

        if lookup_cstr.is_some() {
            println!("found");
            break;
        }
    }

    (lookup_cstr, lookup_chksum)
}

unsafe fn load_repo(pool: *mut _Pool, path: &CStr) {
    let readonly = CString::new("r").unwrap();
    let repomd_fp = solv_xfopen(path.as_ptr(), readonly.as_ptr());
    assert!(!repomd_fp.is_null());

    let repo_name = CString::new("min_iter").unwrap();
    let repo = repo_create(pool, repo_name.as_ptr());

    repo_add_repomdxml(repo, repomd_fp, 0);

    libc::fclose(repomd_fp);

    let what = CString::new("primary").unwrap();

    let (o_name, o_chksum) = find(pool, repo, &what);
    println!("cstr: {:?}", o_name);
    println!("chksum: {:?}", o_chksum);

    if let Some(primary_chksum) = o_chksum {
        solv_chksum_free(primary_chksum, ptr::null_mut());
    }

    repo_free(repo, 0);
}

fn main() {
    // Create the pool
    let pool = unsafe{pool_create()};
    unsafe{pool_setdebuglevel(pool, 2)};
    // Set the pool arch
    let arch = CString::new("x86_64").unwrap();
    let path = CString::new("files/repomd.xml").unwrap();

    unsafe{
        pool_setarch(pool, arch.as_ptr());
        load_repo(pool, &path);
    };

    unsafe{pool_free(pool)};
}
