// List Interfaceで賄えるって書いてあるけど本当か？
pub trait Queue<T: Clone> {
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self) -> Option<T>;
}
