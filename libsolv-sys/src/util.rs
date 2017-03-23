use libc::{c_void, c_char, c_uchar, c_uint, c_int};

extern "C" {
    pub fn solv_malloc(len: usize) -> *mut c_void;
    pub fn solv_malloc2(num: usize, len: usize)
                        -> *mut c_void;
    pub fn solv_calloc(num: usize, len: usize)
                       -> *mut c_void;
    pub fn solv_realloc(old: *mut c_void, len: usize)
                        -> *mut c_void;
    pub fn solv_realloc2(old: *mut c_void, num: usize,
                         len: usize) -> *mut c_void;
    pub fn solv_extend_realloc(old: *mut c_void, len: usize,
                               size: usize, block: usize)
                               -> *mut c_void;
    pub fn solv_free(mem: *mut c_void)
                     -> *mut c_void;
    pub fn solv_strdup(s: *const c_char)
                       -> *mut c_char;
    pub fn solv_oom(num: usize, len: usize);
    pub fn solv_timems(subtract: c_uint)
                       -> c_uint;
    pub fn solv_sort(base: *mut c_void, nmemb: usize,
                     size: usize,
                     compar:  Option<unsafe extern "C" fn(arg1: *const c_void,
                                                          arg2: *const c_void,
                                                          arg3: *mut c_void) -> c_int>,
                     compard: *mut c_void);
    pub fn solv_dupjoin(str1: *const c_char,
                        str2: *const c_char,
                        str3: *const c_char)
                        -> *mut c_char;
    pub fn solv_dupappend(str1: *const c_char,
                          str2: *const c_char,
                          str3: *const c_char)
                          -> *mut c_char;
    pub fn solv_hex2bin(strp: *mut *const c_char,
                        buf: *mut c_uchar,
                        bufl: c_int) -> c_int;
    pub fn solv_bin2hex(buf: *const c_uchar,
                        l: c_int,
                        str: *mut c_char)
                        -> *mut c_char;
    pub fn solv_validutf8(buf: *const c_char) -> usize;
    pub fn solv_latin1toutf8(buf: *const c_char)
                             -> *mut c_char;
    pub fn solv_replacebadutf8(buf: *const c_char,
                               replchar: c_int)
                               -> *mut c_char;
}