use crate::sorter::Sorter;

// Knuth's gap sequence: 1, 4, 13, 40, ... (h = 3h + 1)
fn initial_gap(len: usize) -> usize {
    let mut gap = 1;
    while gap * 3 + 1 < len {
        gap = gap * 3 + 1;
    }
    gap
}

pub struct ShellSorter {
    gap: usize,       // 0 means finished, will restart on next step
    sorted_end: usize,
    live_index: usize,
}

impl ShellSorter {
    pub fn new(len: usize) -> Self {
        let gap = if len > 1 { initial_gap(len) } else { 0 };
        ShellSorter { gap, sorted_end: gap, live_index: gap }
    }
}

impl<T: Ord> Sorter<T> for ShellSorter {
    fn sort_step(&mut self, data: &mut [T]) {
        let n = data.len();
        if n <= 1 {
            return;
        }

        if self.gap == 0 {
            // Finished sorting, restart
            let gap = initial_gap(n);
            self.gap = gap;
            self.sorted_end = gap;
            self.live_index = gap;
            return;
        }

        if self.sorted_end >= n {
            // Done with this gap, shrink to the next Knuth gap (h = (h-1)/3)
            let new_gap = (self.gap - 1) / 3;
            if new_gap == 0 {
                self.gap = 0; // gap=1 pass complete, sort done
            } else {
                self.gap = new_gap;
                self.sorted_end = new_gap;
                self.live_index = new_gap;
            }
        } else if self.live_index < self.gap || data[self.live_index - self.gap] <= data[self.live_index] {
            // Live element is in place for this gap; advance to the next
            self.sorted_end += 1;
            self.live_index = self.sorted_end;
        } else {
            // Swap the live element one gap to the left
            data.swap(self.live_index - self.gap, self.live_index);
            self.live_index -= self.gap;
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
        let mut sorter = ShellSorter::new(data.len());
        for _ in 0..data.len() * data.len() {
            sorter.sort_step(&mut data);
        }
        assert!(data.is_sorted());
    }

    #[test]
    fn single_step_swaps_across_gap() {
        // n=7: initial_gap=4; first insertion is live_index=4
        let mut data = [7u8, 1, 2, 3, 4, 5, 6];
        let mut sorter = ShellSorter::new(data.len());
        sorter.sort_step(&mut data); // gap=4: data[0]=7 > data[4]=4, swap across gap
        assert_eq!(data, [4, 1, 2, 3, 7, 5, 6]);
    }
}
