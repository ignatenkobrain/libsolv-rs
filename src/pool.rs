use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;
use libsolv_sys::Pool as _Pool;
use ::solver::Solver;
use ::repo::Repo;
use std::ffi::CString;
use ::transaction::Transaction;


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

    pub fn create_repo(&self, name: &CString) -> Repo {
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

    pub fn set_arch(&self, arch: &str) { ;
        use libsolv_sys::pool_setarch;
        let borrow = self.borrow_mut();
        let string = CString::new(arch).unwrap();
        unsafe {pool_setarch(borrow._p, string.as_ptr())};
    }
}


pub struct Pool {
    pub _p: *mut _Pool,
}

impl Pool {
    fn new() -> Pool {
        use libsolv_sys::pool_create;
        Pool { _p: unsafe {pool_create()} }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        use libsolv_sys::pool_free;
        unsafe { pool_free(self._p)}
    }
}