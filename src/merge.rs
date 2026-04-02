use crate::sorter::Sorter;

// Bottom-up in-place merge sort.
// Each step is either one comparison (advancing the left pointer) or one full
// insertion (rotating data[m] leftward into position i via sequential swaps).
// This makes the step count O(n log n): O(n) steps per pass × O(log n) passes.
pub struct MergeSorter {
    n: usize,
    width: usize,
    // Current merge window: [l..m) is the sorted left half, [m..r) is the sorted right half.
    l: usize,
    m: usize,
    r: usize,
    i: usize,   // left pointer: next position to compare/fill
    done: bool,
}

impl MergeSorter {
    pub fn new(n: usize) -> Self {
        if n <= 1 {
            return MergeSorter { n, width: 1, l: 0, m: 0, r: 0, i: 0, done: true };
        }
        MergeSorter { n, width: 1, l: 0, m: 1, r: 2.min(n), i: 0, done: false }
    }

    fn advance_block(&mut self) {
        loop {
            let next_l = self.l + 2 * self.width;
            if next_l < self.n {
                self.l = next_l;
                self.m = (self.l + self.width).min(self.n);
                self.r = (self.l + 2 * self.width).min(self.n);
                self.i = self.l;
            } else {
                self.width *= 2;
                if self.width >= self.n {
                    self.done = true;
                    return;
                }
                self.l = 0;
                self.m = self.width.min(self.n);
                self.r = (2 * self.width).min(self.n);
                self.i = 0;
            }
            // Only stop if there is actually a right half to merge
            if self.m < self.r {
                return;
            }
        }
    }
}

impl<T: Ord> Sorter<T> for MergeSorter {
    fn sort_step(&mut self, data: &mut [T]) {
        if self.done {
            *self = MergeSorter::new(self.n);
            return;
        }

        if self.i >= self.m || self.m >= self.r {
            self.advance_block();
            return;
        }

        if data[self.i] <= data[self.m] {
            self.i += 1;
            if self.i >= self.m {
                self.advance_block();
            }
        } else {
            // Rotate data[m] into position i by shifting left — all in one step.
            // This counts as a single "insert" operation.
            let mut k = self.m;
            while k > self.i {
                data.swap(k - 1, k);
                k -= 1;
            }
            self.i += 1;
            self.m += 1;
            if self.i >= self.m || self.m >= self.r {
                self.advance_block();
            }
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
        let mut sorter = MergeSorter::new(data.len());
        for _ in 0..data.len() * data.len() {
            sorter.sort_step(&mut data);
        }
        assert!(data.is_sorted());
    }

    #[test]
    fn sorts_non_power_of_two() {
        let mut data = [5u8, 3, 8, 1, 9, 2, 7, 4, 6];
        let n = data.len();
        let mut sorter = MergeSorter::new(n);
        for _ in 0..n * n {
            sorter.sort_step(&mut data);
        }
        assert!(data.is_sorted());
    }

    #[test]
    fn merge_step_trace() {
        // width=1 pass: merge [3,1] → [1,3], merge [4,2] → [2,4]
        // width=2 pass: merge [1,3,2,4] → [1,2,3,4]
        let mut data = [3u8, 1, 4, 2];
        let mut sorter = MergeSorter::new(data.len());
        for _ in 0..data.len() * data.len() {
            sorter.sort_step(&mut data);
            if data.is_sorted() { break; }
        }
        assert_eq!(data, [1, 2, 3, 4]);
    }
}
