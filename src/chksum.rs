
use libsolv_sys::Chksum as _Chksum;
use libsolv_sys::Id;
use std::ptr;
use std::mem;
use libsolv_sys::solv_knownid;
use libc;

pub struct Chksum {
    _c: *mut _Chksum,
}

impl Chksum {
    fn new(id: Id) -> Option<Chksum> {
        use libsolv_sys::solv_chksum_create;
        let _c = unsafe{solv_chksum_create(id)};
        if _c.is_null() {
            return None;
        } else {
            Some(Chksum{_c: _c})
        }
    }

    pub fn new_md5() -> Option<Chksum> {
        Chksum::new(solv_knownid::REPOKEY_TYPE_MD5 as Id)
    }

    pub fn new_sha1() -> Option<Chksum> {
        Chksum::new(solv_knownid::REPOKEY_TYPE_SHA1 as Id)
    }

    pub fn new_sha256() -> Option<Chksum> {
        Chksum::new(solv_knownid::REPOKEY_TYPE_SHA256 as Id)
    }

    pub fn add(&mut self, s: &str) {
        use libsolv_sys::solv_chksum_add;
        let l = s.as_bytes().len();
        unsafe {solv_chksum_add(self._c, s.as_bytes().as_ptr() as *const libc::c_void, l as i32)}
    }

    pub fn add_fp(&mut self, fp: *mut libc::FILE) {
        use libsolv_sys::solv_chksum_add;
        let mut buffer: [u8; 4096] = [0; 4096];
        let mut l = 0;
        unsafe {
            loop {
                l = libc::fread(buffer.as_mut_ptr() as *mut libc::c_void, 1, mem::size_of::<[u8; 4096]>(), fp);
                if l > 0 {
                    solv_chksum_add(self._c, buffer.as_ptr() as *const libc::c_void, l as i32);
                } else {
                    break;
                }
            }
            libc::rewind(fp);
        }
    }

    pub fn add_fd(&mut self, fd: libc::c_int) {
        use libsolv_sys::solv_chksum_add;
        let mut buffer: [u8; 4096] = [0; 4096];
        let mut l = 0;
        unsafe {
            loop {
                l = libc::read(fd, buffer.as_mut_ptr() as *mut libc::c_void, mem::size_of::<[u8; 4096]>());
                if l > 0 {
                    solv_chksum_add(self._c, buffer.as_ptr() as *const libc::c_void, l as i32);
                } else {
                    break;
                }
            }
            libc::lseek(fd, 0, 0);
        }
    }

    pub fn add_fstat(&mut self, fd: libc::c_int) {
        use libsolv_sys::solv_chksum_add;
        let stb: libc::stat = unsafe {
            let mut tmp = mem::uninitialized();
            if libc::fstat(fd, &mut tmp) == 0 {
                mem::uninitialized()
            } else {
                tmp
            }
        };
        unsafe {
            solv_chksum_add(self._c, &stb.st_dev as *const i32 as *const libc::c_void, mem::size_of_val(&stb.st_dev) as i32);
            solv_chksum_add(self._c, &stb.st_ino as *const u64 as *const libc::c_void, mem::size_of_val(&stb.st_ino) as i32);
            solv_chksum_add(self._c, &stb.st_size as *const i64 as *const libc::c_void, mem::size_of_val(&stb.st_size) as i32);
            solv_chksum_add(self._c, &stb.st_mtime as *const i64 as *const libc::c_void, mem::size_of_val(&stb.st_mtime) as i32);
        }


    }

}

impl Drop for Chksum {
    fn drop(&mut self) {
        use libsolv_sys::solv_chksum_free;
        unsafe {solv_chksum_free(self._c, ptr::null_mut() as *mut u8)};
    }
}