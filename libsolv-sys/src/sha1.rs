#[repr(C)]
pub struct SHA1_CTX {
    pub state: [u32; 5],
    pub count: [u32; 2],
    pub buffer: [u8; 64],
}

pub const SHA1_DIGEST_SIZE:usize = 20;

extern "C" {
    pub fn solv_SHA1_Init(context: *mut SHA1_CTX);
    pub fn solv_SHA1_Update(context: *mut SHA1_CTX, data: *const u8,
                            len: usize);
    pub fn solv_SHA1_Final(context: *mut SHA1_CTX, digest: *mut u8);
}