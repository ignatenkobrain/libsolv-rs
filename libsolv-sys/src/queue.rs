use libc::c_int;
use libsolv::Id;

#[repr(C)]
pub struct Queue {
    pub elements: *mut Id,
    pub count: c_int,
    pub alloc: *mut Id,
    pub left: c_int,
}

extern "C" {
    pub fn queue_init(q: *mut Queue);
    pub fn queue_init_buffer(q: *mut Queue, buf: *mut Id, size: c_int);
    pub fn queue_init_clone(t: *mut Queue, s: *mut Queue); // FIXME: source is const?
    pub fn queue_free(q: *mut Queue);

    pub fn queue_insert(q: *mut Queue, pos: c_int, id: Id);
    pub fn queue_insert2(q: *mut Queue, pos: c_int, id1: Id, id2: Id);
    pub fn queue_insertn(q: *mut Queue, pos: c_int, n: c_int, elements: *mut Id); // FIXME: elements is const?
    pub fn queue_delete(q: *mut Queue, pos: c_int);
    pub fn queue_delete2(q: *mut Queue, pos: c_int);
    pub fn queue_deleten(q: *mut Queue, pos: c_int, n: c_int);
    pub fn queue_prealloc(q: *mut Queue, n: c_int);

    fn queue_empty_real(q: *mut Queue);
    fn queue_shift_real(q: *mut Queue) -> Id;
    fn queue_pop_real(q: *mut Queue) -> Id;
    fn queue_unshift_real(q: *mut Queue, id: Id);
    fn queue_push_real(q: *mut Queue, id: Id);
    fn queue_pushunique_real(q: *mut Queue, id: Id);
    fn queue_push2_real(q: *mut Queue, id1: Id, id2: Id);
    fn queue_truncate_real(q: *mut Queue, n: c_int);
}

pub fn queue_empty(q: *mut Queue) {
    unsafe { queue_empty_real(q) }
}
pub fn queue_shift(q: *mut Queue) -> Id {
    unsafe { queue_shift_real(q) }
}
pub fn queue_pop(q: *mut Queue) -> Id {
    unsafe { queue_pop_real(q) }
}
pub fn queue_unshift(q: *mut Queue, id: Id) {
    unsafe { queue_unshift_real(q, id) }
}
pub fn queue_push(q: *mut Queue, id: Id) {
    unsafe { queue_push_real(q, id) }
}
pub fn queue_pushunique(q: *mut Queue, id: Id) {
    unsafe { queue_pushunique_real(q, id) }
}
pub fn queue_push2(q: *mut Queue, id1: Id, id2: Id) {
    unsafe { queue_push2_real(q, id1, id2) }
}
pub fn queue_truncate(q: *mut Queue, n: c_int) {
    unsafe { queue_truncate_real(q, n) }
}
