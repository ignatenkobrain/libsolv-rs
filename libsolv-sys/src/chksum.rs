use libsolv::Id;
use libc::{c_int, c_uchar, c_char, c_void};

// Opaque pointer
#[repr(C)]
pub struct Chksum([u8; 0]);

extern "C" {
    pub fn solv_chksum_create(type_: Id) -> *mut Chksum;
    pub fn solv_chksum_create_clone(chk: *mut Chksum) -> *mut Chksum;
    pub fn solv_chksum_create_from_bin(type_: Id,
                                       buf: *const c_uchar)
                                       -> *mut Chksum;
    pub fn solv_chksum_add(chk: *mut Chksum,
                           data: *const c_void,
                           len: c_int);
    pub fn solv_chksum_get_type(chk: *mut Chksum) -> Id;
    pub fn solv_chksum_isfinished(chk: *mut Chksum) -> c_int;
    pub fn solv_chksum_get(chk: *mut Chksum, lenp: *mut c_int)
                           -> *const c_uchar;
    pub fn solv_chksum_free(chk: *mut Chksum,
                            cp: *mut c_uchar)
                            -> *mut c_void;
    pub fn solv_chksum_type2str(type_: Id) -> *const c_char;
    pub fn solv_chksum_str2type(str: *const c_char) -> Id;
    pub fn solv_chksum_len(type_: Id) -> c_int;
    pub fn solv_chksum_cmp(chk: *mut Chksum, chk2: *mut Chksum)
                           -> c_int;
}