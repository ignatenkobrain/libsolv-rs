use std::fs::File;
use std::path::Path;
use std::ffi::CString;
use libc;
use libc::FILE;
use std::ptr;
use std::os::unix::io::*;
use std::io::Read;
use std::io;
use std::mem;

use ::errors::*;

pub struct SolvFile {
    pub(crate) _fp: *mut FILE,
}

impl SolvFile {
    pub fn open<T: AsRef<Path>>(p: &T) -> Result<Self> {

        let file = File::open(p)
            .chain_err(||format!("Unable to open {:?}", p.as_ref()))?;
        let fd = file.as_raw_fd();

        SolvFile::open_fd(p, fd)
    }

    pub fn open_fd<T: AsRef<Path>>(p: &T, mut fd: libc::c_int) -> Result<Self> {
        use libsolvext_sys::solv_xfopen_fd;

        // Is this really necessary?
        let cstr = p.as_ref()
            .to_str()
            .ok_or_else(|| format!("Unable convert &Path to &str {:?}", p.as_ref()).into())
            .and_then(|s| CString::new(s).chain_err(|| format!("Unable to convert &str to CString: {:?}", &s)))
            ?;

        unsafe {
            fd = libc::dup(fd);
            if fd == -1 {
                bail!("Unable to dupe {:?}", p.as_ref());
            }
            libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC);
            let _fp = solv_xfopen_fd(cstr.as_ptr(), fd, ptr::null());
            if _fp.is_null() {
                libc::close(fd);
                bail!("Unable to open fd {:?}", p.as_ref());
            }
            Ok(SolvFile{_fp : _fp})
        }
    }

    pub fn rewind(&mut self) {
        use libc::rewind;
        unsafe {rewind(self._fp)}
    }

}

impl Drop for SolvFile {

    fn drop(&mut self) {
        if !self._fp.is_null() {
            unsafe {
                libc::fclose(self._fp);
                self._fp = ptr::null_mut();
            };
        }
    }
}

impl Read for SolvFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        use libc::fread;
        let len = mem::size_of_val(buf);
        let l = unsafe {
            fread(buf.as_mut_ptr() as *mut libc::c_void, 1, len, self._fp)
        };
        println!("{:?}", l);
        Ok(l)
    }
}