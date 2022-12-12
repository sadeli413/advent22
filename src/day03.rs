use clap::Parser;
use itertools::Itertools;
use std::collections::{hash_map::RandomState, HashSet};

fn main() {
    // Create the alphabet
    let offset = vec![0];
    let lower: Vec<u8> = (b'a'..=b'z').collect();
    let upper: Vec<u8> = (b'A'..=b'Z').collect();
    let priorities = vec![offset, lower, upper].concat();

    let input = std::fs::read_to_string(Args::parse().filename).expect("Could not read file.");

    println!("Part 1: {}", part1(&input, &priorities));
    println!("Part 2: {}", part2(&input, &priorities));
}

fn part1(input: &str, priorities: &[u8]) -> usize {
    input
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
        .sum()
}

fn part2(input: &str, priorities: &[u8]) -> usize {
    input
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
        .sum()
}

#[derive(Parser)]
struct Args {
    filename: String,
}
