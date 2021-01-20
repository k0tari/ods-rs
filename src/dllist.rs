use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
#[derive(Debug)]
struct DLList<T> {
    n: usize,
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WLink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug, Default)]
struct Node<T> {
    x: T,
    next: Link<T>,
    prev: WLink<T>,
}

impl<T: Default + Clone> DLList<T> {
    pub fn new() -> Self {
        let head = Node::new(Default::default());
        let tail = Node::new(Default::default());
        head.borrow_mut().prev = None;
        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().next = None;
        tail.borrow_mut().prev = Some(Rc::downgrade(&head));
        Self {
            n: 0,
            head: Some(head),
            tail: Some(tail),
        }
    }
    fn get_node(&self, i: usize) -> Rc<RefCell<Node<T>>> {
        let mut p;
        if i < self.n / 2 {
            p = Rc::clone(self.head.as_ref().unwrap().borrow().next.as_ref().unwrap());
            for _ in 0..i {
                let a = Rc::clone(p.borrow().next.as_ref().unwrap());
                p = a;
            }
        } else {
            p = Rc::clone(self.tail.as_ref().unwrap());
            for _ in 0..(self.n - i) {
                let a = Rc::clone(&p.borrow().prev.as_ref().unwrap().upgrade().unwrap());
                p = a;
            }
        }
        Rc::clone(&p)
    }
    pub fn get(&self, i: usize) -> Option<T> {
        if i >= self.n {
            return None;
        }
        Some(self.get_node(i).borrow().x.clone())
    }
    pub fn set(&self, i: usize, x: T) -> Option<T> {
        if i >= self.n {
            return None;
        }
        let u = self.get_node(i);
        let y = u.borrow().x.clone();
        u.borrow_mut().x = x;
        Some(y)
    }
    fn add_before(&mut self, w: Rc<RefCell<Node<T>>>, x: T) {
        let u = Node::new(x);
        u.borrow_mut().prev = Some(Rc::downgrade(
            &w.borrow().prev.as_ref().unwrap().upgrade().unwrap(),
        ));
        u.borrow_mut().next = Some(Rc::clone(&w));
        w.borrow_mut().prev = Some(Rc::downgrade(&u));
        u.borrow()
            .prev
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
            .borrow_mut()
            .next = Some(Rc::clone(&u));
        self.n += 1;
    }

    pub fn add(&mut self, i: usize, x: T) {
        self.add_before(self.get_node(i), x);
    }
    fn remove_node(&mut self, w: Rc<RefCell<Node<T>>>) {
        w.borrow()
            .prev
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
            .borrow_mut()
            .next = Some(Rc::clone(&w.borrow().next.as_ref().unwrap()));
        w.borrow().next.as_ref().unwrap().borrow_mut().prev =
            Some(w.borrow().prev.as_ref().unwrap().clone());
        self.n -= 1;
    }
    pub fn remove(&mut self, i: usize) {
        if i < self.n {
            self.remove_node(self.get_node(i));
        }
    }
}

impl<T: Default> Node<T> {
    fn new(x: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Self {
            x,
            prev: Default::default(),
            next: Default::default(),
        }))
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn dllist_works() {
        let mut dllist: DLList<i32> = DLList::new();

        assert_eq!(dllist.get(0), None);

        dllist.add(0, 3);
        dllist.add(1, 2);
        dllist.add(0, 5);
        dllist.add(1, 7);
        // dlist: [5, 7, 3, 2]
        assert_eq!(dllist.get(0), Some(5));
        assert_eq!(dllist.get(1), Some(7));
        assert_eq!(dllist.get(2), Some(3));
        assert_eq!(dllist.get(3), Some(2));

        dllist.set(1, 4);
        dllist.set(3, 10);
        // dlist: [5, 4, 3, 10]
        assert_eq!(dllist.get(1), Some(4));
        assert_eq!(dllist.get(3), Some(10));

        dllist.remove(2);
        // dlist: [5, 4, 10]
        assert_eq!(dllist.get(0), Some(5));
        assert_eq!(dllist.get(1), Some(4));
        assert_eq!(dllist.get(2), Some(10));

        dllist.remove(0);
        // dlist: [4, 10]
        assert_eq!(dllist.get(0), Some(4));
        assert_eq!(dllist.get(1), Some(10));

        dllist.remove(1);
        dllist.remove(0);
        assert_eq!(dllist.get(0), None);
    }
}
