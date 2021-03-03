use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    x: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(x: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { x, next: None }))
    }
}

struct SLList<T> {
    head: Link<T>,
    tail: Link<T>,
    n: usize,
}

impl<T> SLList<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            n: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let _node: Node<i32> = Node::new(1);
        let _sllist: SLList<i32> = SLList::new();
        assert_eq!(2 + 2, 4);
    }
}
