use std::fmt;

#[derive(PartialEq)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use List::*;

impl<T> List<T> {
    fn cons(v: T, rest: List<T>) -> List<T> {
        Cons(v, Box::new(rest))
    }

    fn append(self, other: List<T>) -> List<T> {
        if let Cons(v, rest) = self {
            List::cons(v, rest.append(other))
        } else {
            other
        }
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
    println!(
        "{:?}",
        cons("ayy", cons("lmao", Nil)).append(cons("such", cons("list", Nil)))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_nil() {
        let cons = List::cons;

        let list = cons(1, cons(2, cons(3, Nil)));
        let append_list = list.append(Nil);

        let expected = cons(1, cons(2, cons(3, Nil)));
        assert_eq!(expected, append_list);
    }

    #[test]
    fn append_list() {
        let cons = List::cons;

        let list = cons(1, cons(2, cons(3, Nil)));
        let other_list = cons(4, cons(5, Nil));
        let append_list = list.append(other_list);

        let expected = cons(1, cons(2, cons(3, cons(4, cons(5, Nil)))));
        assert_eq!(expected, append_list);
    }
}
