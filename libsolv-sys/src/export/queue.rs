use {Queue, Id};
use libc::c_int;

extern "C" {
    pub fn e_queue_empty(q: *mut Queue);
    pub fn e_queue_shift(q: *mut Queue) -> Id;
    pub fn e_queue_pop(q: *mut Queue) -> Id;
    pub fn e_queue_unshift(q: *mut Queue, id: Id);
    pub fn e_queue_push(q: *mut Queue, id: Id);
    pub fn e_queue_pushunique(q: *mut Queue, id: Id);
    pub fn e_queue_push2(q: *mut Queue, id1: Id, id2: Id);
    pub fn e_queue_truncate(q: *mut Queue, n: c_int);
}
