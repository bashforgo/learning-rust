use std::fmt;

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use List::*;

impl<T> List<T> {
    fn cons(v: T, rest: List<T>) -> List<T> {
        Cons(v, Box::new(rest))
    }
}

impl<T: fmt::Display> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List[")?;

        let mut first = true;
        let mut current = self;
        loop {
            match current {
                Cons(e, rest) => {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                    current = rest;
                }
                Nil => {
                    break;
                }
            }
        }

        write!(f, "]")
    }
}

fn main() {
    let cons = List::cons;
    println!("{:?}", cons(1, cons(2, cons(3, cons(4, Nil)))));
    let cons = List::cons;
    println!(
        "{:?}",
        cons("ayy", cons("lmao", cons("such", cons("list", Nil))))
    );
}
