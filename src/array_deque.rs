pub struct ArrayDeque<T> {
    a: Box<[T]>,
    j: usize,
    n: usize,
}

impl<T: Default> ArrayDeque<T> {
    pub fn new() -> Self {
        Self {
            a: Box::new([Default::default(); 1]),
            j: 0,
            n: 0,
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        if i >= self.n {
            return None;
        }
        Some(&self.a[(i + self.j) % self.a.len()])
    }

    pub fn set(&mut self, i: usize, x: T) -> Option<T> {
        if i >= self.n {
            return None;
        }
        let y = std::mem::replace(&mut self.a[(i + self.j) % self.a.len()], x);
        Some(y)
    }

    pub fn add(&mut self, i: usize, x: T) {
        if self.n == self.a.len() {
            self.resize();
        }
        if i < self.n / 2 {
            self.j = (self.a.len() + self.j - 1) % self.a.len();
            if i > 0 {
                for k in 0..i - 1 {
                    self.a
                        .swap((self.j + k) % self.a.len(), (self.j + k + 1) % self.a.len());
                }
            }
        } else {
            for k in (i + 1..self.n + 1).rev() {
                self.a.swap(
                    (self.j + k) % self.a.len(),
                    (self.j + k - 1 + self.a.len()) % self.a.len(),
                );
            }
        }

        self.a[(self.j + i) % self.a.len()] = x;
        self.n += 1;
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        if self.get(i).is_none() {
            return None;
        }
        let x = std::mem::replace(&mut self.a[(self.j + i) % self.a.len()], Default::default());
        if i < self.n / 2 {
            for k in (1..i + 1).rev() {
                self.a.swap(
                    (self.j + k) % self.a.len(),
                    (self.j + k - 1 + self.a.len()) % self.a.len(),
                );
            }
            self.j = (self.j + 1) % self.a.len();
        } else {
            for k in i..self.n - 1 {
                self.a
                    .swap((self.j + k) % self.a.len(), (self.j + k + 1) % self.a.len());
            }
        }

        self.n -= 1;
        if self.a.len() >= 3 * self.n {
            self.resize();
        }
        Some(x)
    }

    fn resize(&mut self) {
        let new_a = std::iter::repeat_with(|| Default::default())
            .take(std::cmp::max(1, 2 * self.n))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let mut old_a = std::mem::replace(&mut self.a, new_a);
        old_a.rotate_left(self.j);
        for (i, ai) in old_a.into_vec().into_iter().take(self.n).enumerate() {
            self.a[i] = ai;
        }
        self.j = 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn array_deque_works() {
        let mut array_deque = ArrayDeque::new();

        assert_eq!(array_deque.remove(0), None);

        array_deque.add(0, 3);
        // deque: [3]
        assert_eq!(array_deque.get(0), Some(&3));

        array_deque.add(1, 5);
        array_deque.add(0, 2);
        // deque: [2, 3, 5]
        assert_eq!(array_deque.get(0), Some(&2));
        assert_eq!(array_deque.get(1), Some(&3));
        assert_eq!(array_deque.get(2), Some(&5));

        array_deque.add(2, 6);
        array_deque.add(0, 8);
        // deque: [8, 2, 3, 6, 5]
        assert_eq!(array_deque.get(0), Some(&8));
        assert_eq!(array_deque.get(1), Some(&2));
        assert_eq!(array_deque.get(2), Some(&3));
        assert_eq!(array_deque.get(3), Some(&6));
        assert_eq!(array_deque.get(4), Some(&5));

        assert_eq!(array_deque.set(1, 7), Some(2));
        // deque: [8, 7, 3, 6, 5]
        assert_eq!(array_deque.get(1), Some(&7));
        assert_eq!(array_deque.remove(2), Some(3));
        // deque: [8, 7, 6, 5]
        assert_eq!(array_deque.remove(0), Some(8));
        // deque: [7, 6, 5]
        assert_eq!(array_deque.remove(2), Some(5));
        // deque: [7, 6]
        assert_eq!(array_deque.remove(0), Some(7));
        assert_eq!(array_deque.remove(0), Some(6));
        // deque: []
        assert_eq!(array_deque.remove(0), None);

        array_deque.add(0, 10);
        assert_eq!(array_deque.get(0), Some(&10));
    }
}
