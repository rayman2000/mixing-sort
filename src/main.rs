use rand::Rng;
use rayon::prelude::*;

use crate::sorter::{Sorter, run};
use crate::bubble::BubbleSorter;
use crate::selection::SelectionSorter;
//use crate::insertion::InsertionSorter;
use crate::quick::QuickSorter;
// use crate::shell::ShellSorter;
use crate::merge::MergeSorter;

mod sorter;
mod plot;
mod bubble;
mod selection;
mod insertion;
mod quick;
mod heap;
mod shell;
mod merge;

fn main() {
    if std::env::args().any(|a| a == "--viz") {
        visualize();
        return;
    }

    const LEN_START: usize = 32;
    const LEN_END: usize = 128;
    const LEN_STEP: usize = 2;
    const RUNS: usize = 500;
    const MAX_COMBO_SIZE: u32 = 2;

    type Factory = (&'static str, fn(usize) -> Box<dyn Sorter<u8>>);
    let factories: &[Factory] = &[
        ("Bubble",    |n| Box::new(BubbleSorter::new(n))),
        ("Selection", |_| Box::new(SelectionSorter::new())),
        //("Insertion", |_| Box::new(InsertionSorter::new())),
        ("Quick",     |n| Box::new(QuickSorter::new(n))),
        //("Shell",     |n| Box::new(ShellSorter::new(n))),
        ("Merge",     |n| Box::new(MergeSorter::new(n))),
    ];

    let lengths: Vec<usize> = (LEN_START..=LEN_END).step_by(LEN_STEP).collect();

    let masks: Vec<u32> = (1u32..=(1u32 << factories.len()) - 1)
        .filter(|m| m.count_ones() <= MAX_COMBO_SIZE)
        .collect();
    let combo_names: Vec<String> = masks.iter().map(|&mask| {
        factories.iter().enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, (name, _))| *name)
            .collect::<Vec<_>>()
            .join(" + ")
    }).collect();

    // Flatten (len, run) into one parallel iterator so all cores stay busy across all lengths.
    let pairs: Vec<(usize, usize)> = lengths.iter()
        .flat_map(|&len| (0..RUNS).map(move |r| (len, r)))
        .collect();

    let flat_results: Vec<(usize, Vec<Option<usize>>)> = pairs.into_par_iter().map(|(len, _)| {
        let mut data = vec![0u8; len];
        rand::rng().fill_bytes(&mut data);
        let results = masks.iter().map(|&mask| {
            let mut sorters: Vec<Box<dyn Sorter<u8>>> = factories.iter().enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, (_, f))| f(len))
                .collect();
            let mut data_copy = data.clone();
            run(&mut sorters, &mut data_copy, 5 * len * len)
        }).collect();
        (len, results)
    }).collect();

    // Aggregate per (len, combo).
    use std::collections::HashMap;
    let mut totals: HashMap<(usize, usize), (u64, usize)> = HashMap::new();
    for (len, results) in flat_results {
        for (idx, result) in results.into_iter().enumerate() {
            let entry = totals.entry((len, idx)).or_default();
            match result {
                Some(n) => entry.0 += n as u64,
                None    => entry.1 += 1,
            }
        }
    }

    let mut all_points: Vec<(usize, String, f64)> = Vec::new();
    for &len in &lengths {
        for (idx, name) in combo_names.iter().enumerate() {
            if let Some(&(total, dnf_count)) = totals.get(&(len, idx)) {
                let successes = RUNS - dnf_count;
                if successes > 0 {
                    all_points.push((len, name.clone(), total as f64 / successes as f64));
                }
            }
        }
    }

    println!("length,combo,avg_steps");
    for (len, combo, avg) in &all_points {
        println!("{},{},{:.2}", len, combo, avg);
    }

    plot::plot(&all_points);
    eprintln!("saved plot.png");
}

fn visualize() {
    use std::{io::{self, Write}, thread, time::Duration};

    const LEN: usize = 32;
    const DELAY_MS: u64 = 120;
    const BLOCKS: &[char] = &['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    let mut data: [u8; LEN] = {
        let mut d = [0u8; LEN];
        rand::rng().fill_bytes(&mut d);
        d
    };

    let mut bubble = BubbleSorter::new(LEN);
    let mut selection = SelectionSorter::new();

    let render = |data: &[u8], b_pos: &[usize], s_pos: &[usize], step: usize| {
        print!("\x1b[2J\x1b[H");
        println!("step {:>4}   \x1b[34m[B]\x1b[0m bubble   \x1b[31m[S]\x1b[0m selection   \x1b[33m[X]\x1b[0m both", step);
        println!();
        for (i, &val) in data.iter().enumerate() {
            let ch = BLOCKS[val as usize * (BLOCKS.len() - 1) / 255];
            let is_b = b_pos.contains(&i);
            let is_s = s_pos.contains(&i);
            match (is_b, is_s) {
                (true, true)  => print!("\x1b[33m{}\x1b[0m", ch),
                (true, false) => print!("\x1b[34m{}\x1b[0m", ch),
                (false, true) => print!("\x1b[31m{}\x1b[0m", ch),
                _             => print!("{}", ch),
            }
        }
        println!();
        for i in 0..LEN {
            let is_b = b_pos.contains(&i);
            let is_s = s_pos.contains(&i);
            match (is_b, is_s) {
                (true, true)  => print!("\x1b[33mX\x1b[0m"),
                (true, false) => print!("\x1b[34mB\x1b[0m"),
                (false, true) => print!("\x1b[31mS\x1b[0m"),
                _             => print!(" "),
            }
        }
        println!();
        io::stdout().flush().unwrap();
    };

    render(&data, &[], &[], 0);
    thread::sleep(Duration::from_millis(800));

    for step in 1.. {
        let before = data;
        bubble.sort_step(&mut data);
        let b_pos: Vec<usize> = (0..LEN).filter(|&i| before[i] != data[i]).collect();
        let mid = data;

        selection.sort_step(&mut data);
        let s_pos: Vec<usize> = (0..LEN).filter(|&i| mid[i] != data[i]).collect();

        render(&data, &b_pos, &s_pos, step);

        if data.is_sorted() {
            println!("\nSorted in {} steps!", step);
            break;
        }
        thread::sleep(Duration::from_millis(DELAY_MS));
    }
}


