use {Repodata, Repokey, Id};
use libc::c_int;

extern "C" {
    pub fn e_repodata_id2key(data: *mut Repodata, keyid: Id) -> *mut Repokey;
    pub fn e_repodata_id2schema(data: *mut Repodata, schemaid: Id) -> *mut Id;
    pub fn e_repodata_precheck_keyname(data: *mut Repodata, keyname: Id) -> c_int;
    pub fn e_repodata_has_keyname(data: *mut Repodata, kename: Id) -> c_int;
}
