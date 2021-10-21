use std::rc::Rc;

struct Node<T> {
    val: T,
    prev: Option<Rc<Node<T>>>,
    next: Option<Rc<Node<T>>>,
}

trait One {
    fn one() -> Self;
}

struct Placeholder {}

impl One for Placeholder {
    fn one() -> Self {
        Placeholder {}
    }
}

struct List<T> {
    size: usize,
    head: Node<T>,
    tail: Node<T>,
}

impl<T: One> List<T> {
    fn new() -> Self {
        // 边界指针
        let mut h = Node {
            val: T::one(),
            prev: None,
            next: None,
        };
        let mut t = Node {
            val: T::one(),
            prev: None,
            next: None,
        };
        List {
            size: 0,
            head: h,
            tail: t,
        }
    }

    fn push_back(&mut self, val: T) {
    }
}