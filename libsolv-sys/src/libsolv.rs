use libc::{c_int, c_uint};

pub type Id = c_int;
pub type Offset = c_uint;
pub type Hashval = c_uint;
pub type Hashtable = *mut Id;