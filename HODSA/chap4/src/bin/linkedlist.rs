use std::cell::RefCell;
use std::rc::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value:String,
    next:SingleLink
}

struct TransactionLog {
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



fn main() {
    println!("Hello, List!");

    let mut log = TransactionLog::new_empty();
    log.append("test1".to_string());
    log.append("test2".to_string());

    println!("{}", log.length);
}
