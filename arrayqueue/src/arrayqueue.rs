use interface::queue::Queue;

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

impl<T: Clone> Queue<T> for Array<T> {
    fn add(&mut self, x: T) -> bool {
        if self.n + 1 > self.a.len() {
            self.resize()
        }

        self.a[(self.j + self.n) % self.a.len()] = Some(x);
        self.n += 1;
        true
    }

    fn remove(&mut self) -> Option<T> {
        let x: Option<T> = self.a[self.j].take();
        self.j += 1;
        self.n -= 1;

        if self.a.len() > 3 * self.n {
            self.resize();
        }

        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Array<&str> {
        /// for unittest
        fn get(&self, i: usize) -> Option<&str> {
            self.a.get(i)?.as_ref().cloned()
        }
    }

    #[test]
    fn it_works() {
        let _arrayqueue: Array<&str> = Array::new();
        assert_eq!(2 + 2, 4);
    }

    fn assert_queue(queue: &Array<&str>, expected_list: Vec<Option<&str>>) {
        for (i, e) in expected_list.iter().enumerate() {
            assert_eq!(queue.get(i), *e);
        }
    }

    #[test]
    fn test_arrayqueue() {
        let mut arrayqueue: Array<&str> = Array::with_length(6); // [][][][][][]
        assert!(arrayqueue.add("A")); // ["A"][][][][][]
        assert!(arrayqueue.add("B")); // ["A"]["B"][][][][]
        assert!(arrayqueue.add("a")); // ["A"]["B"]["a"][][][]
        assert!(arrayqueue.add("b")); // ["A"]["B"]["a"]["b"][][]
        assert!(arrayqueue.add("c")); // ["A"]["B"]["a"]["b"]["c"][]
        assert_eq!(arrayqueue.remove(), Some("A")); // []["B"]["a"]["b"]["c"][]
        assert_eq!(arrayqueue.remove(), Some("B")); // [][]["a"]["b"]["c"][]

        assert_queue(
            &arrayqueue,
            vec![None, None, Some("a"), Some("b"), Some("c"), None],
        );

        assert!(arrayqueue.add("d"));

        assert_queue(
            &arrayqueue,
            vec![None, None, Some("a"), Some("b"), Some("c"), Some("d")],
        );

        assert!(arrayqueue.add("e"));

        assert_queue(
            &arrayqueue,
            vec![Some("e"), None, Some("a"), Some("b"), Some("c"), Some("d")],
        );

        assert_eq!(arrayqueue.remove(), Some("a"));

        assert_queue(
            &arrayqueue,
            vec![Some("e"), None, None, Some("b"), Some("c"), Some("d")],
        );

        assert!(arrayqueue.add("f"));

        assert_queue(
            &arrayqueue,
            vec![Some("e"), Some("f"), None, Some("b"), Some("c"), Some("d")],
        );

        assert!(arrayqueue.add("g"));

        assert_queue(
            &arrayqueue,
            vec![
                Some("e"),
                Some("f"),
                Some("g"),
                Some("b"),
                Some("c"),
                Some("d"),
            ],
        );

        assert!(arrayqueue.add("h"));

        assert_queue(
            &arrayqueue,
            vec![
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
                None,
            ],
        );

        assert_eq!(arrayqueue.remove(), Some("b"));

        assert_queue(
            &arrayqueue,
            vec![
                None,
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
                None,
            ],
        );

        assert_eq!(arrayqueue.remove(), Some("c"));

        assert_queue(
            &arrayqueue,
            vec![
                None,
                None,
                Some("d"),
                Some("e"),
                Some("f"),
                Some("g"),
                Some("h"),
                None,
                None,
                None,
                None,
                None,
            ],
        );

        assert_eq!(arrayqueue.remove(), Some("d"));

        assert_queue(
            &arrayqueue,
            vec![
                None,
                None,
                None,
                Some("e"),
                Some("f"),
                Some("g"),
                Some("h"),
                None,
                None,
                None,
                None,
                None,
            ],
        );

        assert_eq!(arrayqueue.remove(), Some("e"));

        assert_queue(
            &arrayqueue,
            vec![Some("f"), Some("g"), Some("h"), None, None, None, None],
        );
    }
}
