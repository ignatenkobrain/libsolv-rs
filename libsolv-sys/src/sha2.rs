
pub const SHA224_BLOCK_LENGTH: usize = 64;
pub const SHA224_DIGEST_LENGTH: usize = 28;
pub const SHA256_BLOCK_LENGTH: usize = 64;
pub const SHA256_DIGEST_LENGTH: usize = 32;
pub const SHA384_BLOCK_LENGTH: usize = 128;
pub const SHA384_DIGEST_LENGTH: usize = 48;
pub const SHA512_BLOCK_LENGTH: usize = 128;
pub const SHA512_DIGEST_LENGTH: usize = 64;

#[repr(C)]
pub struct SHA256_CTX {
    pub state: [u32; 8],
    pub bitcount: u64,
    pub buffer: [u32; SHA256_BLOCK_LENGTH/4],
}

#[repr(C)]
pub struct SHA512_CTX {
    pub state: [u64; 8],
    pub bitcount: [u64; 2],
    pub buffer: [u64; SHA512_BLOCK_LENGTH/8],
}

#[test]
fn bindgen_test_layout_SHA512_CTX() {
    assert_eq!(::core::mem::size_of::<SHA512_CTX>() , 208usize , concat ! (
               "Size of: " , stringify ! ( SHA512_CTX ) ));
    assert_eq! (::core::mem::align_of::<SHA512_CTX>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( SHA512_CTX ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const SHA512_CTX ) ) . state as * const _ as
            usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( SHA512_CTX ) , "::" ,
                stringify ! ( state ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const SHA512_CTX ) ) . bitcount as * const _
            as usize } , 64usize , concat ! (
                "Alignment of field: " , stringify ! ( SHA512_CTX ) , "::" ,
                stringify ! ( bitcount ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const SHA512_CTX ) ) . buffer as * const _ as
            usize } , 80usize , concat ! (
                "Alignment of field: " , stringify ! ( SHA512_CTX ) , "::" ,
                stringify ! ( buffer ) ));
}

#[allow(non_camel_case_types)]
pub type SHA224_CTX = SHA256_CTX;

#[allow(non_camel_case_types)]
pub type SHA384_CTX = SHA512_CTX;

extern "C" {
    pub fn solv_SHA224_Init(context: *mut SHA224_CTX);
    pub fn solv_SHA224_Update(context: *mut SHA224_CTX, sha2_byte: *const u8,
                              len: usize);
    pub fn solv_SHA224_Final(sha2_byte: *mut u8, context: *mut SHA224_CTX);
    pub fn solv_SHA256_Init(context: *mut SHA256_CTX);
    pub fn solv_SHA256_Update(context: *mut SHA256_CTX, sha2_byte: *const u8,
                              len: usize);
    pub fn solv_SHA256_Final(sha2_byte: *mut u8, context: *mut SHA256_CTX);
    pub fn solv_SHA384_Init(context: *mut SHA384_CTX);
    pub fn solv_SHA384_Update(context: *mut SHA384_CTX, sha2_byte: *const u8,
                              len: usize);
    pub fn solv_SHA384_Final(sha2_byte: *mut u8, context: *mut SHA384_CTX);
    pub fn solv_SHA512_Init(context: *mut SHA512_CTX);
    pub fn solv_SHA512_Update(context: *mut SHA512_CTX, sha2_byte: *const u8,
                              len: usize);
    pub fn solv_SHA512_Final(sha2_byte: *mut u8, context: *mut SHA512_CTX);
}