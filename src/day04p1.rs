use clap::Parser;
use std::collections::{hash_map::RandomState, HashSet};
use std::ops::Range;

fn main() {
    let part1: usize = std::fs::read_to_string(Args::parse().filename)
        .expect("Could not read file.")
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| {
                    let range: Vec<usize> = range
                        .split('-')
                        .map(|i| i.parse::<usize>().unwrap())
                        .collect();

                    HashSet::from_iter(Range {
                        start: range[0],
                        end: range[1] + 1,
                    })
                })
                .collect()
        })
        .map(|sets: Vec<HashSet<usize, RandomState>>| {
            let intersection = &sets[0] & &sets[1];
            usize::from(intersection.len() == sets[0].len() || intersection.len() == sets[1].len())
        })
        .sum();

    println!("{}", part1);
}

#[derive(Parser)]
struct Args {
    filename: String,
}
