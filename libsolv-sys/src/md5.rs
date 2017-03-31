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

#[test]
fn bindgen_test_layout_MD5_CTX() {
    assert_eq!(::core::mem::size_of::<MD5_CTX>() , 240usize , concat ! (
               "Size of: " , stringify ! ( MD5_CTX ) ));
    assert_eq! (::core::mem::align_of::<MD5_CTX>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( MD5_CTX ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const MD5_CTX ) ) . lo as * const _ as usize }
    , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( MD5_CTX ) , "::" ,
                stringify ! ( lo ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const MD5_CTX ) ) . hi as * const _ as usize }
    , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( MD5_CTX ) , "::" ,
                stringify ! ( hi ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const MD5_CTX ) ) . a as * const _ as usize } ,
    16usize , concat ! (
                "Alignment of field: " , stringify ! ( MD5_CTX ) , "::" ,
                stringify ! ( a ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const MD5_CTX ) ) . b as * const _ as usize } ,
    24usize , concat ! (
                "Alignment of field: " , stringify ! ( MD5_CTX ) , "::" ,
                stringify ! ( b ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const MD5_CTX ) ) . c as * const _ as usize } ,
    32usize , concat ! (
                "Alignment of field: " , stringify ! ( MD5_CTX ) , "::" ,
                stringify ! ( c ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const MD5_CTX ) ) . d as * const _ as usize } ,
    40usize , concat ! (
                "Alignment of field: " , stringify ! ( MD5_CTX ) , "::" ,
                stringify ! ( d ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const MD5_CTX ) ) . buffer as * const _ as
            usize } , 48usize , concat ! (
                "Alignment of field: " , stringify ! ( MD5_CTX ) , "::" ,
                stringify ! ( buffer ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const MD5_CTX ) ) . block as * const _ as usize
    } , 112usize , concat ! (
                "Alignment of field: " , stringify ! ( MD5_CTX ) , "::" ,
                stringify ! ( block ) ));
}

extern "C" {
    pub fn solv_MD5_Init(ctx: *mut MD5_CTX);
    pub fn solv_MD5_Update(ctx: *mut MD5_CTX,
                           data: *mut c_void,
                           size: c_ulong);
    pub fn solv_MD5_Final(result: *mut c_uchar,
                          ctx: *mut MD5_CTX);
}