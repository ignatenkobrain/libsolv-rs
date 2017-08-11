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
use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;

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
use libsolv_sys::Datapos;
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
use libsolv_sys::REPODATA_STUB;

pub type LoadCallback = Option<Box<Fn(_Repodata)>>;

pub struct PoolContext {
    pool_rc: Rc<RefCell<PoolHandle>>,
}

impl PoolContext {
    pub fn new() -> PoolContext {
        PoolContext {pool_rc: Rc::new(RefCell::new(PoolHandle::new()))}
    }

    pub fn create_repo<S: AsRef<str>>(&self, name: S) -> RepoHandle {
        RepoHandle::new_with_context(self.pool_rc.clone(), name)
    }

    pub fn borrow(&self) -> Ref<PoolHandle> {
        self.pool_rc.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<PoolHandle> {
        self.pool_rc.borrow_mut()
    }
}

pub struct PoolHandle {
    pool: *mut Pool,
    arch: Option<CString>,
    callback: Box<LoadCallback>
}

impl PoolHandle {
    fn new() -> PoolHandle {
        PoolHandle { pool: unsafe {pool_create()}, arch: None, callback: Box::new(None) }
    }

    pub fn clear_loadcallback(&mut self) {
        unsafe {pool_setloadcallback(self.pool, None, ptr::null_mut())};
        mem::replace(self.callback.as_mut(), None);
    }

    pub fn set_loadcallback<F: 'static + Fn(_Repodata)>(&mut self, cb: F) {
        use libsolv_sys::pool_setloadcallback;
        mem::replace(self.callback.as_mut(), Some(Box::new(cb)));
        let cb_ptr = &mut *self.callback as *mut LoadCallback as *mut libc::c_void;
        unsafe {pool_setloadcallback(self.pool, Some(loadcallback), cb_ptr)};
    }

    pub fn set_arch<S: AsRef<str>>(&mut self, arch: S) {
        self.arch = Some(CString::new(arch.as_ref()).unwrap());
    }

    pub fn set_debuglevel(&mut self, level: i32) {
        unsafe {pool_setdebuglevel(self.pool, level)};
    }

    pub unsafe fn as_ptr(&self) -> *mut Pool {
        self.pool
    }
}

impl Drop for PoolHandle {
    fn drop(&mut self) {
        unsafe { pool_free(self.pool)}
    }
}

#[derive(Debug)]
pub struct FileHandle {
    path: CString,
    file: File,
    pub fp: *mut FILE
}

impl FileHandle {
    pub fn xf_open<P: AsRef<Path>>(path: P) -> FileHandle {
        let cstring_path = CString::new(path.as_ref().to_str().unwrap())
            .unwrap();
        let file = File::open(path)
            .expect("unable to find file");
        let fd = file.as_raw_fd();
        let fp = unsafe {
            let new_fd = libc::dup(fd);
            if fd == -1 {
                panic!("unable to dup {:?}", file);
            }
            libc::fcntl(new_fd, libc::F_SETFD, libc::FD_CLOEXEC);
            let fp = solv_xfopen_fd(cstring_path.as_ptr(), fd, ptr::null());
            if fp.is_null() {
                libc::close(fd);
                panic!("Unable to open fd {:?}", file);
            }
            fp
        };
        FileHandle{path: cstring_path, file: file, fp: fp}
    }

    pub unsafe fn as_ptr(&self) -> *mut FILE {
        self.fp
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        unsafe{libc::fclose(self.fp)};
    }
}

pub struct RepoHandle {
    pool_rc: Rc<RefCell<PoolHandle>>,
    name: CString,
    repo: *mut Repo,
}

impl RepoHandle {
    pub fn new_with_context<S: AsRef<str>>(pool_rc: Rc<RefCell<PoolHandle>>, name: S) -> RepoHandle {
        let cstr_name = CString::new(name.as_ref())
            .unwrap();
        let repo = {
            let borrow = pool_rc.borrow_mut();
            unsafe{repo_create(borrow.as_ptr(), cstr_name.as_ptr())}
        };
        RepoHandle{pool_rc: pool_rc, name: cstr_name, repo: repo}
    }

    pub unsafe fn as_ptr(&self) -> *mut Repo {
        self.repo
    }

    pub fn add_repomdxml(&mut self, repomdxml_file: &mut FileHandle) {
        unsafe {repo_add_repomdxml(self.repo, repomdxml_file.as_ptr(), 0)};
    }

    pub fn add_repomd(&mut self, repomd_file: &mut FileHandle) {
        unsafe{repo_add_rpmmd(self.repo, repomd_file.as_ptr(), ptr::null(), 0)};
    }

    pub fn build_meta_iterator(&self) -> DataIteratorBuilder {
        DataIteratorBuilder{
            pool: self.pool_rc.borrow_mut(),
            repo: &self,
            p: SOLVID_META as Id,
            key: None,
            what: None,
            flags: None,
            prepend_keyname: None
        }
    }
}

impl Drop for RepoHandle {
    fn drop(&mut self) {
        let borrow = self.pool_rc.borrow_mut();
        unsafe{repo_free(self.repo, 0)};
    }
}

pub struct DataIteratorBuilder<'a> {
    pool: RefMut<'a, PoolHandle>,
    repo: &'a RepoHandle,
    p: Id,
    key: Option<Id>,
    what: Option<CString>,
    flags: Option<Id>,
    prepend_keyname: Option<Id>
}

impl<'a> DataIteratorBuilder<'a> {

    pub fn set_key(mut self, key: solv_knownid) -> Self{
        self.key = Some(key as Id);
        self
    }

    pub fn set_search_string<S: AsRef<str>>(mut self, what: S) -> Self {
        let cstr_what = CString::new(what.as_ref()).unwrap();
        self.what = Some(cstr_what);
        self.flags = Some(SEARCH_STRING as Id);
        self
    }

    pub fn set_prepend_keyname(mut self, prepend_keyname: solv_knownid) -> Self {
        self.prepend_keyname = Some(prepend_keyname as Id);
        self
    }

    pub fn build(self) -> RepoDataIterator<'a> {
        let pool = self.pool;
        let repo = self.repo;
        let p = self.p;
        let key = self.key.unwrap() as Id;
        let what = self.what.unwrap();
        let flags = self.flags.unwrap();

        let di = unsafe {
            let mut di = mem::uninitialized();
            dataiterator_init(&mut di, pool.as_ptr(), repo.as_ptr(), p, key, what.as_ptr(), flags);
            if let Some(prepend_keyname) = self.prepend_keyname {
                dataiterator_prepend_keyname(&mut di, prepend_keyname);
            }
            di
        };

        RepoDataIterator{
            pool: pool,
            repo: repo,
            di: di,
            what: what
        }
    }
}

pub struct RepoDataIterator<'a> {
    pool: RefMut<'a, PoolHandle>,
    repo: &'a RepoHandle,
    di: Dataiterator,
    what: CString
}

impl<'a> Drop for RepoDataIterator<'a> {
    fn drop(&mut self) {
        unsafe{dataiterator_free(&mut self.di)};
    }
}

impl<'a> Iterator for RepoDataIterator<'a> {
    type Item = RepoDataMatch<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if unsafe {dataiterator_step(&mut self.di) } == 0 {
            None
        } else {
            let ndi = unsafe {
                let mut ndi = mem::uninitialized();
                dataiterator_init_clone(&mut ndi, &mut self.di);
                dataiterator_strdup(&mut ndi);
                ndi
            };
            Some(RepoDataMatch{repo: self.repo, ndi: ndi})
        }
    }
}

pub struct RepoDataMatch<'a> {
    repo: &'a RepoHandle,
    ndi: Dataiterator,
}

impl<'a> RepoDataMatch<'a> {
    pub fn parent_pos(&mut self) -> RepoDataPos {
        let _pool: &mut _Pool = unsafe { &mut *self.ndi.pool };
        let old_pos = _pool.pos;
        unsafe { dataiterator_setpos_parent(&mut self.ndi) };
        let pos = _pool.pos;
        _pool.pos = old_pos;
        println!("parent pos: {:?}", &pos);
        RepoDataPos{repo: self.repo, pos: pos}
    }
}

impl<'a> Drop for RepoDataMatch<'a> {
    fn drop(&mut self) {
        unsafe{dataiterator_free(&mut self.ndi)};
    }
}

pub struct RepoDataPos<'a> {
    repo: &'a RepoHandle,
    pos: Datapos
}

impl<'a> RepoDataPos<'a> {

    pub fn location(&self) -> Option<CString> {
        let repo: &mut Repo = unsafe {&mut *self.pos.repo};
        let _pool: &mut _Pool = unsafe{&mut *repo.pool};
        println!("pos: {:?}", self.pos);
        let old_pos = _pool.pos;
        println!("pool: {:?}", _pool);
        _pool.pos = self.pos;
        println!("pool_after: {:?}", _pool);
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

    pub fn checksum(&self) -> Option<*mut _Chksum> {
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

fn find_ub(repo: &RepoHandle, what: &str) -> (Option<CString>, Option<*mut _Chksum>){
    let mut lookup_cstr = None;
    let mut lookup_chksum = None;

    let mut di = repo.build_meta_iterator()
        .set_key(solv_knownid::REPOSITORY_REPOMD_TYPE)
        .set_search_string(what)
        .set_prepend_keyname(solv_knownid::REPOSITORY_REPOMD)
        .build();

    for mut repo_match in di {
        println!("loop!");
        let parent_pos = repo_match.parent_pos();
        lookup_cstr = parent_pos.location();
        println!("cstr: {:?}", lookup_cstr);
        lookup_chksum = parent_pos.checksum();
        println!("chksum: {:?}", lookup_chksum);
        if lookup_cstr.is_some() {
            break;
        }
    }
    (lookup_cstr, lookup_chksum)
}


fn find_no_ub(pool: &mut PoolHandle, repo: &RepoHandle, what: &str) -> (Option<CString>, Option<*mut _Chksum>){
    let what = CString::new(what)
        .unwrap();
    let mut lookup_cstr = None;
    let mut lookup_chksum = None;

    let mut di = unsafe {
        let mut di = mem::uninitialized();
        dataiterator_init(&mut di, pool.as_ptr(), repo.as_ptr(),
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

fn updateaddedprovides(pool: &mut PoolHandle, repo: &mut RepoHandle, addwhatprovies: &mut Queue) {
    let repo_ref: &mut Repo = unsafe {&mut *repo.as_ptr()};
    let _pool: &mut _Pool = unsafe{&mut *repo_ref.pool};
    if repo_ref.nsolvables == 0 {
        println!("0 nsolvables");
        return;
    }
    //first_repodata()
    if repo_ref.nsolvables < 2 {
        println!("too few nsolvables");
        return;
    }
    let mut repodate: &mut Repodata = unsafe{&mut *repo_id2repodata(repo.as_ptr(), 1)};
    if repodate.loadcallback.is_some() {
        println!("loadcallback found on 1st repodata");
        return
    }

    for i in 2..repo_ref.nsolvables {
        repodate = unsafe{&mut *repo_id2repodata(repo.as_ptr(), i)};
        if repodate.loadcallback.is_none() {
            println!("loadcallback not found on repodata {}", i);
            return
        }
    }
    // we don't get any farther on this function.

}

fn load_repo<P: AsRef<str>>(pool_context: &PoolContext, repo_name: P,  base_path: P) -> RepoHandle {

    let mut repomdstr = base_path.as_ref().to_owned();
    repomdstr.push_str("/repodata/repomd.xml");

    // open the repomd.xml
    let mut repomdxml_file = FileHandle::xf_open(&repomdstr);

    // Create the repo & load the repomdxml
    let mut repo = pool_context.create_repo(repo_name);
    repo.add_repomdxml(&mut repomdxml_file);

    // Search for the primary entry

    let(option_cstr, option_chksum) = {
        let mut borrow = pool_context.borrow_mut();
        find_no_ub(&mut borrow, &repo, "primary")
    };


    //let(option_cstr, option_chksum) = find(&repo, "primary");

    let repo_md_chksum = option_chksum
        .expect("Expected primary checksum");
    let repo_md_name = option_cstr
        .expect("Expected primary name");

    // Load the primary entry
    let mut repo_file_buf = PathBuf::new();
    repo_file_buf.push(base_path.as_ref());
    repo_file_buf.push(repo_md_name.to_str().unwrap());

    let mut repomd_file = FileHandle::xf_open(repo_file_buf);


    repo.add_repomd(&mut repomd_file);

    // add_exts
    let mut repodata_id = {
        let rd = unsafe {& *repo_add_repodata(repo.as_ptr(), 0)};
        rd.repodataid
    };

    let(option_cstr, option_chksum) = {
        let mut borrow = pool_context.borrow_mut();
        find_no_ub(&mut borrow, &repo, "filelists")
    };


    //let(option_cstr, option_chksum) = find(&repo, "filelists");

    let filelists_chksum = option_chksum
        .expect("Expected checksum");
    let filelists_name = option_cstr
        .expect("Expected name");

    let filelists_cstr = CString::new("filelists").unwrap();
    unsafe {
        let repomd_handle = repodata_new_handle(repo_id2repodata(repo.as_ptr(), repodata_id));
        repodata_set_poolstr(repo_id2repodata(repo.as_ptr(), repodata_id), repomd_handle, solv_knownid::REPOSITORY_REPOMD_TYPE as Id, filelists_cstr.as_ptr());
        repodata_set_str(repo_id2repodata(repo.as_ptr(), repodata_id), repomd_handle, solv_knownid::REPOSITORY_REPOMD_LOCATION as Id, filelists_name.as_ptr());
        let chksum_buf = solv_chksum_get(filelists_chksum, ptr::null_mut());
        repodata_set_bin_checksum(repo_id2repodata(repo.as_ptr(), repodata_id), repomd_handle, solv_knownid::REPOSITORY_REPOMD_CHECKSUM as Id, solv_chksum_get_type(filelists_chksum), chksum_buf);
        repodata_add_idarray(repo_id2repodata(repo.as_ptr(), repodata_id), repomd_handle, solv_knownid::REPOSITORY_KEYS as Id, solv_knownid::SOLVABLE_FILELIST as Id);
        repodata_add_idarray(repo_id2repodata(repo.as_ptr(), repodata_id), repomd_handle, solv_knownid::REPOSITORY_KEYS as Id, solv_knownid::REPOKEY_TYPE_DIRSTRARRAY as Id);
        repodata_add_flexarray(repo_id2repodata(repo.as_ptr(), repodata_id), SOLVID_META, solv_knownid::REPOSITORY_EXTERNAL as Id, repomd_handle);
        repodata_internalize(repo_id2repodata(repo.as_ptr(), repodata_id));

        solv_chksum_free(filelists_chksum, ptr::null_mut());
    }

    // create stubs
    unsafe {
        let repo_ref = unsafe{&*repo.as_ptr()};
        if repo_ref.nrepodata != 0 {
            let  data = repo_id2repodata(repo.as_ptr(), repo_ref.nrepodata - 1);
        }
        let data = &mut *repo_id2repodata(repo.as_ptr(), repodata_id);
        if data.state != REPODATA_STUB as i32 {
            repodata_create_stubs(data);
        }
    }


    {
        let mut borrow = pool_context.borrow_mut();

        //addedprovides = pool.addfileprovides_queue()
        let mut whatprovides_queue = unsafe{
            let mut queue =  mem::uninitialized();
            queue_init(&mut queue);
            pool_addfileprovides_queue(borrow.as_ptr(), &mut queue, ptr::null_mut());
            queue
        };

        updateaddedprovides(&mut borrow, &mut repo, &mut whatprovides_queue);
        unsafe {queue_free(&mut whatprovides_queue)};
        unsafe{pool_createwhatprovides(borrow.as_ptr())};
    }



    repo
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

    let pool_context = PoolContext::new();

    {
        let mut pool = pool_context.borrow_mut();
        pool.set_debuglevel(2);
        pool.set_loadcallback(|_| println!("loadsuccess."));
        pool.set_arch("x86_64");
    }

    load_repo(&pool_context, "os", "/Users/abaxter/Projects/fedora-modularity/depchase/repos/rawhide/x86_64/os");
    load_repo(&pool_context, "source", "/Users/abaxter/Projects/fedora-modularity/depchase/repos/rawhide/x86_64/sources");

    {
        let mut pool = pool_context.borrow_mut();
        let solver = unsafe { solver_create(pool.as_ptr()) };
        // left off at creating solver
        unsafe { solver_free(solver) };
    }
}