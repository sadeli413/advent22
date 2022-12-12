use clap::Parser;
use itertools::Itertools;

const SIZE: usize = 14;

fn main() {
    let part1 = std::fs::read_to_string(Args::parse().filename)
        .expect("Could not read file")
        .as_bytes()
        .windows(SIZE)
        .zip(1..)
        .take_while(|(bytes, _)| !bytes.iter().all_unique())
        .last()
        .map(|(_, i)| i)
        .unwrap()
        + SIZE;

    println!("{}", part1);
}

#[derive(Parser)]
struct Args {
    filename: String,
}
