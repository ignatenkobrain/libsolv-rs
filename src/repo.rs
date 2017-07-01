use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::CString;
use ::pool::Pool;
use libsolv_sys::Repo as _Repo;

pub struct Repo {
    ctx: Rc<RefCell<Pool>>,
    _r: *mut _Repo,
}

impl Repo {
    pub(crate) fn new_with_context(ctx: Rc<RefCell<Pool>>, name: &CString) -> Self {
        use libsolv_sys::repo_create;
        let _r = {
            let borrow = ctx.borrow_mut();
            unsafe {repo_create(borrow._p, name.as_ptr())}
        };
        Repo{ctx: ctx, _r: _r}
    }

    fn add_repomd_xml(&mut self, s: &str) {
        //use libsolvext_sys::repo_add_repomdxml;
    }
}

impl Drop for Repo {
    fn drop(&mut self) {
        use libsolv_sys::repo_freedata;
        let borrow = self.ctx.borrow_mut();
        unsafe{repo_freedata(self._r)}
    }
}