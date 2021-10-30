use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct LinkedList<T: Copy> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
struct Node<T: Copy> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Copy> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

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
                new_node_rc.borrow_mut().next = Some(Rc::clone(&head));
                self.head = Some(new_node_rc);
            }
        }
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(head) => {
                match head.borrow_mut().next.take() {
                    None => self.head = None,
                    Some(next) => self.head = Some(next),
                }
                self.tail = None;
                Some(head.borrow_mut().data)
            }
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);
    println!("{:?}", list);
    println!("{:?}", list.pop());
    println!("{:?}", list);
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
}
