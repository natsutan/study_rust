// https://github.com/PacktPublishing/Hands-On-Data-Structures-and-Algorithms-with-Rust/blob/master/Chapter04/src/doubly_linked_list.rs

use std::cell::{Ref, RefCell};
use std::rc::Rc;
//use alloc::vec::IntoIter;


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

impl IntoIterator for BetterTransactionLog {
    type Item = String;
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head)
    }
}


impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;

        self.current = match current{
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            },
            None => None
        };
        result
    }
}

impl Node {
    fn new(value:String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next:None,
            prev:None
        }))
    }
}

impl BetterTransactionLog {
    pub fn new_empty() -> BetterTransactionLog {
        BetterTransactionLog {
            head:None,
            tail:None,
            length:0
        }
    }

    pub fn append(&mut self, value:String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new.clone());
                new.borrow_mut().prev = Some(old);
            },
            None => self.head = Some(new.clone())
        }
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map( |head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head).ok().expect("Something is terribly wrong").into_inner().value
        }
        )
    }
    pub fn back_iter(self) -> ListIterator {
        ListIterator::new(self.tail)
    }

    pub fn iter(&self) -> ListIterator {
        ListIterator::new(self.head.clone())
    }
}


fn main() {
    println!("double linked");
    let mut log = BetterTransactionLog::new_empty();

    log.append("test1".to_string());
    log.append("test2".to_string());
    log.append("test3".to_string());

    let mut iter = log.clone().into_iter();
    for l in iter {
        println!("{}", l)
    }

    let mut biter = log.clone().back_iter();
    for l in biter {
        println!("{}", l)
    }

}

