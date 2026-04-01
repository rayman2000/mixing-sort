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