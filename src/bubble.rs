use crate::sorter::Sorter;

pub struct BubbleSorter{
    current_index: usize,
    current_end: usize

    
}

impl BubbleSorter {
    pub fn new(l: usize) -> Self {
        BubbleSorter {
            current_index: 0,
            current_end: l
        }
    }
}

impl<T: Ord> Sorter<T> for BubbleSorter {
    fn sort_step(&mut self, data: &mut [T]) {
        
        if self.current_end == 1 {
            // We have finished sorting, so we restart
            // self.current_index should already be 0
            self.current_end = data.len();
        } else if self.current_index == self.current_end - 1 {
            // We have reached the end of the current pass, so we start a new one
            self.current_index = 0;
            self.current_end -= 1;
        } else {
            // We are in the middle of a pass, so we compare and swap if necessary
            if data[self.current_index] > data[self.current_index + 1] {
                data.swap(self.current_index, self.current_index + 1);
            }
            self.current_index += 1;
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
        let mut sorter = BubbleSorter::new(data.len());
        for _ in 0..data.len() * data.len() {
            sorter.sort_step(&mut data);
        }
        assert!(data.is_sorted());
    }

    #[test]
    fn single_step_swaps_adjacent() {
        let mut data = [3u8, 1, 2];
        let mut sorter = BubbleSorter::new(data.len());
        sorter.sort_step(&mut data);
        assert_eq!(data, [1, 3, 2]);
    }
}