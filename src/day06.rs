use clap::Parser;
use itertools::Itertools;

const PART1: usize = 4;
const PART2: usize = 14;

fn main() {
    let input = std::fs::read_to_string(Args::parse().filename).expect("Could not read file");

    println!("Part 1: {}", puzzle(&input, PART1));
    println!("Part 2: {}", puzzle(&input, PART2));
}

fn puzzle(input: &str, size: usize) -> usize {
    input
        .as_bytes()
        .windows(size)
        .zip(1..)
        .take_while(|(bytes, _)| !bytes.iter().all_unique())
        .last()
        .map(|(_, i)| i)
        .unwrap()
        + size
}

#[derive(Parser)]
struct Args {
    filename: String,
}
