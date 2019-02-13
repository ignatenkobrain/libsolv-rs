use Map;
use libc::c_int;

extern "C" {
    pub fn e_map_empty(m: *mut Map);
    pub fn e_map_set(m: *mut Map, n: c_int);
    pub fn e_map_setall(m: *mut Map);
    pub fn e_map_clr(m: *mut Map, n: c_int);
    pub fn e_map_tst(m: *const Map, n: c_int) -> c_int;
}
