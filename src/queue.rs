use libsolv_sys::Queue as _Queue;
use libsolv_sys::Id;
use libc::c_int;
use std::{slice, fmt, mem};

pub struct Queue {
    pub(crate) _q: _Queue,
}

impl Queue {
    pub fn new() -> Self {
        use libsolv_sys::queue_init;
        let internal = unsafe {
            let mut internal: _Queue = mem::uninitialized();
            queue_init(&mut internal);
            internal
        };

        Queue{ _q: internal}
    }

    pub fn clear(&mut self) {
        use libsolv_sys::export::queue::e_queue_empty;
        unsafe{e_queue_empty(&mut self._q)};
    }
    pub fn len(&self) -> c_int {
        self._q.count
    }

    pub fn shift(&mut self) -> Id {
        use libsolv_sys::export::queue::e_queue_shift;
        unsafe{e_queue_shift(&mut self._q)}
    }

    pub fn pop(&mut self) -> Id {
        use libsolv_sys::export::queue::e_queue_pop;
        unsafe{e_queue_pop(&mut self._q)}
    }

    pub fn unshift(&mut self, id: Id) {
        use libsolv_sys::export::queue::e_queue_unshift;
        unsafe{e_queue_unshift(&mut self._q, id)}
    }

    pub fn push(&mut self, id: Id) {
        use libsolv_sys::export::queue::e_queue_push;
        unsafe{e_queue_push(&mut self._q, id)}
    }
    pub fn pushunique(&mut self, id1: Id) {
        use libsolv_sys::export::queue::e_queue_pushunique;
        unsafe {e_queue_pushunique(&mut self._q, id1)};
    }

    pub fn push2(&mut self, id1: Id, id2: Id) {
        use libsolv_sys::export::queue::e_queue_push2;
        unsafe {e_queue_push2(&mut self._q, id1, id2)};
    }

    pub fn truncate(&mut self, n: c_int) {
        use libsolv_sys::export::queue::e_queue_truncate;
        unsafe {e_queue_truncate(&mut self._q, n)};
    }

    pub fn insert(&mut self, pos: c_int, id: Id) {
        use libsolv_sys::queue_insert;
        unsafe {queue_insert(&mut self._q, pos, id)};
    }

    pub fn insert2(&mut self, pos: c_int, id1: Id, id2: Id) {
        use libsolv_sys::queue_insert2;
        unsafe {queue_insert2(&mut self._q, pos, id1, id2)};
    }

    pub fn insertn(&mut self, pos: c_int, elements: &mut [Id]) {
        use libsolv_sys::queue_insertn;
        let n = elements.len() as c_int;
        unsafe {queue_insertn(&mut self._q, pos, n, elements.as_mut_ptr())}
    }

    pub fn delete(&mut self, pos: c_int) {
        use libsolv_sys::queue_delete;
        unsafe {queue_delete(&mut self._q, pos)};
    }

    pub fn delete2(&mut self, pos: c_int) {
        use libsolv_sys::queue_delete2;
        unsafe {queue_delete2(&mut self._q, pos)};
    }

    pub fn deleten(&mut self, pos: c_int, n: c_int) {
        use libsolv_sys::queue_deleten;
        unsafe {queue_deleten(&mut self._q, pos, n)}
    }
}

impl Default for Queue {
    fn default() -> Self {
        Queue::new()
    }
}


impl Drop for Queue {
    fn drop(&mut self) {
        use libsolv_sys::queue_free;
        unsafe {queue_free(&mut self._q)}
    }
}

impl AsRef<[i32]> for Queue {
    fn as_ref(&self) -> &[i32] {
        unsafe {slice::from_raw_parts(self._q.elements, self._q.count as usize)}
    }
}

impl AsMut<[i32]> for Queue {
    fn as_mut(&mut self) -> &mut [i32] {
        unsafe {slice::from_raw_parts_mut(self._q.elements, self._q.count as usize)}
    }
}

impl fmt::Debug for Queue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Queue {{ elements: {:?}, elements_ptr: {:?}, count: {:?}, alloc_ptr: {:?}, left: {}}}",
               self.as_ref(),
               self._q.elements,
               self._q.count,
               self._q.alloc,
               self._q.left)
    }
}

#[cfg(test)]
mod tests {
    use queue::*;

    #[test]
    fn init_and_free() {
        let mut queue: Queue = Default::default();
        queue.push2(1, 2);
        println!("start: {:?}", queue);

        assert!(!queue._q.elements.is_null());
        assert!(!queue._q.alloc.is_null());

        assert_eq!(1, queue.shift());
        println!("shift: {:?}", queue);

        queue.clear();
        println!("empty: {:?}", queue);

    }
}