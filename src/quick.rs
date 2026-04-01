use crate::sorter::Sorter;

pub struct QuickSorter {
    stack: Vec<(usize, usize)>,
    lo: usize,
    hi: usize,
    i: usize,
    j: usize,
    active: bool,
}

impl QuickSorter {
    pub fn new(len: usize) -> Self {
        let mut stack = Vec::new();
        if len > 1 {
            stack.push((0, len - 1));
        }
        QuickSorter { stack, lo: 0, hi: 0, i: 0, j: 0, active: false }
    }
}

impl<T: Ord> Sorter<T> for QuickSorter {
    fn sort_step(&mut self, data: &mut [T]) {
        if !self.active {
            loop {
                match self.stack.pop() {
                    None => {
                        // Finished sorting, restart
                        if data.len() > 1 {
                            self.stack.push((0, data.len() - 1));
                        }
                        return;
                    }
                    Some((lo, hi)) if lo >= hi => continue, // skip trivial subarrays
                    Some((lo, hi)) => {
                        self.lo = lo;
                        self.hi = hi;
                        self.i = lo;
                        self.j = lo;
                        self.active = true;
                        break;
                    }
                }
            }
        }

        // Lomuto partition: pivot is data[self.hi]
        if self.j < self.hi {
            if data[self.j] <= data[self.hi] {
                data.swap(self.i, self.j);
                self.i += 1;
            }
            self.j += 1;
        } else {
            // j == hi: put pivot in its final position
            data.swap(self.i, self.hi);
            let p = self.i;
            if p > self.lo {
                self.stack.push((self.lo, p - 1));
            }
            if p < self.hi {
                self.stack.push((p + 1, self.hi));
            }
            self.active = false;
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
        let mut sorter = QuickSorter::new(data.len());
        for _ in 0..data.len() * data.len() {
            sorter.sort_step(&mut data);
        }
        assert!(data.is_sorted());
    }

    #[test]
    fn single_step_partitions() {
        // Trace [3, 1, 2], pivot = data[2] = 2
        let mut data = [3u8, 1, 2];
        let mut sorter = QuickSorter::new(data.len());
        sorter.sort_step(&mut data); // j=0: 3 > pivot(2), no swap, j→1
        assert_eq!(data, [3, 1, 2]);
        sorter.sort_step(&mut data); // j=1: 1 <= pivot(2), swap(0,1) → [1,3,2], j→2
        assert_eq!(data, [1, 3, 2]);
        sorter.sort_step(&mut data); // j=2=hi: finalize, swap pivot → [1,2,3]
        assert_eq!(data, [1, 2, 3]);
    }
}
