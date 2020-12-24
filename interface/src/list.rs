pub trait List<T: Clone> {
    fn size(&self) -> usize;
    fn get(&self, i: usize) -> Option<T>;
    fn set(&mut self, i: usize, x: T) -> Option<T>;
    fn add(&mut self, i: usize, x: T);
    fn remove(&mut self, i: usize);
}
