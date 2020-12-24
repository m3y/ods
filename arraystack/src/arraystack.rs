use interface::list::List;

#[derive(Clone, Debug, Default)]
pub struct Array<T> {
    a: Box<[Option<T>]>,
    n: usize,
}

impl<T> Array<T> {
    pub fn capacity(&self) -> usize {
        self.a.len()
    }

    pub fn new() -> Self {
        Self::with_length(1)
    }

    pub fn with_length(capacity: usize) -> Self {
        Self {
            a: Self::allocate_in_heap(capacity),
            n: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[Option<T>]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn resize(&mut self) {
        let new_a = Self::allocate_in_heap(std::cmp::max(self.n * 2, 1));
        let old_a = std::mem::replace(&mut self.a, new_a);
        for (i, elem) in old_a.into_vec().into_iter().enumerate().take(self.n) {
            self.a[i] = elem;
        }
    }
}

impl<T: Clone> List<T> for Array<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, i: usize) -> Option<T> {
        self.a.get(i)?.as_ref().cloned()
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        self.a.get_mut(i)?.replace(x)
    }

    fn add(&mut self, i: usize, x: T) {
        if self.n + 1 > self.capacity() {
            self.resize()
        }

        if i >= self.n {
            self.a[i] = Some(x)
        } else {
            self.a[i..self.n].rotate_right(1);
            let end = self.a[i].replace(x);
            self.a[self.n] = end;
        }
        self.n += 1;
    }

    fn remove(&mut self, i: usize) {
        if i < self.n {
            self.a[i..self.n].rotate_left(1);
        }
        self.n -= 1;
        if self.capacity() >= 3 * self.n {
            self.resize();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use interface::list::List;

    #[test]
    fn it_works() {
        let mut arraystack: Array<&str> = Array::new();
        assert_eq!(arraystack.capacity(), 1);
        assert_eq!(arraystack.get(0), None);

        arraystack.set(0, "b");
        assert_eq!(arraystack.get(0), Some("b"));

        let actual = arraystack.set(0, "c");
        assert_eq!(actual, Some("b"));
    }

    #[test]
    fn test_arraystack() {
        let mut arraystack: Array<&str> = Array::with_length(6);
        assert_eq!(arraystack.capacity(), 6);

        arraystack.add(0, "b");
        arraystack.add(1, "r");
        arraystack.add(2, "e");
        arraystack.add(3, "d");

        assert_eq!(arraystack.capacity(), 6);
        assert_eq!(arraystack.size(), 4);

        assert_eq!(arraystack.get(0), Some("b"));
        assert_eq!(arraystack.get(1), Some("r"));
        assert_eq!(arraystack.get(2), Some("e"));
        assert_eq!(arraystack.get(3), Some("d"));

        arraystack.add(2, "e");
        assert_eq!(arraystack.capacity(), 6);
        assert_eq!(arraystack.size(), 5);

        arraystack.add(5, "r");

        assert_eq!(arraystack.capacity(), 6);
        assert_eq!(arraystack.size(), 6);

        assert_eq!(arraystack.get(0), Some("b"));
        assert_eq!(arraystack.get(1), Some("r"));
        assert_eq!(arraystack.get(2), Some("e"));
        assert_eq!(arraystack.get(3), Some("e"));
        assert_eq!(arraystack.get(4), Some("d"));
        assert_eq!(arraystack.get(5), Some("r"));

        arraystack.add(5, "e");

        assert_eq!(arraystack.size(), 7);
        assert_eq!(arraystack.capacity(), 12);

        arraystack.remove(4);
        assert_eq!(arraystack.size(), 6);
        assert_eq!(arraystack.capacity(), 12);

        arraystack.remove(4);
        assert_eq!(arraystack.size(), 5);
        assert_eq!(arraystack.capacity(), 12);

        arraystack.remove(4);
        assert_eq!(arraystack.size(), 4);
        assert_eq!(arraystack.capacity(), 8);

        arraystack.remove(4);
        assert_eq!(arraystack.size(), 3);
        assert_eq!(arraystack.capacity(), 8);

        assert_eq!(arraystack.get(0), Some("b"));
        assert_eq!(arraystack.get(1), Some("r"));
        assert_eq!(arraystack.get(2), Some("e"));
    }
}
