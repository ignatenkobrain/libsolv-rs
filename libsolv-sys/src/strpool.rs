use libsolv::{Id, Offset, Hashval, Hashtable};
use libc::{c_char, c_int ,c_uint};


#[repr(C)]
pub struct Stringpool {
    pub strings: *mut Offset,
    pub nstrings: c_int,
    pub stringspace: *mut c_char,
    pub sstrings: Offset,
    pub stringhashtbl: Hashtable,
    pub stringhashmask: Hashval,
}

#[test]
fn bindgen_test_layout_Stringpool() {
    assert_eq!(::core::mem::size_of::<Stringpool>() , 48usize , concat ! (
               "Size of: " , stringify ! ( Stringpool ) ));
    assert_eq! (::core::mem::align_of::<Stringpool>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( Stringpool ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const Stringpool ) ) . strings as * const _ as
            usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( Stringpool ) , "::" ,
                stringify ! ( strings ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const Stringpool ) ) . nstrings as * const _
            as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( Stringpool ) , "::" ,
                stringify ! ( nstrings ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const Stringpool ) ) . stringspace as * const
        _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( Stringpool ) , "::" ,
                stringify ! ( stringspace ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const Stringpool ) ) . sstrings as * const _
            as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( Stringpool ) , "::" ,
                stringify ! ( sstrings ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const Stringpool ) ) . stringhashtbl as *
        const _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( Stringpool ) , "::" ,
                stringify ! ( stringhashtbl ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const Stringpool ) ) . stringhashmask as *
        const _ as usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( Stringpool ) , "::" ,
                stringify ! ( stringhashmask ) ));
}

extern "C" {
    pub fn stringpool_init(ss: *mut Stringpool,
                           strs: *mut *const c_char);
    pub fn stringpool_init_empty(ss: *mut Stringpool);
    pub fn stringpool_clone(ss: *mut Stringpool, from: *mut Stringpool);
    pub fn stringpool_free(ss: *mut Stringpool);
    pub fn stringpool_freehash(ss: *mut Stringpool);
    pub fn stringpool_str2id(ss: *mut Stringpool, str: *const c_char,
                             create: c_int) -> Id;
    pub fn stringpool_strn2id(ss: *mut Stringpool, str: *const c_char,
                              len: c_uint, create: c_int) -> Id;
    pub fn stringpool_shrink(ss: *mut Stringpool);
}

pub unsafe fn stringpool_id2str(ss: *mut Stringpool, id: Id) -> *const c_char {
    let ref pool = *ss;
    pool.stringspace.offset(*pool.strings.offset(id as isize) as isize)

}