use crate::sorter::Sorter;

pub struct HeapSorter {
    building: bool,
    build_index: usize, // next node to sift during build (counts down to 0)
    heap_end: usize,    // last index of the heap region (sorted region is beyond)
    sift_pos: usize,    // current position mid-sift
    sifting: bool,
}

impl HeapSorter {
    pub fn new(len: usize) -> Self {
        HeapSorter {
            building: true,
            build_index: if len > 1 { len / 2 - 1 } else { 0 },
            heap_end: len.saturating_sub(1),
            sift_pos: 0,
            sifting: false,
        }
    }
}

impl<T: Ord> Sorter<T> for HeapSorter {
    fn sort_step(&mut self, data: &mut [T]) {
        let n = data.len();
        if n <= 1 {
            return;
        }

        if !self.sifting {
            if self.building {
                self.sift_pos = self.build_index;
            } else {
                if self.heap_end == 0 {
                    // Finished sorting, restart
                    self.building = true;
                    self.build_index = n / 2 - 1;
                    self.heap_end = n - 1;
                    return;
                }
                // Extract max: swap root to sorted region, shrink heap
                data.swap(0, self.heap_end);
                self.heap_end -= 1;
                self.sift_pos = 0;
            }
            self.sifting = true;
        }

        // One sift-down step: swap with the largest child if it's bigger
        let pos = self.sift_pos;
        let left = 2 * pos + 1;
        let right = 2 * pos + 2;
        let heap_size = self.heap_end + 1;

        let mut largest = pos;
        if left < heap_size && data[left] > data[largest] {
            largest = left;
        }
        if right < heap_size && data[right] > data[largest] {
            largest = right;
        }

        if largest != pos {
            data.swap(pos, largest);
            self.sift_pos = largest;
        } else {
            // Element is in the right place, sift done
            self.sifting = false;
            if self.building {
                if self.build_index == 0 {
                    self.building = false;
                } else {
                    self.build_index -= 1;
                }
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
        let mut sorter = HeapSorter::new(data.len());
        for _ in 0..data.len() * data.len() {
            sorter.sort_step(&mut data);
            if data.is_sorted() { return; }
        }
        panic!("did not sort within n² steps");
    }

    #[test]
    fn single_step_sifts() {
        // [2, 1, 3]: build starts at index 0 (the only non-leaf in a 3-element heap)
        let mut data = [2u8, 1, 3];
        let mut sorter = HeapSorter::new(data.len());
        sorter.sort_step(&mut data); // build: 3 > 2, swap(0, 2) → 3 at root
        assert_eq!(data, [3, 1, 2]);
        sorter.sort_step(&mut data); // pos 2 is a leaf, no swap; build phase complete
        assert_eq!(data, [3, 1, 2]);
        sorter.sort_step(&mut data); // extract: swap root(3) to end → 3 in sorted region
        assert_eq!(data, [2, 1, 3]);
    }
}
