use clap::Parser;

fn main() {
    let mut calories: Vec<usize> = std::fs::read_to_string(Args::parse().filename)
        .expect("Could not read input file.")
        .split("\n\n")
        .map(|elves| {
            elves
                .split('\n')
                .map(|calories| calories.parse::<usize>().unwrap_or(0))
                .sum()
        })
        .collect();

    calories.sort();
    calories.reverse();

    println!("Part 1: {}", calories.first().unwrap_or(&0));
    println!("Part 2: {}", calories.iter().take(3).sum::<usize>());
}

#[derive(Parser)]
struct Args {
    filename: String,
}
