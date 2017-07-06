use std::fs::File;
use std::path::Path;
use std::ffi::CString;
use libc;
use std::ptr;
use std::os::unix::io::*;

pub fn open<T: AsRef<Path>>(p: &T) -> Option<File> {
    let f = File::open(p).unwrap();
    let fd = f.as_raw_fd();
    open_fd(p, fd)
}

pub fn open_fd<T: AsRef<Path>>(p: &T, mut fd: libc::c_int) -> Option<File> {
    use libsolvext_sys::solv_xfopen_fd;
    let cstr = CString::new(p.as_ref().to_str().unwrap()).unwrap();

    unsafe {
        fd = libc::dup(fd);
        if fd == -1 {
            return None;
        }
        libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC);
        let _f = solv_xfopen_fd(cstr.as_ptr(), fd, ptr::null());
        if _f.is_null() {
            libc::close(fd);
            return None;
        }
        Some(FromRawFd::from_raw_fd(fd))
    }
}