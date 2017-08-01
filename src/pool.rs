use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;
use libsolv_sys::{Pool as PoolT, _Pool, _Repodata};
use ::solver::Solver;
use ::repo::Repo;
use std::ffi::CString;
use ::transaction::Transaction;
use std::marker::PhantomData;
use std::ptr;
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


pub struct Pool {
    pub _p: *mut PoolT,
}

impl Pool {
    fn new() -> Pool {
        use libsolv_sys::pool_create;
        Pool { _p: unsafe {pool_create()} }
    }

    pub fn set_arch(&mut self, arch: &str) { ;
        use libsolv_sys::pool_setarch;
        let string = CString::new(arch).unwrap();
        unsafe {pool_setarch(self._p, string.as_ptr())};
    }

    pub fn clear_loadcallback(&mut self) {
        use libsolv_sys::pool_setloadcallback;
        unsafe {pool_setloadcallback(self._p, None, ptr::null_mut())};
    }

    pub fn set_loadcallback(&mut self, cb: unsafe extern "C" fn(_: *mut _Pool, _: *mut _Repodata, _: *mut libc::c_void) -> libc::c_int) {
        use libsolv_sys::pool_setloadcallback;
        unsafe {pool_setloadcallback(self._p, Some(cb), ptr::null_mut())};
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        use libsolv_sys::pool_free;
        unsafe { pool_free(self._p)}
    }
}