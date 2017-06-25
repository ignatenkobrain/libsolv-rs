use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;
use libsolv_sys::Pool as _Pool;
use ::solver::Solver;
use ::repo::Repo;
use std::ffi::CString;
use ::transaction::Transaction;


pub struct Pool {
    pool: Rc<RefCell<PoolContext>>,
}

impl Pool {
    pub fn new() -> Self {
        Pool {pool: Rc::new(RefCell::new(PoolContext::new()))}
    }

    pub fn create_solver(&self) -> Solver {
        Solver::new_with_context(self.pool.clone())
    }

    pub fn create_transaction(&self) -> Transaction {
        Transaction::new_with_context(self.pool.clone())
    }

    pub fn create_repo(&self, name: &CString) -> Repo {
        Repo::new_with_context(self.pool.clone(), name)
    }

    pub fn clone_context(&self) -> Rc<RefCell<PoolContext>> {
        self.pool.clone()
    }

    pub fn borrow_context(&self) -> Ref<PoolContext> {
        self.pool.borrow()
    }

    pub fn borrow_context_mut(&self) -> RefMut<PoolContext> {
        self.pool.borrow_mut()
    }

    pub fn set_arch(&self, arch: &str) {
        use libsolv_sys::pool_setarch;
        let borrow = self.borrow_context_mut();
        let string = CString::new(arch).unwrap();
        unsafe {pool_setarch(borrow._p, string.as_ptr())};
    }
}


pub struct PoolContext {
    pub _p: *mut _Pool,
}

impl PoolContext {
    fn new() -> PoolContext {
        use libsolv_sys::pool_create;
        PoolContext { _p: unsafe {pool_create()} }
    }
}

impl Drop for PoolContext {
    fn drop(&mut self) {
        use libsolv_sys::pool_free;
        unsafe { pool_free(self._p)}
    }
}