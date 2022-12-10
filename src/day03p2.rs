use std::collections::{hash_map::RandomState, HashSet};

use clap::Parser;
use itertools::Itertools;

fn main() {
    // Create the alphabet
    let offset = vec![0];
    let lower: Vec<u8> = (b'a'..=b'z').collect();
    let upper: Vec<u8> = (b'A'..=b'Z').collect();
    let priorities = vec![offset, lower, upper].concat();

    let part2: usize = std::fs::read_to_string(Args::parse().filename)
        .expect("Could not read file.")
        .lines()
        .filter(|s| !s.is_empty())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .map(|line| HashSet::from_iter(line.as_bytes().to_owned()))
                .collect()
        })
        .map(|lines: Vec<HashSet<u8, RandomState>>| {
            (&(&lines[0] & &lines[1]) & &lines[2])
                .into_iter()
                .map(|byte| priorities.iter().position(|&b| b == byte).unwrap())
                .min()
                .unwrap()
        })
        .sum();

    println!("{:?}", part2);
}

#[derive(Parser)]
struct Args {
    filename: String,
}
