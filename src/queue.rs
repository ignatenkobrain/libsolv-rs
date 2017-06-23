use libsolv_sys::Queue as _Queue;
use libsolv_sys::Id;
use libc::c_int;
use std::{slice, fmt, mem};

pub struct Queue {
    _q: _Queue,
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

    pub fn insert(&mut self, pos: c_int, id: Id) {
        use libsolv_sys::queue_insert;
        unsafe {queue_insert(&mut self._q, pos, id)};
    }

    pub fn insert2(&mut self, pos: c_int, id1: Id, id2: Id) {
        use libsolv_sys::queue_insert2;
        unsafe {queue_insert2(&mut self._q, pos, id1, id2)};
    }

    pub fn push2(&mut self, id1: Id, id2: Id) {
        use libsolv_sys::queue::queue_push2_static;
        unsafe {queue_push2_static(&mut self._q, id1, id2)};
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
        assert!(!queue._q.elements.is_null());
        assert!(!queue._q.alloc.is_null());
        /*
        unsafe {
            queue_init(&mut queue);
            queue_insert2(&mut queue, 0, 1, 2);


            println!("start: {:?}", queue);

            assert_eq!(1, queue_shift(&mut queue));
            println!("shift: {:?}", queue);
            queue_empty(&mut queue);

            println!("empty: {:?}", queue);

            queue_free(&mut queue);

            println!("free: {:?}", queue);

            assert!(queue._q.elements.is_null());
            assert!(queue._q.alloc.is_null());

        }*/
    }
}