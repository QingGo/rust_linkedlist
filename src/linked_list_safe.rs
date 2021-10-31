use std::cell::RefCell;
use std::rc::Rc;

use crate::List;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }
}

impl<T: std::fmt::Debug> List<T> for LinkedList<T> {
    fn push(&mut self, elem: T) {
        let new_node = Node {
            data: elem,
            next: None,
        };

        let new_node_rc = Rc::new(RefCell::new(new_node));
        match self.head {
            None => {
                self.head = Some(Rc::clone(&new_node_rc));
                self.tail = Some(Rc::clone(&new_node_rc));
            }
            Some(ref mut head) => {
                new_node_rc.borrow_mut().next = Some(Rc::clone(head));
                self.head = Some(new_node_rc);
            }
        }
    }

    fn pop(&mut self) -> Option<T> {
        let old_head = match self.head.take() {
            None => None,
            Some(head) => {
                match head.borrow_mut().next.take() {
                    None => self.head = None,
                    Some(next) => self.head = Some(next),
                }
                self.tail = None;
                Some(head)
            }
        };
        // Option<Rc<RefCell<Node<T>>>> -> Option<T>
        old_head.map(|r| Rc::try_unwrap(r).ok().unwrap().into_inner().data)
    }
}
