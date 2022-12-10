use clap::Parser;
use std::collections::{hash_map::RandomState, HashSet};

fn main() {
    // Create the alphabet
    let offset = vec![0];
    let lower: Vec<u8> = (b'a'..=b'z').collect();
    let upper: Vec<u8> = (b'A'..=b'Z').collect();
    let priorities = vec![offset, lower, upper].concat();

    let part1: usize = std::fs::read_to_string(Args::parse().filename)
        .expect("Could not read file.")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.as_bytes()
                .chunks(line.len() / 2)
                .map(|chunk| HashSet::from_iter(chunk.to_owned()))
                .collect()
        })
        .map(|line: Vec<HashSet<u8, RandomState>>| {
            (&line[0] & &line[1])
                .into_iter()
                .map(|byte| priorities.iter().position(|&b| b == byte).unwrap())
                .min()
                .unwrap()
        })
        .sum();

    println!("{}", part1);
}

#[derive(Parser)]
struct Args {
    filename: String,
}
