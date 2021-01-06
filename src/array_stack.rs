pub struct ArrayStack<T> {
    a: Box<[Option<T>]>,
    n: usize,
}

impl<T> ArrayStack<T> {
    pub fn new() -> Self {
        Self {
            a: Box::new([None; 1]),
            n: 0,
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        if i < self.n {
            self.a[i].as_ref()
        } else {
            None
        }
    }

    pub fn set(&mut self, i: usize, x: T) -> Option<T> {
        if i < self.n {
            self.a[i].replace(x)
        } else {
            None
        }
    }

    pub fn add(&mut self, i: usize, x: T) {
        if self.n == self.a.len() {
            self.resize();
        }

        self.a[i..self.n + 1].rotate_right(1);
        self.a[i] = Some(x);
        self.n += 1;
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        if self.n == 0 {
            return None;
        }
        let x = self.a[i].take();
        self.a[i..self.n].rotate_left(1);
        self.n -= 1;
        if self.a.len() >= 3 * self.n {
            self.resize();
        }
        x
    }

    pub fn size(&self) -> usize {
        self.n
    }

    fn resize(&mut self) {
        let mut new_a = std::iter::repeat_with(|| None)
            .take(std::cmp::max(1, 2 * self.n))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        for i in 0..self.n {
            new_a[i] = self.a[i].take();
        }
        self.a = new_a;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn array_stack_works() {
        let mut array_stack = ArrayStack::new();

        array_stack.add(0, 3);
        array_stack.add(1, 5);
        // a:[3, 5]
        assert_eq!(array_stack.get(0), Some(&3));
        assert_eq!(array_stack.get(1), Some(&5));
        assert_eq!(array_stack.n, 2);
        assert_eq!(array_stack.a.len(), 2);

        assert_eq!(array_stack.set(1, 2), Some(5));
        // a:[3, 2]
        assert_eq!(array_stack.get(1), Some(&2));

        array_stack.add(0, -1);
        // a:[-1, 3, 2]
        assert_eq!(array_stack.get(0), Some(&-1));
        assert_eq!(array_stack.get(1), Some(&3));
        assert_eq!(array_stack.get(2), Some(&2));
        assert_eq!(array_stack.n, 3);
        assert_eq!(array_stack.a.len(), 4);

        array_stack.add(2, 5);
        // a:[-1, 3, 5, 2]
        assert_eq!(array_stack.get(0), Some(&-1));
        assert_eq!(array_stack.get(1), Some(&3));
        assert_eq!(array_stack.get(2), Some(&5));
        assert_eq!(array_stack.get(3), Some(&2));
        assert_eq!(array_stack.n, 4);
        assert_eq!(array_stack.a.len(), 4);

        array_stack.add(4, 10);
        // a:[-1, 3, 5, 2, 10]
        assert_eq!(array_stack.get(4), Some(&10));
        assert_eq!(array_stack.n, 5);
        assert_eq!(array_stack.a.len(), 8);

        assert_eq!(array_stack.remove(3), Some(2));
        // a:[-1, 3, 5, 10]

        assert_eq!(array_stack.remove(1), Some(3));
        // a:[-1, 5, 10]
        assert_eq!(array_stack.n, 3);
        assert_eq!(array_stack.a.len(), 8);

        assert_eq!(array_stack.remove(2), Some(10));
        // a:[-1, 5]
        assert_eq!(array_stack.n, 2);
        assert_eq!(array_stack.a.len(), 4);

        assert_eq!(array_stack.remove(0), Some(-1));
        assert_eq!(array_stack.remove(0), Some(5));
        // a:[]
        assert_eq!(array_stack.get(0), None);

        array_stack.add(0, 7);
        // a:[7]
        assert_eq!(array_stack.get(0), Some(&7));
    }
}
