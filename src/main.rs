use rand::Rng;

use crate::sorter::Sorter;
use crate::bubble::BubbleSorter;

mod sorter;
mod bubble;

fn main() {

    const LEN: usize = 32;
    let mut data = [0u8; LEN];
    rand::rng().fill_bytes(&mut data);
    let mut sorters: Vec<Box<dyn Sorter<u8>>> = vec![
        Box::new(BubbleSorter::new(data.len())),
        // You can add more sorters here
    ];

    run(&mut sorters, &mut data);
    println!("{:?}", data);
}

fn run<T: Ord>(sorters: &mut [Box<dyn Sorter<T>>], data: &mut [T]) {
    
    let iter_length = data.len() * data.len(); // n^2 steps
    
    for _ in 0..iter_length {
        for sorter in sorters.iter_mut() {
            sorter.sort_step(data);
        }
    }
}
