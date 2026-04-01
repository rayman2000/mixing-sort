pub trait Sorter<T> {
    fn sort_step(&mut self, data: &mut [T]);
}