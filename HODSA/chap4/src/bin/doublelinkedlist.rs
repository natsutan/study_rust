
use std::cell::{Ref, RefCell};
use std::rc::Rc;


type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone)]
pub struct Node {
    value:String,
    next:Link,
    prev:Link
}

#[derive(Debug, Clone)]
pub struct BetterTransactionLog {
    head:Link,
    tail:Link,
    pub length:u64
}

pub struct ListIterator {
    current:Link,
}

impl ListIterator {
    fn new(start_at:Link) -> ListIterator {
        ListIterator { current: start_at}
    }
}

impl Iterator for ListIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            },
            None => None
        };
        result
    }
}


fn main() {
    println!("double linked");
}

