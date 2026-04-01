use crate::sorter::Sorter;

pub struct SelectionSorter {
    sorted_end: usize, // position currently being filled
    scan: usize,       // current scan position
    min_index: usize,  // best candidate found so far
}

impl SelectionSorter {
    pub fn new() -> Self {
        SelectionSorter { sorted_end: 0, scan: 1, min_index: 0 }
    }
}

impl<T: Ord> Sorter<T> for SelectionSorter {
    fn sort_step(&mut self, data: &mut [T]) {
        let n = data.len();
        if self.sorted_end >= n - 1 {
            // Finished sorting, restart
            self.sorted_end = 0;
            self.scan = 1;
            self.min_index = 0;
            return;
        }

        if self.scan >= n {
            // Scan complete: place the minimum found
            data.swap(self.sorted_end, self.min_index);
            self.sorted_end += 1;
            self.scan = self.sorted_end + 1;
            self.min_index = self.sorted_end;
        } else {
            // One comparison
            if data[self.scan] < data[self.min_index] {
                self.min_index = self.scan;
            }
            self.scan += 1;
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
        let mut sorter = SelectionSorter::new();
        for _ in 0..data.len() * data.len() {
            sorter.sort_step(&mut data);
        }
        assert!(data.is_sorted());
    }

    #[test]
    fn single_step_scans() {
        let mut data = [3u8, 1, 2];
        let mut sorter = SelectionSorter::new();
        sorter.sort_step(&mut data); // scan: data[1]=1 < data[0]=3, min_index→1
        assert_eq!(data, [3, 1, 2]); // no swap yet
        sorter.sort_step(&mut data); // scan: data[2]=2 vs min(1), no change
        assert_eq!(data, [3, 1, 2]); // still no swap
        sorter.sort_step(&mut data); // scan complete: swap(0,1) → minimum placed
        assert_eq!(data, [1, 3, 2]);
    }
}
