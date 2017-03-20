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
    pub fn queue_init_clone(t: *mut Queue, s: *mut Queue); // FIXME: source is const? - Yes
    pub fn queue_free(q: *mut Queue);

    pub fn queue_alloc_one(q: *mut Queue); /* internal */
    pub fn queue_alloc_one_head(q: *mut Queue); /* internal */

    pub fn queue_insert(q: *mut Queue, pos: c_int, id: Id);
    pub fn queue_insert2(q: *mut Queue, pos: c_int, id1: Id, id2: Id);
    pub fn queue_insertn(q: *mut Queue, pos: c_int, n: c_int, elements: *mut Id); // FIXME: elements is const? - Yes.
    pub fn queue_delete(q: *mut Queue, pos: c_int);
    pub fn queue_delete2(q: *mut Queue, pos: c_int);
    pub fn queue_deleten(q: *mut Queue, pos: c_int, n: c_int);
    pub fn queue_prealloc(q: *mut Queue, n: c_int);

}

pub unsafe fn queue_empty(q: *mut Queue) {
    let ref mut queue = *q;
    if !queue.alloc.is_null() {
        queue.left += (queue.elements as usize - queue.alloc as usize) as c_int + queue.count;
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
    *queue.elements.offset(-1) = id;
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
        queue_push(q, id);
    }
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
