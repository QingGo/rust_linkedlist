use crate::List;
use std::ptr;

#[derive(Debug)]
pub struct LinkedListUnsafe<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> LinkedListUnsafe<T> {
    pub fn new() -> Self {
        LinkedListUnsafe {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }
}

impl<T: std::fmt::Debug> List<T> for LinkedListUnsafe<T> {
    fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            data: elem,
            next: ptr::null_mut(),
        });

        let raw_ptr = Box::into_raw(new_node);

        if self.head.is_null() {
            self.head = raw_ptr;
        } else {
            unsafe {
                (*self.tail).next = raw_ptr;
            }
        }

        self.tail = raw_ptr;
    }

    fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }

        let raw_ptr = self.head;
        self.head = unsafe { (*raw_ptr).next };

        if self.head.is_null() {
            self.tail = ptr::null_mut();
        }

        unsafe {
            let node = Box::from_raw(raw_ptr);
            Some(node.data)
        }
    }
}
