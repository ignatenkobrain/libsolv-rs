use libc::c_int;
use std::{slice, fmt, mem};
use libsolv::Id;

#[repr(C)]
pub struct Queue {
    pub elements: *mut Id,
    pub count: c_int,
    pub alloc: *mut Id,
    pub left: c_int,
}

#[test]
fn bindgen_test_layout_Queue() {
    assert_eq!(::core::mem::size_of::<Queue>() , 32usize , concat ! (
               "Size of: " , stringify ! ( Queue ) ));
    assert_eq! (::core::mem::align_of::<Queue>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( Queue ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const Queue ) ) . elements as * const _ as
            usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( Queue ) , "::" ,
                stringify ! ( elements ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const Queue ) ) . count as * const _ as usize
    } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( Queue ) , "::" ,
                stringify ! ( count ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const Queue ) ) . alloc as * const _ as usize
    } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( Queue ) , "::" ,
                stringify ! ( alloc ) ));
    assert_eq! (unsafe {
        & ( * ( 0 as * const Queue ) ) . left as * const _ as usize }
    , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( Queue ) , "::" ,
                stringify ! ( left ) ));
}

impl Default for Queue {
    fn default() -> Self {
        let mut queue;
        unsafe {
            queue = mem::uninitialized();
            queue_init(&mut queue);
        }
        queue
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {queue_free(self)}
    }
}

impl AsRef<[i32]> for Queue {
    fn as_ref(&self) -> &[i32] {
        unsafe {slice::from_raw_parts(self.elements, self.count as usize)}
    }
}

impl AsMut<[i32]> for Queue {
    fn as_mut(&mut self) -> &mut [i32] {
        unsafe {slice::from_raw_parts_mut(self.elements, self.count as usize)}
    }
}

impl fmt::Debug for Queue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Queue {{ elements: {:?}, elements_ptr: {:?}, count: {:?}, alloc_ptr: {:?}, left: {}}}",
               self.as_ref(),
               self.elements,
               self.count,
               self.alloc,
               self.left)
    }
}


extern "C" {
    pub fn queue_init(q: *mut Queue);
    pub fn queue_init_buffer(q: *mut Queue, buf: *mut Id, size: c_int);
    pub fn queue_init_clone(t: *mut Queue, s: *mut Queue);
    pub fn queue_free(q: *mut Queue);

    pub fn queue_alloc_one(q: *mut Queue); /* internal */
    pub fn queue_alloc_one_head(q: *mut Queue); /* internal */

    pub fn queue_insert(q: *mut Queue, pos: c_int, id: Id);
    pub fn queue_insert2(q: *mut Queue, pos: c_int, id1: Id, id2: Id);
    pub fn queue_insertn(q: *mut Queue, pos: c_int, n: c_int, elements: *mut Id);
    pub fn queue_delete(q: *mut Queue, pos: c_int);
    pub fn queue_delete2(q: *mut Queue, pos: c_int);
    pub fn queue_deleten(q: *mut Queue, pos: c_int, n: c_int);
    pub fn queue_prealloc(q: *mut Queue, n: c_int);

}

pub unsafe fn queue_empty(q: *mut Queue) {
    let ref mut queue = *q;
    if !queue.alloc.is_null() {
        let unused = (queue.elements as usize - queue.alloc as usize) as c_int / mem::size_of::<Id>() as c_int;
        queue.left += unused + queue.count;
        queue.elements = queue.alloc;
    } else {
        queue.left += queue.count;
    }
    queue.count = 0;
}

pub unsafe fn queue_shift(q: *mut Queue) -> Id {
    let ref mut queue = *q;
    match queue.count {
        0 => 0,
        _ => {
            queue.count -= 1;
            let element = *queue.elements;
            queue.elements = queue.elements.offset(1);
            element
        }
    }
}

pub unsafe fn queue_pop(q: *mut Queue) -> Id {
    let ref mut queue = *q;
    match queue.count {
        0 => 0,
        _ => {
            queue.left += 1;
            queue.count -= 1;
            *queue.elements.offset(queue.count as isize)
        }
    }
}

pub unsafe fn queue_unshift(q: *mut Queue, id: Id) {
    let ref mut queue = *q;
    if queue.alloc.is_null() || queue.alloc == queue.elements {
        queue_alloc_one_head(q);
    }
    queue.elements = queue.elements.offset(-1);
    *queue.elements = id;
    queue.count += 1;
}
pub unsafe fn queue_push(q: *mut Queue, id: Id) {
    let ref mut queue = *q;
    if queue.left == 0 {
        queue_alloc_one(q);
    }
    *queue.elements.offset(queue.count as isize) = id;
    queue.count += 1;
    queue.left -= 1;
}
pub unsafe fn queue_pushunique(q: *mut Queue, id: Id) {
    let ref mut queue = *q;
    for i in (0..queue.count).rev() {
        if *queue.elements.offset(i as isize) == id {
            return;
        }
    }
    queue_push(q, id);
}
pub unsafe fn queue_push2(q: *mut Queue, id1: Id, id2: Id) {
    queue_push(q, id1);
    queue_push(q, id2);
}

pub unsafe fn queue_truncate(q: *mut Queue, n: c_int) {
    let ref mut queue = *q;
    if queue.count > n {
        queue.left += queue.count - n;
        queue.count = n;
    }
}


#[cfg(test)]
mod tests {
    use queue::*;

    #[test]
    fn init_and_free() {
        let mut queue: Queue = Default::default();
        unsafe {
            queue_init(&mut queue);
            queue_insert2(&mut queue, 0, 1, 2);
            assert!(!queue.elements.is_null());
            assert!(!queue.alloc.is_null());

            println!("start: {:?}", queue);

            assert_eq!(1, queue_shift(&mut queue));
            println!("shift: {:?}", queue);
            queue_empty(&mut queue);

            println!("empty: {:?}", queue);

            queue_free(&mut queue);

            println!("free: {:?}", queue);

            assert!(queue.elements.is_null());
            assert!(queue.alloc.is_null());

        }
    }
}
