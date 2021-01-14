use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
struct SLList<T> {
    n: usize,
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
#[derive(Debug)]
struct Node<T> {
    x: T,
    next: Link<T>,
}

impl<T: Default> SLList<T> {
    pub fn new() -> Self {
        Self {
            n: 0,
            head: None,
            tail: None,
        }
    }
    pub fn push(&mut self, x: T) {
        let u = Node::new(x);
        u.borrow_mut().next = self.head.take();
        self.head = Some(Rc::clone(&u));
        if self.n == 0 {
            self.tail = Some(Rc::clone(&u));
        }
        self.n += 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(old_head) => {
                self.head = old_head.borrow_mut().next.take();
                self.n -= 1;
                if self.n == 0 {
                    self.tail = None;
                }
                Some(Rc::try_unwrap(old_head).ok().unwrap().into_inner().x)
            }
            None => None,
        }
    }

    pub fn remove(&mut self) -> Option<T> {
        self.pop()
    }

    pub fn add(&mut self, x: T) -> bool {
        let u = Node::new(x);
        match self.tail.take() {
            None => self.head = Some(Rc::clone(&u)),
            Some(tail) => {
                tail.borrow_mut().next = Some(Rc::clone(&u));
            }
        }
        self.tail = Some(Rc::clone(&u));
        self.n += 1;

        true
    }
}

impl<T: Default> Node<T> {
    fn new(x: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Self {
            x,
            next: Default::default(),
        }))
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sllist_works() {
        let mut sllist = SLList::new();
        assert_eq!(sllist.pop(), None);
        assert_eq!(sllist.remove(), None);
        sllist.push(1);
        sllist.push(2);
        sllist.push(3);
        sllist.add(4);
        println!("{:?}", sllist);
        // sllist: [3, 2, 1, 4]
        assert_eq!(sllist.pop(), Some(3));
        assert_eq!(sllist.pop(), Some(2));
        assert_eq!(sllist.pop(), Some(1));
        assert_eq!(sllist.pop(), Some(4));
        assert_eq!(sllist.pop(), None);
        sllist.add(5);
        assert_eq!(sllist.pop(), Some(5));
    }
}
