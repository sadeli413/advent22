use std::collections::HashMap;

use clap::Parser;

fn main() {
    let input = std::fs::read_to_string(Args::parse().filename).expect("Could not read file");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
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

    input
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
        .sum()
}

fn part2(input: &str) -> usize {
    // Letters to score
    let mut conversion = HashMap::new();
    conversion.insert("A", 0); // Rock
    conversion.insert("B", 1); // Paper
    conversion.insert("C", 2); // Scissors
    conversion.insert("X", 0); // Lose
    conversion.insert("Y", 3); // Draw
    conversion.insert("Z", 6); // Win

    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|line| line.split(' ').collect())
        .map(|line: Vec<&str>| {
            let opponent: &usize = conversion.get(line[0]).unwrap();
            let myself = match line[1] {
                "X" => opponent.checked_sub(1).unwrap_or(2) + 1,
                "Y" => opponent + 1,
                "Z" => (opponent + 1) % 3 + 1,
                _ => panic!("at the disco"),
            };
            myself + conversion.get(line[1]).unwrap()
        })
        .sum()
}

#[derive(Parser)]
struct Args {
    filename: String,
}
