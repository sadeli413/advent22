use std::collections::HashMap;

use clap::Parser;

fn main() {
    // Letters to score
    let mut conversion = HashMap::new();
    conversion.insert("A", 0); // Rock
    conversion.insert("B", 1); // Paper
    conversion.insert("C", 2); // Scissors
    conversion.insert("X", 0); // Lose
    conversion.insert("Y", 3); // Draw
    conversion.insert("Z", 6); // Win

    let part2: usize = std::fs::read_to_string(Args::parse().filename)
        .expect("Could not read file")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|line| line.split(' ').collect())
        .map(|line: Vec<&str>| {
            let opponent: &usize = conversion.get(line[0]).unwrap();
            let myself = match line[1] {
                "X" => opponent.checked_sub(1).unwrap_or(2) + 1,
                "Y" => opponent + 1,
                "Z" => (opponent + 1) % 3 + 1,
                _ => panic!("ummmmmmm what"),
            };
            myself + conversion.get(line[1]).unwrap()
        })
        .sum();

    println!("{}", part2);
}

#[derive(Parser)]
struct Args {
    filename: String,
}
