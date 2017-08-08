use ::repo::Repo;
use libc;
use std::ptr;
use ::ext::solvfile::*;

pub trait RpmMd {

    fn add_repomd(&mut self, file: &mut SolvFile) -> bool;

    fn add_repomdxml(&mut self, file: &mut SolvFile) -> bool;
}

impl RpmMd for Repo {
    fn add_repomd(&mut self, file: &mut SolvFile) -> bool {
        use libsolvext_sys::repo_add_rpmmd;
        let borrow = self.ctx.borrow_mut();
        unsafe {
            let r = repo_add_rpmmd(self._r, file._fp, ptr::null(), 0);
            r == 0
        }
    }

    fn add_repomdxml(&mut self, file: &mut SolvFile) -> bool {
        use libsolvext_sys::repo_add_repomdxml;
        let borrow = self.ctx.borrow_mut();
        unsafe {
            let r = repo_add_repomdxml(self._r, file._fp, 0);
            println!("r: {}", r);
            r == 0
        }
    }
}