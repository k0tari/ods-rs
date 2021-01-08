use crate::array_stack::ArrayStack;

#[derive(Debug)]
pub struct RootishArrayStack<T> {
    n: usize,
    blocks: ArrayStack<Box<[Option<T>]>>,
}

impl<T: Default + Clone> RootishArrayStack<T> {
    pub fn new() -> Self {
        Self {
            n: 0,
            blocks: ArrayStack::new(),
        }
    }

    fn i2b(i: usize) -> usize {
        ((-3.0 + (9.0 + 8.0 * (i as f64)).sqrt()) / 2.0).ceil() as usize
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        let b = RootishArrayStack::<T>::i2b(i);
        let j = i - b * (b + 1) / 2;
        match self.blocks.get(b) {
            Some(block) => block[j].as_ref(),
            None => None,
        }
    }

    pub fn set(&mut self, i: usize, x: T) -> Option<T> {
        let b = RootishArrayStack::<T>::i2b(i);
        let j = i - b * (b + 1) / 2;
        match self.blocks.get_mut(b) {
            Some(block) => {
                let y = block[j].take();
                block[j] = Some(x);
                y
            }
            None => None,
        }
    }

    pub fn add(&mut self, i: usize, x: T) {
        let r = self.blocks.size();
        if r * (r + 1) / 2 < self.n + 1 {
            self.grow();
        }
        self.n += 1;
        for j in (i + 1..self.n).rev() {
            if let Some(x) = self.get(j - 1) {
                let x = x.clone();
                self.set(j, x);
            }
        }
        self.set(i, x);
    }

    fn grow(&mut self) {
        let block_size = self.blocks.size();
        let block = std::iter::repeat_with(|| None)
            .take(block_size + 1)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        self.blocks.add(block_size, block);
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        if self.n == 0 {
            return None;
        }
        let x = self.get(i).map(|x| x.clone())?;
        for j in i..self.n - 1 {
            if let Some(x) = self.get(j + 1) {
                let x = x.clone();
                self.set(j, x);
            }
        }
        self.n -= 1;
        let r = self.blocks.size();
        if r.saturating_sub(2) * r.saturating_sub(1) / 2 >= self.n {
            self.shrink();
        }
        Some(x)
    }

    fn shrink(&mut self) {
        let mut r = self.blocks.size();
        while r > 0 && r.saturating_sub(2) * r.saturating_sub(1) >= self.n {
            self.blocks.remove(self.blocks.size() - 1);
            r -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn rootish_array_stack_works() {
        let mut rootish_array_stack = RootishArrayStack::new();

        assert_eq!(rootish_array_stack.remove(0), None);

        rootish_array_stack.add(0, 3);
        // stack: [3]
        assert_eq!(rootish_array_stack.get(0), Some(&3));

        rootish_array_stack.add(1, 5);
        rootish_array_stack.add(0, 2);
        // stack: [2, 3, 5]
        assert_eq!(rootish_array_stack.get(0), Some(&2));
        assert_eq!(rootish_array_stack.get(1), Some(&3));
        assert_eq!(rootish_array_stack.get(2), Some(&5));

        rootish_array_stack.add(2, 6);
        rootish_array_stack.add(0, 8);
        // stack: [8, 2, 3, 6, 5]
        assert_eq!(rootish_array_stack.get(0), Some(&8));
        assert_eq!(rootish_array_stack.get(1), Some(&2));
        assert_eq!(rootish_array_stack.get(2), Some(&3));
        assert_eq!(rootish_array_stack.get(3), Some(&6));
        assert_eq!(rootish_array_stack.get(4), Some(&5));

        assert_eq!(rootish_array_stack.set(1, 7), Some(2));
        // stack: [8, 7, 3, 6, 5]
        assert_eq!(rootish_array_stack.get(1), Some(&7));
        assert_eq!(rootish_array_stack.remove(2), Some(3));
        // stack: [8, 7, 6, 5]
        assert_eq!(rootish_array_stack.remove(0), Some(8));
        // stack: [7, 6, 5]
        assert_eq!(rootish_array_stack.remove(2), Some(5));
        // stack: [7, 6]
        assert_eq!(rootish_array_stack.remove(0), Some(7));
        assert_eq!(rootish_array_stack.remove(0), Some(6));
        // stack: []
        assert_eq!(rootish_array_stack.remove(0), None);

        rootish_array_stack.add(0, 10);
        assert_eq!(rootish_array_stack.get(0), Some(&10));
    }
}
