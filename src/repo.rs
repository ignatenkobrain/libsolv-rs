use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::ffi::{CString, CStr};
use std::ptr;
use std::mem;
use libc;
use ::pool::Pool;
use libsolv_sys::{Id, Repo as _Repo, Dataiterator as _Dataiterator};

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
    repo: &'a mut Repo,
    match_str: CString,
    _di: *mut _Dataiterator
}

impl<'a> DataIterator<'a> {
    fn new<T: AsRef<str>>(repo: &'a mut Repo, p: Id, key: Id, m: T, flags: libc::c_int) -> DataIterator<'a> {
        use libsolv_sys::{solv_calloc, dataiterator_init};
        let _repo = repo._r;
        let pool = repo.ctx.borrow_mut();
        let match_str = CString::new(m.as_ref())
            .expect(&format!("Unable to create CString from {:?}", m.as_ref()));

        let di = unsafe {
            let mut di = solv_calloc(1, mem::size_of::<_Dataiterator>()) as *mut _Dataiterator;
            dataiterator_init(di, pool._p, _repo, p, key, match_str.as_ptr(), flags);
            di
        };

        DataIterator{pool: pool, repo: repo, match_str: match_str, _di: di}

    }
}

impl<'a> Drop for DataIterator<'a> {
    fn drop(&mut self) {
        use libsolv_sys::{dataiterator_free, solv_free};
        unsafe {
            dataiterator_free(self._di);
            solv_free(self._di as *mut libc::c_void);
        }
        self._di = ptr::null_mut();
    }
}