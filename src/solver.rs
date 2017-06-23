use std::cell::RefCell;
use std::rc::Rc;
use ::pool::PoolContext;
use libsolv_sys::Solver as _Solver;

pub struct Solver {
    ctx: Rc<RefCell<PoolContext>>,
    _s: *mut _Solver,
}

impl Solver {
    pub(crate) fn new_with_context(ctx: Rc<RefCell<PoolContext>>) -> Self {
        use libsolv_sys::solver_create;
        let _s = {
            let borrow = ctx.borrow_mut();
            unsafe {solver_create(borrow._p)}
        };
        Solver{ctx: ctx, _s: _s}
    }

    pub(crate) fn new_with_solver(ctx: Rc<RefCell<PoolContext>>, _s: *mut _Solver) -> Self {
        Solver{ctx: ctx, _s: _s}
    }

}

impl Drop for Solver {
    fn drop(&mut self) {
        use libsolv_sys::solver_free;
        let borrow = self.ctx.borrow_mut();
        unsafe{solver_free(self._s)}
    }
}