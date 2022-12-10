use clap::Parser;
use itertools::Itertools;

fn main() {
    let file = std::fs::read_to_string(Args::parse().filename).expect("Could not read file.");
    let input: Vec<&str> = file.split("\n\n").collect();
    let stacks = input[0];
    let instructions = input[1];
    let mut stacks = Stacks::from(stacks);

    instructions.lines().for_each(|line| {
        stacks.do_instruction(Instruction::from(line));
    });

    stacks
        .stacks
        .iter()
        .for_each(|v| print!("{}", v.last().unwrap()));
    println!();
}

struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn from(input: &str) -> Stacks {
        // Count number of stacks
        let num_stacks = (input.lines().next().unwrap().len() + 1) / 4;

        // Arrange the stacks
        let mut stacks = vec![vec![]; num_stacks];

        input
            .chars()
            .chunks(4)
            .into_iter()
            .zip(0..)
            .for_each(|(mut chunk, i)| {
                let c = chunk.nth(1).unwrap();
                c.is_ascii_uppercase()
                    .then(|| stacks[i % num_stacks].insert(0, c));
            });

        Stacks { stacks }
    }

    fn do_instruction(&mut self, instruction: Instruction) {
        let start = self.stacks[instruction.from].len() - instruction.count;
        let drained: Vec<char> = self.stacks[instruction.from].drain(start..).rev().collect();
        self.stacks[instruction.to].extend(drained);
    }
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from(line: &str) -> Instruction {
        let digits: Vec<usize> = line
            .split_whitespace()
            .map(|word| word.parse::<usize>())
            .filter_map(|result| result.ok())
            .collect();

        Instruction {
            count: digits[0],
            from: digits[1] - 1,
            to: digits[2] - 1,
        }
    }
}

#[derive(Parser)]
struct Args {
    filename: String,
}
