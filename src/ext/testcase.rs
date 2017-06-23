use std::io::Result;
use std::path::Path;
use ::solver::Solver;
use libsolv_sys::Solver as _Solver;
use ::pool::Pool;
use ::queue::Queue;
use std::ptr;
use std::ffi::CStr;
use libc::{c_char, c_int, FILE};

pub fn read<P: AsRef<Path>, T: AsRef<CStr>>(pool: &Pool, path: P, testcase: T, job: &mut Queue) -> Result<Solver> {
    use libsolv_sys::testcase_read;

    let fp: *mut FILE = ptr::null_mut();
    let mut resultp: *mut c_char = ptr::null_mut();
    let mut resultflagsp: c_int = 0;
    let solver: *mut _Solver = {
        let borrow = pool.borrow_context_mut();
        unsafe {testcase_read(borrow._p, fp, testcase.as_ref().as_ptr(), &mut job._q, &mut resultp, &mut resultflagsp)}
    };

    //TODO: We left off here. Use path, not testcase. Check solver result
    Ok(Solver::new_with_solver(pool.clone_context(), solver))

}