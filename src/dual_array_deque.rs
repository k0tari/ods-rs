use crate::array_stack::ArrayStack;

pub struct DualArrayDeque<T> {
    front: ArrayStack<T>,
    back: ArrayStack<T>,
}

impl<T: Default + Clone> DualArrayDeque<T> {
    pub fn new() -> Self {
        Self {
            front: ArrayStack::new(),
            back: ArrayStack::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.front.size() + self.back.size()
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        if i < self.front.size() {
            self.front.get(self.front.size() - i - 1)
        } else {
            self.back.get(i - self.front.size())
        }
    }

    pub fn set(&mut self, i: usize, x: T) -> Option<T> {
        if i < self.front.size() {
            self.front.set(self.front.size() - i - 1, x)
        } else {
            self.back.set(i - self.front.size(), x)
        }
    }

    pub fn add(&mut self, i: usize, x: T) {
        if i < self.front.size() {
            self.front.add(self.front.size() - i, x);
        } else {
            self.back.add(i - self.front.size(), x);
        }
        self.balance();
    }

    pub fn balance(&mut self) {
        let n = self.size();
        let mid = n / 2;
        if 3 * self.front.size() < self.back.size() || 3 * self.back.size() < self.front.size() {
            let mut f = ArrayStack::new();
            for i in 0..mid {
                f.add(i, self.get(mid - i - 1).unwrap().clone());
            }
            let mut b = ArrayStack::new();
            for i in 0..n - mid {
                b.add(i, self.get(mid + i).unwrap().clone());
            }
            self.front = f;
            self.back = b;
        }
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        let x = if i < self.front.size() {
            self.front.remove(self.front.size() - i - 1)
        } else {
            self.back.remove(i - self.front.size())
        };
        self.balance();
        x
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn dual_array_deque_works() {
        let mut dual_array_deque = DualArrayDeque::new();

        assert_eq!(dual_array_deque.remove(0), None);

        dual_array_deque.add(0, 3);
        // deque: [3]
        assert_eq!(dual_array_deque.get(0), Some(&3));

        dual_array_deque.add(1, 5);
        dual_array_deque.add(0, 2);
        // deque: [2, 3, 5]
        assert_eq!(dual_array_deque.get(0), Some(&2));
        assert_eq!(dual_array_deque.get(1), Some(&3));
        assert_eq!(dual_array_deque.get(2), Some(&5));

        dual_array_deque.add(2, 6);
        dual_array_deque.add(0, 8);
        // deque: [8, 2, 3, 6, 5]
        assert_eq!(dual_array_deque.get(0), Some(&8));
        assert_eq!(dual_array_deque.get(1), Some(&2));
        assert_eq!(dual_array_deque.get(2), Some(&3));
        assert_eq!(dual_array_deque.get(3), Some(&6));
        assert_eq!(dual_array_deque.get(4), Some(&5));

        assert_eq!(dual_array_deque.set(1, 7), Some(2));
        // deque: [8, 7, 3, 6, 5]
        assert_eq!(dual_array_deque.get(1), Some(&7));
        assert_eq!(dual_array_deque.remove(2), Some(3));
        // deque: [8, 7, 6, 5]
        assert_eq!(dual_array_deque.remove(0), Some(8));
        // deque: [7, 6, 5]
        assert_eq!(dual_array_deque.remove(2), Some(5));
        // deque: [7, 6]
        assert_eq!(dual_array_deque.remove(0), Some(7));
        assert_eq!(dual_array_deque.remove(0), Some(6));
        // deque: []
        assert_eq!(dual_array_deque.remove(0), None);

        dual_array_deque.add(0, 10);
        assert_eq!(dual_array_deque.get(0), Some(&10));
    }
}
