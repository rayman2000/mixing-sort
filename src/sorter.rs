pub trait Sorter<T> {
    fn sort_step(&mut self, data: &mut [T]);
}

pub fn run<T: Ord>(sorters: &mut [Box<dyn Sorter<T>>], data: &mut [T], max_steps: usize) -> Option<usize> {
    for steps in 1..=max_steps {
        for sorter in sorters.iter_mut() {
            sorter.sort_step(data);
        }
        if data.is_sorted() {
            return Some(steps);
        }
    }
    None
}