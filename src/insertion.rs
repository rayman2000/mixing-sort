use crate::sorter::Sorter;

pub struct InsertionSorter {
    sorted_end: usize,
    live_index: usize,
}

impl InsertionSorter {
    pub fn new() -> Self {
        InsertionSorter { sorted_end: 1, live_index: 1 }
    }
}

impl<T: Ord> Sorter<T> for InsertionSorter {
    fn sort_step(&mut self, data: &mut [T]) {
        if self.sorted_end >= data.len() {
            // Finished sorting, restart
            self.sorted_end = 1;
            self.live_index = 1;
        } else if self.live_index == 0 || data[self.live_index - 1] <= data[self.live_index] {
            // Element is in the right place, move to the next insertion
            self.sorted_end += 1;
            self.live_index = self.sorted_end;
        } else {
            // Swap the live element one step to the left
            data.swap(self.live_index - 1, self.live_index);
            self.live_index -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn sorts_random_data() {
        let mut data = [0u8; 32];
        rand::rng().fill_bytes(&mut data);
        let mut sorter = InsertionSorter::new();
        for _ in 0..data.len() * data.len() {
            sorter.sort_step(&mut data);
        }
        assert!(data.is_sorted());
    }

    #[test]
    fn single_step_swaps_leftward() {
        let mut data = [1u8, 3, 2];
        let mut sorter = InsertionSorter::new();
        sorter.sort_step(&mut data); // live=1: 3 >= 1, advance sorted_end → live=2
        assert_eq!(data, [1, 3, 2]);
        sorter.sort_step(&mut data); // live=2: 3 > 2, swap → [1, 2, 3], live=1
        assert_eq!(data, [1, 2, 3]);
    }
}
