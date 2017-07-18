use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::CString;
use ::pool::Pool;
use libsolv_sys::Repo as _Repo;

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