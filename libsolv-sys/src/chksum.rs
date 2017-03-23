use libsolv::Id;
use libc::{c_int, c_uchar, c_char, c_void};
use ::md5::MD5_CTX;
use ::sha1::SHA1_CTX;
use ::sha2::{SHA224_CTX, SHA256_CTX, SHA384_CTX, SHA512_CTX};

#[repr(C)]
pub union ContextUnion {
    pub md5: MD5_CTX,
    pub sha1: SHA1_CTX,
    pub sha224: SHA224_CTX,
    pub sha256: SHA256_CTX,
    pub sha384: SHA384_CTX,
    pub sha512: SHA512_CTX,
}


#[repr(C)]
pub struct Chksum {
    pub type_: Id,
    pub done: c_int,
    pub result: [c_uchar; 64],
    pub c: ContextUnion,
}

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