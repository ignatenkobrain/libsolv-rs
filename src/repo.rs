use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::ffi::{CString, CStr};
use std::ptr;
use std::mem;
use std::iter::Iterator;
use std::marker::PhantomData;
use libc;
use ::pool::Pool;
use ::chksum::Chksum;
use libsolv_sys::{Repo as _Repo, Dataiterator as _Dataiterator, Datapos as _Datapos};
use ::{Id, solv_knownid};
use std::fmt::Display;
use std::fmt;

pub use libsolv_sys::{SEARCH_STRING, SOLVID_META};

pub struct RepoDataRef {}

pub struct Repo {
    pub(crate) ctx: Rc<RefCell<Pool>>,
    pub(crate) _r: *mut _Repo,
}



impl Repo {
    pub(crate) fn new_with_context<S: AsRef<str>>(ctx: Rc<RefCell<Pool>>, name: S) -> Self {
        use libsolv_sys::repo_create;
        let cstr_name = CString::new(name.as_ref()).expect("invalid cstring");
        let _r = {
            let borrow = ctx.borrow_mut();
            unsafe {repo_create(borrow._p, cstr_name.as_ptr())}
        };
        Repo{ctx: ctx, _r: _r}
    }

    pub fn iter_mut(& mut self, p: Id, key: Id) -> DataIterator {
        DataIterator::new(self, p, key)
    }

    pub fn iter_mut_with_string<T:AsRef<str>>(&mut self, p: Id, key: Id, what: T, flags: libc::c_int) -> DataIterator {
        DataIterator::new_with_string(self, p, key, what, flags)
    }
}

impl Drop for Repo {
    fn drop(&mut self) {
        use libsolv_sys::repo_free;
        let borrow = self.ctx.borrow_mut();
        unsafe{repo_free(self._r, 0)}
    }
}


pub struct DataIterator<'a> {
    pool: RefMut<'a, Pool>,
    what: Option<CString>,
    _di: _Dataiterator
}

impl<'a> DataIterator<'a> {

    fn new(repo: &mut Repo, p: Id, key: Id) -> DataIterator {
        use libsolv_sys::{solv_calloc, dataiterator_init};
        let pool = repo.ctx.borrow_mut();

        let di = unsafe {
            let mut di = mem::uninitialized();
            // TODO: handle non-zero returns?
            dataiterator_init(&mut di, pool._p, repo._r, p, key, ptr::null(), 0);
            di
        };

        DataIterator{pool: pool, what: None, _di: di}
    }

    fn new_with_string<T: AsRef<str>>(repo: &mut Repo, p: Id, key: Id, what: T, flags: libc::c_int) -> DataIterator {
        use libsolv_sys::{solv_calloc, dataiterator_init};
        let pool = repo.ctx.borrow_mut();
        let what_str = CString::new(what.as_ref())
            .expect(&format!("Unable to create CString from {:?}", what.as_ref()));

        let what_ptr = what_str.as_ptr();

        let di = unsafe {
            let mut di = mem::uninitialized();
            // TODO: handle non-zero returns?
            dataiterator_init(&mut di, pool._p, repo._r, p, key, what_str.as_ptr(), flags);
            di
        };

        let dis = DataIterator{pool: pool, what: Some(what_str), _di: di};
        if let Some(ref moved_cstr) = dis.what {
            println!("Orig: {:?}, Moved: {:?}", what_ptr, moved_cstr.as_ptr());
        }
        dis
    }

    pub fn prepend_keyname(&mut self, key_name: solv_knownid) {
        use libsolv_sys::dataiterator_prepend_keyname;
        unsafe {dataiterator_prepend_keyname(&mut self._di, key_name as Id)};
    }

}

impl<'a> Drop for DataIterator<'a> {
    fn drop(&mut self) {
        use libsolv_sys::{dataiterator_free, solv_free};
        unsafe {
            dataiterator_free(&mut self._di);
        }
    }
}

impl<'a> Iterator for DataIterator<'a> {
    type Item = DataMatch<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        use libsolv_sys::dataiterator_step;
        if unsafe {dataiterator_step(&mut self._di) } == 0 {
            None
        } else {
            Some(DataMatch::clone_from(self._di, PhantomData))
        }
    }
}

#[derive(Debug)]
pub struct DataMatch<'a> {
    _ndi: _Dataiterator,
    _l: PhantomData<&'a ()>
}

impl<'a> DataMatch<'a> {

    fn clone_from(mut di: _Dataiterator, l: PhantomData<&'a ()>) -> DataMatch {

        println!("clone from: {:?}", &di);
        use libsolv_sys::{dataiterator_init_clone, dataiterator_strdup};
        let ndi = unsafe {
            let mut ndi = mem::uninitialized();
            dataiterator_init_clone(&mut ndi, &mut di);
            dataiterator_strdup(&mut ndi);
            ndi
        };

        println!("cloned di: {:?}", &ndi);
        DataMatch{_ndi: ndi, _l: l}
    }

    pub fn pos(&mut self) -> DataPos {
        use libsolv_sys::dataiterator_setpos;
        let ref mut pool = unsafe{*self._ndi.pool};
        let old_pos = pool.pos;
        unsafe {dataiterator_setpos(&mut self._ndi)};
        let pos = pool.pos;
        pool.pos = old_pos;
        DataPos{_dp: pos, _l: PhantomData}
    }

    pub fn parent_pos(&mut self) -> DataPos {
        use libsolv_sys::dataiterator_setpos_parent;
        println!("kv parent: {:?}", unsafe{&*self._ndi.kv.parent});
        let ref mut pool = unsafe{*self._ndi.pool};
        println!("old_pool: {:?}", pool);
        let old_pos = pool.pos;
        println!("old_pos: {:?}", &old_pos);
        unsafe {dataiterator_setpos_parent(&mut self._ndi)};
        println!("parent_pool: {:?}", pool);
        let pos = pool.pos;
        println!("pos: {:?}", &pos);
        pool.pos = old_pos;
        DataPos{_dp: pos, _l: PhantomData}
    }
}

impl<'a> Display for DataMatch<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use libsolv_sys::{repodata_stringify, SEARCH_FILES, SEARCH_CHECKSUMS, s_KeyValue};
        let kv = &self._ndi.kv as *const s_KeyValue as *mut s_KeyValue;
        let string = unsafe{repodata_stringify(self._ndi.pool, self._ndi.data, self._ndi.key, kv, (SEARCH_FILES | SEARCH_CHECKSUMS) as i32)};
        let cstr = unsafe{CStr::from_ptr(string)};
        write!(f, "{:?}", cstr)
    }
}

//TODO: Left off at DataPos

impl<'a> Drop for DataMatch<'a> {

    fn drop(&mut self) {
        use libsolv_sys::{dataiterator_free, solv_free};
        unsafe {
            dataiterator_free(&mut self._ndi);
        }
    }
}

#[derive(Debug)]
pub struct DataPos<'a> {
    _dp: _Datapos,
    _l: PhantomData<&'a ()>
}

impl<'a> DataPos<'a> {

    pub fn lookup_str(&mut self, keyname: Id) -> Option<&CStr> {
        use libsolv_sys::{SOLVID_POS, pool_lookup_str};
        let ref mut pool = unsafe{*(*(self._dp).repo).pool};
        let old_pos = pool.pos;
        pool.pos = self._dp;
        unsafe {
            let r = pool_lookup_str(pool, SOLVID_POS, keyname);
            pool.pos = old_pos;
            if r.is_null() {
                None
            } else {
                Some(CStr::from_ptr(r))
            }
        }
    }

    pub fn lookup_checksum(&mut self, keyname:Id) -> Option<Chksum> {
        use libsolv_sys::{SOLVID_POS, pool_lookup_bin_checksum, solv_chksum_create_from_bin};
        println!("Made it into function: {:?}", self);

        let repo = unsafe{&mut *self._dp.repo};
        //ERROR: Incorrect state encountered when uncommenting the next line.
        //println!("repo : {:?}", repo.pool);

        // let pool = unsafe {&mut *repo.pool};
                //let ref mut pool = unsafe{*(*(self._dp).repo).pool};
        /*
                println!("deref pool.");
                let old_pos = pool.pos;
                let mut type_id = 0;
                pool.pos = self._dp;
                println!("Made it past pos shenanigans.");
                unsafe {
                    let b = pool_lookup_bin_checksum(pool, SOLVID_POS, keyname, &mut type_id);
                    pool.pos = old_pos;
                    let _c = solv_chksum_create_from_bin(type_id, b);
                    if _c.is_null() {
                        None
                    } else {
                        Some(Chksum::new_from(_c))
                    }

                }*/

        None
    }

}
