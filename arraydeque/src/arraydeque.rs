use interface::list::List;

#[derive(Clone, Debug, Default)]
pub struct Array<T> {
    a: Box<[Option<T>]>,
    j: usize,
    n: usize,
}

impl<T> Array<T> {
    pub fn new() -> Self {
        Self::with_length(1)
    }

    pub fn with_length(capacity: usize) -> Self {
        Self {
            a: Self::allocate_in_heap(capacity),
            j: 0,
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
        let mut old_a = std::mem::replace(&mut self.a, new_a);
        for i in 0..self.n {
            self.a[i] = old_a[(self.j + i) % old_a.len()].take();
        }
        self.j = 0;
    }
}

impl<T: Clone> List<T> for Array<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, i: usize) -> Option<T> {
        self.a.get((self.j + i) % self.a.len())?.as_ref().cloned()
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        self.a.get_mut((self.j + i) % self.a.len())?.replace(x)
    }

    fn add(&mut self, i: usize, x: T) {
        if self.n + 1 > self.a.len() {
            self.resize();
        }

        if i < self.n / 2 {
            self.j = if self.j == 0 {
                self.a.len() - 1
            } else {
                self.j - 1
            };
            // left shift
            for k in 0..i {
                self.a[(self.j + k) % self.a.len()] =
                    self.a[(self.j + k + 1) % self.a.len()].take();
            }
        } else {
            // right shift
            for k in (i + 1..=self.n).rev() {
                self.a[(self.j + k) % self.a.len()] =
                    self.a[(self.j + k - 1) % self.a.len()].take();
            }
        }

        self.a[(self.j + i) % self.a.len()] = Some(x);
        self.n += 1;
    }

    fn remove(&mut self, i: usize) {
        let _x = self.a[(self.j + i) % self.a.len()].take();
        if i < self.n / 2 {
            // right shift
            for k in (1..=i).rev() {
                self.a[(self.j + k) % self.a.len()] =
                    self.a[(self.j + k - 1) % self.a.len()].take();
            }
            self.j = (self.j + 1) % self.a.len();
        } else {
            // left shift
            for k in i..self.n - 1 {
                self.a[(self.j + k) % self.a.len()] =
                    self.a[(self.j + k + 1) % self.a.len()].take();
            }
        }

        self.n -= 1;
        if 3 * self.n < self.a.len() {
            self.resize()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Array<&str> {
        /// for unittest
        fn rawget(&self, i: usize) -> Option<&str> {
            self.a.get(i)?.as_ref().cloned()
        }
    }

    #[test]
    fn it_works() {
        let _arraydeque: Array<&str> = Array::new();
        assert_eq!(2 + 2, 4);
    }

    fn assert_queue(queue: &Array<&str>, expected_list: Vec<Option<&str>>) {
        for (i, e) in expected_list.iter().enumerate() {
            assert_eq!(queue.rawget(i), *e);
        }
    }

    #[test]
    fn test_arraydeque() {
        let mut arraydeque: Array<&str> = Array::with_length(12);
        arraydeque.add(0, "a");
        arraydeque.add(1, "b");
        arraydeque.add(2, "c");
        arraydeque.add(3, "d");
        arraydeque.add(4, "e");
        arraydeque.add(5, "f");
        arraydeque.add(6, "g");
        arraydeque.add(7, "h");

        assert_queue(
            &arraydeque,
            vec![
                Some("a"),
                Some("b"),
                Some("c"),
                Some("d"),
                Some("e"),
                Some("f"),
                Some("g"),
                Some("h"),
                None,
                None,
                None,
                None,
            ],
        );

        arraydeque.remove(2);

        assert_queue(
            &arraydeque,
            vec![
                None,
                Some("a"),
                Some("b"),
                Some("d"),
                Some("e"),
                Some("f"),
                Some("g"),
                Some("h"),
                None,
                None,
                None,
                None,
            ],
        );

        arraydeque.add(4, "x");

        assert_queue(
            &arraydeque,
            vec![
                None,
                Some("a"),
                Some("b"),
                Some("d"),
                Some("e"),
                Some("x"),
                Some("f"),
                Some("g"),
                Some("h"),
                None,
                None,
                None,
            ],
        );

        arraydeque.add(3, "y");

        assert_queue(
            &arraydeque,
            vec![
                Some("a"),
                Some("b"),
                Some("d"),
                Some("y"),
                Some("e"),
                Some("x"),
                Some("f"),
                Some("g"),
                Some("h"),
                None,
                None,
                None,
            ],
        );

        arraydeque.add(3, "z");

        assert_queue(
            &arraydeque,
            vec![
                Some("b"),
                Some("d"),
                Some("z"),
                Some("y"),
                Some("e"),
                Some("x"),
                Some("f"),
                Some("g"),
                Some("h"),
                None,
                None,
                Some("a"),
            ],
        );
    }
}
