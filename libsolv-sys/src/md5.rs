use libc::{c_uchar, c_ulong, c_void};

#[repr(C)]
pub struct MD5_CTX {
    pub lo: c_ulong,
    pub hi: c_ulong,
    pub a: c_ulong,
    pub b: c_ulong,
    pub c: c_ulong,
    pub d: c_ulong,
    pub buffer: [c_uchar; 64],
    pub block: [c_ulong; 16],
}

extern "C" {
    pub fn solv_MD5_Init(ctx: *mut MD5_CTX);
    pub fn solv_MD5_Update(ctx: *mut MD5_CTX,
                           data: *mut c_void,
                           size: c_ulong);
    pub fn solv_MD5_Final(result: *mut c_uchar,
                          ctx: *mut MD5_CTX);
}