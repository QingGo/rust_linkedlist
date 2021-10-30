mod linked_list_safe;
mod linked_list_unsafe;

use linked_list_safe::LinkedList;
use linked_list_unsafe::LinkedListUnsafe;

trait List<T>: std::fmt::Debug {
    fn push(&mut self, elem: T);
    fn pop(&mut self) -> Option<T>;
}

fn main() {
    let mut lists: Vec<Box<dyn List<i32>>> = vec![
        Box::new(LinkedList::new()),
        Box::new(LinkedListUnsafe::new()),
    ];
    for list in lists.iter_mut() {
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
}
