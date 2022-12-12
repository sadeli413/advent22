use clap::Parser;
use std::collections::{hash_map::RandomState, HashSet};
use std::ops::Range;

type PartFn = fn(HashSet<usize>, Vec<HashSet<usize>>) -> bool;

fn main() {
    let input = std::fs::read_to_string(Args::parse().filename).expect("Could not read file.");

    println!("Part 1: {}", puzzle(&input, part1));
    println!("Part 2: {}", puzzle(&input, part2));
}

fn puzzle(input: &str, part: PartFn) -> usize {
    input
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
            usize::from(part(intersection, sets))
        })
        .sum()
}

fn part1(intersection: HashSet<usize>, sets: Vec<HashSet<usize, RandomState>>) -> bool {
    intersection.len() == sets[0].len() || intersection.len() == sets[1].len()
}

fn part2<T>(intersection: HashSet<usize>, _: T) -> bool {
    !intersection.is_empty()
}

#[derive(Parser)]
struct Args {
    filename: String,
}
