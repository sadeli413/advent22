use std::collections::HashMap;

use clap::Parser;

fn main() {
    // Letters to score
    let mut conversion = HashMap::new();
    conversion.insert("A", 1); // Rock
    conversion.insert("B", 2); // Paper
    conversion.insert("C", 3); // Scissors
    conversion.insert("X", 1); // Rock
    conversion.insert("Y", 2); // Paper
    conversion.insert("Z", 3); // Scissors

    // Deltas to score
    let mut scores = HashMap::new();
    scores.insert(-1, 0);
    scores.insert(2, 0);
    scores.insert(0, 3);
    scores.insert(1, 6);
    scores.insert(-2, 6);

    let part1: i32 = std::fs::read_to_string(Args::parse().filename)
        .expect("Could not read file")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.split(' ')
                .map(|letter| conversion.get(letter).unwrap())
                .collect()
        })
        .map(|line: Vec<&i32>| {
            let opponent = line[0];
            let myself = line[1];
            let score = scores.get(&(myself - opponent)).unwrap();
            score + myself
        })
        .sum();

    println!("{}", part1);
}

#[derive(Parser)]
struct Args {
    filename: String,
}
