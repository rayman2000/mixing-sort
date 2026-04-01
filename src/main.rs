use rand::Rng;

use crate::sorter::Sorter;
use crate::bubble::BubbleSorter;
use crate::selection::SelectionSorter;
use crate::insertion::InsertionSorter;
use crate::quick::QuickSorter;
use crate::shell::ShellSorter;

mod sorter;
mod bubble;
mod selection;
mod insertion;
mod quick;
mod heap;
mod shell;

fn main() {
    const LEN: usize = 64;
    const RUNS: usize = 1000;

    type Factory = (&'static str, fn(usize) -> Box<dyn Sorter<u8>>);
    let factories: &[Factory] = &[
        ("Bubble",    |n| Box::new(BubbleSorter::new(n))),
        ("Selection", |_| Box::new(SelectionSorter::new())),
        ("Insertion", |_| Box::new(InsertionSorter::new())),
        ("Quick",     |n| Box::new(QuickSorter::new(n))),
        ("Shell",     |n| Box::new(ShellSorter::new(n))),
    ];

    let combo_count = (1u32 << factories.len()) - 1;
    // (name, total_steps, dnf_count)
    let mut results: Vec<(String, u64, usize)> = (1u32..=combo_count).map(|mask| {
        let names: Vec<&str> = factories.iter().enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, (name, _))| *name)
            .collect();
        (names.join(" + "), 0u64, 0usize)
    }).collect();

    for run_i in 0..RUNS {
        if run_i % 100 == 0 {
            eprintln!("  {}/{}", run_i, RUNS);
        }
        let data: [u8; LEN] = {
            let mut d = [0u8; LEN];
            rand::rng().fill_bytes(&mut d);
            d
        };

        for (idx, mask) in (1u32..=combo_count).enumerate() {
            let mut sorters: Vec<Box<dyn Sorter<u8>>> = factories.iter().enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, (_, f))| f(LEN))
                .collect();
            let mut data_copy = data;
            match run(&mut sorters, &mut data_copy) {
                Some(n) => results[idx].1 += n as u64,
                None    => results[idx].2 += 1,
            }
        }
    }

    results.sort_by(|a, b| {
        // DNF-only entries go last; otherwise sort by average steps
        match (a.2 == RUNS, b.2 == RUNS) {
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            _ => a.1.cmp(&b.1),
        }
    });

    println!("--- Average steps over {} runs (LEN={}) ---", RUNS, LEN);
    for (rank, (name, total, dnfs)) in results.iter().enumerate() {
        let successes = RUNS - dnfs;
        if successes == 0 {
            println!("{:>2}. {:<40}   DNF ({} runs)", rank + 1, name, dnfs);
        } else {
            let avg = *total as f64 / successes as f64;
            if *dnfs > 0 {
                println!("{:>2}. {:<40} {:>8.1} avg  (DNF: {})", rank + 1, name, avg, dnfs);
            } else {
                println!("{:>2}. {:<40} {:>8.1} avg", rank + 1, name, avg);
            }
        }
    }
}

fn run<T: Ord>(sorters: &mut [Box<dyn Sorter<T>>], data: &mut [T]) -> Option<usize> {
    const MAX_STEPS: usize = 10000;
    for steps in 1..=MAX_STEPS {
        for sorter in sorters.iter_mut() {
            sorter.sort_step(data);
        }
        if data.is_sorted() {
            return Some(steps);
        }
    }
    None
}
