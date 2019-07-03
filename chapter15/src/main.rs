use std::fmt;
use std::rc::Rc;

#[derive(PartialEq)]
enum Element<T> {
    Cons(Rc<T>, List<T>),
    Nil,
}

type List<T> = Rc<Element<T>>;

trait Nil<T> {
    fn nil() -> List<T>;
}
impl<T> Nil<T> for List<T> {
    fn nil() -> List<T> {
        Rc::new(Element::Nil)
    }
}

trait Cons<T, V, R> {
    fn cons(v: V, rest: R) -> List<T>;
}

impl<T> Cons<T, T, &List<T>> for List<T> {
    fn cons(v: T, rest: &List<T>) -> List<T> {
        Rc::new(Element::Cons(Rc::new(v), Rc::clone(rest)))
    }
}

impl<T> Cons<T, T, List<T>> for List<T> {
    fn cons(v: T, rest: List<T>) -> List<T> {
        List::cons(v, &rest)
    }
}

impl<T> Cons<T, &Rc<T>, &List<T>> for List<T> {
    fn cons(v: &Rc<T>, rest: &List<T>) -> List<T> {
        Rc::new(Element::Cons(Rc::clone(v), Rc::clone(rest)))
    }
}

trait Append<T> {
    fn append(&self, rest: &List<T>) -> List<T>;
}

impl<T> Append<T> for List<T> {
    fn append(&self, other: &List<T>) -> List<T> {
        let element = &**self;
        if let Element::Cons(v, rest) = element {
            List::cons(v, &rest.append(other))
        } else {
            Rc::clone(other)
        }
    }
}

impl<T: fmt::Display> fmt::Debug for Element<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List[")?;

        let mut first = true;
        let mut current = self;
        loop {
            match current {
                Element::Cons(e, rest) => {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                    current = rest;
                }
                Element::Nil => {
                    break;
                }
            }
        }

        write!(f, "]")
    }
}

fn main() {
    let (cons, nil) = (List::cons, List::nil);
    println!("{:?}", cons(1, cons(2, cons(3, cons(4, nil())))));
    let (cons, nil) = (List::cons, List::nil);
    println!(
        "{:?}",
        cons("ayy", cons("lmao", cons("such", cons("list", nil()))))
    );
    println!(
        "{:?}",
        cons("ayy", cons("lmao", nil())).append(&cons("such", cons("list", nil())))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_nil() {
        let (cons, nil) = (List::cons, List::nil);

        let list = cons(1, cons(2, cons(3, nil())));
        let append_list = list.append(&nil());

        let expected = cons(1, cons(2, cons(3, nil())));
        assert_eq!(expected, append_list);
    }

    #[test]
    fn append_list() {
        let (cons, nil) = (List::cons, List::nil);

        let list = cons(1, cons(2, cons(3, nil())));
        let other_list = cons(4, cons(5, nil()));
        let append_list = list.append(&other_list);

        let expected = cons(1, cons(2, cons(3, cons(4, cons(5, nil())))));
        assert_eq!(expected, append_list);
    }

    #[test]
    fn multiple_owners() {
        let cons = List::cons;
        let empty: List<&str> = List::nil();
        assert_eq!(1, Rc::strong_count(&empty));

        let element = Rc::new("value");
        assert_eq!(1, Rc::strong_count(&element));

        {
            let list_a = cons(&element, &empty);
            assert_eq!(2, Rc::strong_count(&element));
            assert_eq!(2, Rc::strong_count(&empty));

            {
                let _list_b = cons(&element, &list_a);
                assert_eq!(3, Rc::strong_count(&element));
                assert_eq!(2, Rc::strong_count(&empty));
            }

            assert_eq!(2, Rc::strong_count(&element));
            assert_eq!(2, Rc::strong_count(&empty));
        }

        assert_eq!(1, Rc::strong_count(&element));
        assert_eq!(1, Rc::strong_count(&empty));
    }
}
