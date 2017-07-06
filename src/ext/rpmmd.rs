use ::repo::Repo;
use libc;
use std::ptr;
use std::os::unix::io::*;
use std::fs::File;

pub trait RpmMd {

    fn add_repomd(&mut self, file: & File) -> bool;

    fn add_repomdxml(&mut self, file: & File) -> bool;
}

impl RpmMd for Repo {
    fn add_repomd(&mut self, file: &File) -> bool {
        use libsolvext_sys::repo_add_rpmmd;
        let borrow = self.ctx.borrow_mut();
        let mut fd = file.as_raw_fd();
        unsafe {
            fd = libc::dup(fd);
            if fd == -1 {
                return false;
            }
            libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC);
            let f = libc::fdopen(fd, ptr::null());
            if f.is_null() {
                libc::close(fd);
                return false;
            }
            let r = repo_add_rpmmd(self._r, f, ptr::null(), 0);

            libc::fclose(f);

            r == 0
        }
    }

    fn add_repomdxml(&mut self, file: &File) -> bool {
        use libsolvext_sys::repo_add_repomdxml;
        let borrow = self.ctx.borrow_mut();
        let mut fd = file.as_raw_fd();
        unsafe {
            fd = libc::dup(fd);
            if fd == -1 {
                return false;
            }
            libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC);
            let f = libc::fdopen(fd, ptr::null());
            if f.is_null() {
                libc::close(fd);
                return false;
            }
            let r = repo_add_repomdxml(self._r, f, 0);

            libc::fclose(f);

            r == 0
        }
    }
}