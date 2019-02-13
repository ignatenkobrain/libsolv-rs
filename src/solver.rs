use std::cell::RefCell;
use std::rc::Rc;
use ::pool::Pool;
use ::queue::Queue;
use libsolv_sys::Solver as _Solver;
use libc;

pub struct Solver {
    pub(crate) ctx: Rc<RefCell<Pool>>,
    pub(crate) _s: *mut _Solver,
}

impl Solver {
    pub(crate) fn new_with_context(ctx: Rc<RefCell<Pool>>) -> Self {
        use libsolv_sys::solver_create;
        let _s = {
            let borrow = ctx.borrow_mut();
            unsafe {solver_create(borrow._p)}
        };
        Solver{ctx: ctx, _s: _s}
    }

    pub(crate) fn new_with_solver(ctx: Rc<RefCell<Pool>>, _s: *mut _Solver) -> Self {
        Solver{ctx: ctx, _s: _s}
    }

    pub fn solve(&mut self, job: &mut Queue) -> libc::c_int {
        use libsolv_sys::solver_solve;
        let borrow = self.ctx.borrow_mut();
        unsafe{solver_solve(self._s, &mut job._q)}
    }

}

impl Drop for Solver {
    fn drop(&mut self) {
        use libsolv_sys::solver_free;
        let borrow = self.ctx.borrow_mut();
        unsafe{solver_free(self._s)}
    }
}