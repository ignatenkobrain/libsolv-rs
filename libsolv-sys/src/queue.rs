use super::{Queue, Id};
use libc::c_int;

extern "C" {
    fn queue_empty_static(q: *mut Queue);
    fn queue_shift_static(q: *mut Queue) -> Id;
    fn queue_pop_static(q: *mut Queue) -> Id;
    fn queue_unshift_static(q: *mut Queue, id: Id);
    fn queue_push_static(q: *mut Queue, id: Id);
    fn queue_pushunique_static(q: *mut Queue, id: Id);
    fn queue_push2_static(q: *mut Queue, id1: Id, id2: Id);
    fn queue_truncate_static(q: *mut Queue, n: c_int);
}

pub fn queue_empty(q: *mut Queue) {
    unsafe { queue_empty_static(q) }
}
pub fn queue_shift(q: *mut Queue) -> Id {
    unsafe { queue_shift_static(q) }
}
pub fn queue_pop(q: *mut Queue) -> Id {
    unsafe { queue_pop_static(q) }
}
pub fn queue_unshift(q: *mut Queue, id: Id) {
    unsafe { queue_unshift_static(q, id) }
}
pub fn queue_push(q: *mut Queue, id: Id) {
    unsafe { queue_push_static(q, id) }
}
pub fn queue_pushunique(q: *mut Queue, id: Id) {
    unsafe { queue_pushunique_static(q, id) }
}
pub fn queue_push2(q: *mut Queue, id1: Id, id2: Id) {
    unsafe { queue_push2_static(q, id1, id2) }
}
pub fn queue_truncate(q: *mut Queue, n: c_int) {
    unsafe { queue_truncate_static(q, n) }
}