use {Dirpool, Id};

extern "C" {
    pub fn e_dirpool_parent(dp: *const Dirpool, did: Id) -> Id;
    pub fn e_dirpool_sibling(dp: *mut Dirpool, did: Id) -> Id;
    pub fn e_dirpool_child(dp: *mut Dirpool, did: Id) -> Id;
    pub fn e_dirpool_free_dirtraverse(dp: *mut Dirpool);
    pub fn e_dirpool_compid(dp: *const Dirpool, did: Id) -> Id;
}
