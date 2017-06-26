
use libsolv_sys::Chksum as _Chksum;
use libsolv_sys::Id;
use std::ptr;
use libsolv_sys::solv_knownid;


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

    fn new_md5() -> Option<Chksum> {
        Chksum::new(solv_knownid::REPOKEY_TYPE_MD5 as Id)
    }

    fn new_sha1() -> Option<Chksum> {
        Chksum::new(solv_knownid::REPOKEY_TYPE_SHA1 as Id)
    }

    fn new_sha256() -> Option<Chksum> {
        Chksum::new(solv_knownid::REPOKEY_TYPE_SHA256 as Id)
    }



}

impl Drop for Chksum {
    fn drop(&mut self) {
        use libsolv_sys::solv_chksum_free;
        unsafe {solv_chksum_free(self._c, ptr::null_mut() as *mut u8)};
    }
}