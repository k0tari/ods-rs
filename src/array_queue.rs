pub struct ArrayQueue<T> {
    a: Box<[T]>,
    j: usize,
    n: usize,
}

impl<T: Default> ArrayQueue<T> {
    pub fn new() -> Self {
        Self {
            a: Box::new([Default::default(); 1]),
            j: 0,
            n: 0,
        }
    }

    pub fn add(&mut self, x: T) -> bool {
        if self.n + 1 > self.a.len() {
            self.resize();
        }
        self.a[(self.j + self.n) % self.a.len()] = x;
        self.n += 1;

        true
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.n == 0 {
            return None;
        }
        let x = std::mem::replace(&mut self.a[self.j], Default::default());
        self.j = (self.j + 1) % self.a.len();
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
    fn array_queue_works() {
        let mut array_queue = ArrayQueue::new();

        assert_eq!(array_queue.remove(), None);
        array_queue.add(3);
        array_queue.add(1);
        array_queue.add(2);
        // queue: [3, 1, 2]
        assert_eq!(array_queue.remove(), Some(3));
        // queue: [1, 2]
        array_queue.add(4);
        array_queue.add(5);
        // queue: [1, 2, 4, 5]
        assert_eq!(array_queue.remove(), Some(1));
        assert_eq!(array_queue.remove(), Some(2));
        // queue: [4, 5]
        array_queue.add(10);
        array_queue.add(8);
        array_queue.add(2);
        // queue: [4, 5, 10, 8, 2]
        assert_eq!(array_queue.remove(), Some(4));
        assert_eq!(array_queue.remove(), Some(5));
        assert_eq!(array_queue.remove(), Some(10));
        assert_eq!(array_queue.remove(), Some(8));
        assert_eq!(array_queue.remove(), Some(2));
        assert_eq!(array_queue.remove(), None);
        // queue: []
        array_queue.add(7);
        // queue: [7]
        assert_eq!(array_queue.remove(), Some(7));
    }
}
