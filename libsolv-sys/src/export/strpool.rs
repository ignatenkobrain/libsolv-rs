use {Stringpool, Id};
use libc::c_char;

extern "C" {
    pub fn e_stringpool_id2str(ss: *mut Stringpool, id: Id) -> *const c_char;
}