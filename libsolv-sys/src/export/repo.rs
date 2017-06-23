use {Solvable, Pool, Repo, Id};
use libc::{c_int, c_char};

extern "C" {
    pub fn e_repo_name(repo: *const Repo) -> *const c_char;
    pub fn e_pool_id2repo(pool: *mut Pool, repoid: Id) -> *mut Repo;
    pub fn e_pool_disabled_solvable(pool: *const Pool, s: *mut Solvable) -> c_int;
    pub fn e_pool_installable(pool: *const Pool, s: *mut Solvable) -> c_int;
}
