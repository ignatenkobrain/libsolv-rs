use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::ffi::{CString, CStr};
use std::ptr;
use std::mem;
use std::marker::PhantomData;
use libc;
use ::pool::Pool;
use ::chksum::Chksum;
use libsolv_sys::{Id, Repo as _Repo, Dataiterator as _Dataiterator, Datapos as _Datapos};

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

fn should_fail(repo: &mut Repo) -> DataPos {
    repo.iter_mut(1, 2).next().parent_pos()
}

impl Drop for Repo {
    fn drop(&mut self) {
        use libsolv_sys::repo_freedata;
        let borrow = self.ctx.borrow_mut();
        unsafe{repo_freedata(self._r)}
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

        let di = unsafe {
            let mut di = mem::uninitialized();
            // TODO: handle non-zero returns?
            dataiterator_init(&mut di, pool._p, repo._r, p, key, what_str.as_ptr(), flags);
            di
        };

        DataIterator{pool: pool, what: Some(what_str), _di: di}
    }

    fn next(&mut self) -> DataMatch {
        DataMatch::clone_from(&mut self._di)
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

pub struct DataMatch<'a> {
    _ndi: _Dataiterator,
    _l: PhantomData<&'a ()>
}

impl<'a> DataMatch<'a> {

    fn clone_from(di: &mut _Dataiterator) -> DataMatch {
        use libsolv_sys::{solv_calloc, dataiterator_init_clone, dataiterator_strdup};
        let ndi = unsafe {
            let mut ndi = mem::uninitialized();
            dataiterator_init_clone(&mut ndi, di);
            dataiterator_strdup(&mut ndi);
            ndi
        };

        DataMatch{_ndi: ndi, _l: PhantomData}
    }

    fn pos(&mut self) -> DataPos {
        use libsolv_sys::dataiterator_setpos;
        let ref mut pool = unsafe{*self._ndi.pool};
        let old_pos = pool.pos;
        unsafe {dataiterator_setpos(&mut self._ndi)};
        let pos = pool.pos;
        pool.pos = old_pos;
        DataPos{_dp: pos, _l: PhantomData}
    }

    fn parent_pos(&mut self) -> DataPos {
        use libsolv_sys::dataiterator_setpos_parent;
        let ref mut pool = unsafe{*self._ndi.pool};
        let old_pos = pool.pos;
        unsafe {dataiterator_setpos_parent(&mut self._ndi)};
        let pos = pool.pos;
        pool.pos = old_pos;
        DataPos{_dp: pos, _l: PhantomData}
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


pub struct DataPos<'a> {
    _dp: _Datapos,
    _l: PhantomData<&'a ()>
}

impl<'a> DataPos<'a> {

    pub fn lookup_str(&mut self, keyname: Id) -> &CStr {
        use libsolv_sys::{SOLVID_POS, pool_lookup_str};
        let ref mut pool = unsafe{*(*(self._dp).repo).pool};
        let old_pos = pool.pos;
        pool.pos = self._dp;
        let r = unsafe {pool_lookup_str(pool, SOLVID_POS, keyname)};
        pool.pos = old_pos;
        unsafe {CStr::from_ptr(r)}
    }

    pub fn lookup_checksum(&mut self, keyname:Id) -> Chksum {
        use libsolv_sys::{SOLVID_POS, pool_lookup_bin_checksum, solv_chksum_create_from_bin};
        let ref mut pool = unsafe{*(*(self._dp).repo).pool};
        let old_pos = pool.pos;
        let mut type_id = 0;
        pool.pos = self._dp;
        unsafe {
            let b = pool_lookup_bin_checksum(pool, SOLVID_POS, keyname, &mut type_id);
            pool.pos = old_pos;
            let _c = solv_chksum_create_from_bin(type_id, b);
            Chksum::new_from(_c)
        }
    }

}