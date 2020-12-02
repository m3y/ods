struct Array<T> {
    a: T,
    length: usize,
}

impl<T> Array<T> {
    pub fn length(&self) -> usize {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a: Array<u8> = Array { a: 1, length: 1 };
        assert_eq!(a.a, 1);
        assert_eq!(a.length(), 1);
    }
}
