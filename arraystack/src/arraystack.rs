struct Array<T> {
    a: T,
    length: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a: Array<u8> = Array { a: 1, length: 0 };
        assert_eq!(a.a, 1);
    }
}
