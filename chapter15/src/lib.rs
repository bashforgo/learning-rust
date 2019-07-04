use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: String,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        let leaf = Rc::new(Node {
            value: String::from("lmao"),
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        });

        let branch = Rc::new(Node {
            value: String::from("ayy"),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("{:?}", branch);
    }
}
