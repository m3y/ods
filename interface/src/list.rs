pub trait List<T>
where
    T: std::fmt::Debug,
{
    fn size(&self) -> usize;
    fn get(&self, i: usize) -> Option<T>;
    fn set(&self, i: usize, x: T);
    fn add(&self, i: usize, x: T);
    fn remove(&self, i: usize);
}
