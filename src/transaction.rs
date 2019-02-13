use std::cell::RefCell;
use std::rc::Rc;
use ::pool::Pool;
use libsolv_sys::Transaction as _Transaction;

pub struct Transaction {
    pub(crate) ctx: Rc<RefCell<Pool>>,
    pub(crate) _t: *mut _Transaction,
}

impl Transaction {
    pub(crate) fn new_with_context(ctx: Rc<RefCell<Pool>>) -> Self {
        use libsolv_sys::transaction_create;
        let _t = {
            let borrow = ctx.borrow_mut();
            unsafe {transaction_create(borrow._p)}
        };
        Transaction{ctx: ctx, _t: _t}
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        use libsolv_sys::transaction_free;
        let borrow = self.ctx.borrow_mut();
        unsafe{transaction_free(self._t)}
    }
}