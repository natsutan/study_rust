use std::cell::RefCell;
use std::rc::Rc;
use core::borrow::Borrow;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
pub struct Node {
    value:String,
    next:SingleLink
}

pub struct TransactionLog {
    head:SingleLink,
    tail:SingleLink,
    pub length:u32
}

impl Node {
    fn new(value:String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next:None
        }))
    }
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head:None,
            tail:None,
            length:0
        }
    }

    pub fn append(&mut self, value:String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone())
        }
        self.length += 1;
        self.tail = Some(new);
    }

}

pub fn print_log(log: &TransactionLog) {
    match log.head.borrow() {
        Some(n) => print_node_str(n),
        None => ()
    }
    ()
}

pub fn print_node_str(node_rc: &Rc<RefCell<Node>>) {
    let node= node_rc.borrow_mut();
    println ! ("LOG:\"{}\"", &node.value);
    match &node.next {
        Some(n) => print_node_str(&n),
        None => ()
    }
}

fn main() {

    let mut log = TransactionLog::new_empty();
    log.append("test1".to_string());
    log.append("test2".to_string());
    print_log(&log);


}
