use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;
use libsolv_sys::{Pool as PoolT, s_Pool, s_Repodata};
use ::solver::Solver;
use ::repo::{Repo, RepoDataRef};
use std::ffi::CString;
use ::transaction::Transaction;
use std::marker::PhantomData;
use std::ptr;
use std::mem;
use libc;

pub struct PoolContext {
    pool: Rc<RefCell<Pool>>,
}

impl PoolContext {
    pub fn new() -> Self {
        PoolContext {pool: Rc::new(RefCell::new(Pool::new()))}
    }

    pub fn create_solver(&self) -> Solver {
        Solver::new_with_context(self.pool.clone())
    }

    pub fn create_transaction(&self) -> Transaction {
        Transaction::new_with_context(self.pool.clone())
    }

    pub fn create_repo<S: AsRef<str>>(&self, name: S) -> Repo {
        Repo::new_with_context(self.pool.clone(), name)
    }

    pub fn clone_context(&self) -> Rc<RefCell<Pool>> {
        self.pool.clone()
    }

    pub fn borrow(&self) -> Ref<Pool> {
        self.pool.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<Pool> {
        self.pool.borrow_mut()
    }

}

pub type LoadCallback = Option<Box<Fn(RepoDataRef)>>;

pub struct Pool {
    pub _p: *mut PoolT,
    callback: Box<LoadCallback>
}

impl Pool {
    fn new() -> Pool {
        use libsolv_sys::{pool_create, pool_setdebuglevel};
        let p = Pool { _p: unsafe {pool_create()}, callback: Box::new(None) };
        unsafe {pool_setdebuglevel(p._p, 4)};
        p

    }

    pub fn set_arch(&mut self, arch: &str) { ;
        use libsolv_sys::pool_setarch;
        let string = CString::new(arch).unwrap();
        unsafe {pool_setarch(self._p, string.as_ptr())};
    }

    pub fn clear_loadcallback(&mut self) {
        use libsolv_sys::pool_setloadcallback;
        unsafe {pool_setloadcallback(self._p, None, ptr::null_mut())};
        mem::replace(self.callback.as_mut(), None);
    }

    pub fn set_loadcallback<F: 'static + Fn(RepoDataRef)>(&mut self, cb: F) {
        use libsolv_sys::pool_setloadcallback;
        mem::replace(self.callback.as_mut(), Some(Box::new(cb)));
        let cb_ptr = &mut *self.callback as *mut LoadCallback as *mut libc::c_void;
        unsafe {pool_setloadcallback(self._p, Some(loadcallback), cb_ptr)};
    }
}

unsafe extern "C" fn loadcallback(_p: *mut s_Pool, _rd: *mut s_Repodata, _d: *mut libc::c_void) -> libc::c_int {
    let cb = _d as *const LoadCallback;
    println!("Entering callback function");
    if let Some(ref function) = *cb {
        let b = RepoDataRef{};
        function(b);
    };
    0
}

impl Drop for Pool {
    fn drop(&mut self) {
        use libsolv_sys::pool_free;
        unsafe { pool_free(self._p)}
    }
}
