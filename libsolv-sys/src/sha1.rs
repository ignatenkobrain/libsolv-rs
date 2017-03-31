#[repr(C)]
pub struct SHA1_CTX {
    pub state: [u32; 5],
    pub count: [u32; 2],
    pub buffer: [u8; 64],
}

#[test]
fn bindgen_test_layout_SHA1_CTX() {
    assert_eq!(::std::mem::size_of::<SHA1_CTX>() , 92usize , concat ! (
               "Size of: " , stringify ! ( SHA1_CTX ) ));
    assert_eq! (::std::mem::align_of::<SHA1_CTX>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( SHA1_CTX ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const SHA1_CTX ) ) . state as * const _ as
            usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( SHA1_CTX ) , "::" ,
                stringify ! ( state ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const SHA1_CTX ) ) . count as * const _ as
            usize } , 20usize , concat ! (
                "Alignment of field: " , stringify ! ( SHA1_CTX ) , "::" ,
                stringify ! ( count ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const SHA1_CTX ) ) . buffer as * const _ as
            usize } , 28usize , concat ! (
                "Alignment of field: " , stringify ! ( SHA1_CTX ) , "::" ,
                stringify ! ( buffer ) ));
}

pub const SHA1_DIGEST_SIZE:usize = 20;

extern "C" {
    pub fn solv_SHA1_Init(context: *mut SHA1_CTX);
    pub fn solv_SHA1_Update(context: *mut SHA1_CTX, data: *const u8,
                            len: usize);
    pub fn solv_SHA1_Final(context: *mut SHA1_CTX, digest: *mut u8);
}