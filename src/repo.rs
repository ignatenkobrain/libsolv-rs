use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::ffi::{CString, CStr};
use std::ptr;
use std::mem;
use std::marker::PhantomData;
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

    pub fn iter_mut(& mut self, p: Id, key: Id) -> DataIterator {
        DataIterator::new(self, p, key)
    }

    pub fn iter_mut_with_string<T:AsRef<str>>(&mut self, p: Id, key: Id, what: T, flags: libc::c_int) -> DataIterator {
        DataIterator::new_with_string(self, p, key, what, flags)
    }
}

fn test(repo: &mut Repo) -> DataMatch {
    repo.iter_mut(1, 2).next()
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
    _di: *mut _Dataiterator
}

impl<'a> DataIterator<'a> {

    fn new(repo: &mut Repo, p: Id, key: Id) -> DataIterator {
        use libsolv_sys::{solv_calloc, dataiterator_init};
        let pool = repo.ctx.borrow_mut();

        let di = unsafe {
            let mut di = solv_calloc(1, mem::size_of::<_Dataiterator>()) as *mut _Dataiterator;
            // TODO: handle non-zero returns?
            dataiterator_init(di, pool._p, repo._r, p, key, ptr::null(), 0);
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
            let mut di = solv_calloc(1, mem::size_of::<_Dataiterator>()) as *mut _Dataiterator;
            // TODO: handle non-zero returns?
            dataiterator_init(di, pool._p, repo._r, p, key, what_str.as_ptr(), flags);
            di
        };

        DataIterator{pool: pool, what: Some(what_str), _di: di}
    }

    fn next(&mut self) -> DataMatch {
        DataMatch::clone_from(self._di)
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

pub struct DataMatch<'a> {
    _ndi: *mut _Dataiterator,
    _l: PhantomData<&'a ()>
}

impl<'a> DataMatch<'a> {

    fn clone_from(di: *mut _Dataiterator) -> DataMatch<'a> {
        use libsolv_sys::{solv_calloc, dataiterator_init_clone, dataiterator_strdup};
        let ndi = unsafe {
            let mut ndi = solv_calloc(1, mem::size_of::<_Dataiterator>()) as *mut _Dataiterator;
            dataiterator_init_clone(ndi, di);
            dataiterator_strdup(ndi);
            ndi
        };

        DataMatch{_ndi: ndi, _l: PhantomData}
    }
}

//TODO: Left off at DataPos

impl<'a> Drop for DataMatch<'a> {

    fn drop(&mut self) {
        use libsolv_sys::{dataiterator_free, solv_free};
        unsafe {
            dataiterator_free(self._ndi);
            solv_free(self._ndi as *mut libc::c_void);
        }
        self._ndi = ptr::null_mut();
    }
}

