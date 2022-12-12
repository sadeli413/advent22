use clap::Parser;
use itertools::Itertools;

type PartFn = fn(&mut Stacks, &Instruction, usize) -> Vec<char>;

fn main() {
    let file = std::fs::read_to_string(Args::parse().filename).expect("Could not read file.");
    let input: Vec<&str> = file.split("\n\n").collect();
    let stacks = input[0];
    let instructions = input[1];

    Stacks::from(stacks)
        .do_instructions(instructions, part1)
        .display(1);
    Stacks::from(stacks)
        .do_instructions(instructions, part2)
        .display(2);
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

    fn do_instructions(mut self, instructions: &str, part: PartFn) -> Stacks {
        instructions.lines().for_each(|line| {
            let instruction = Instruction::from(line);
            let start = self.stacks[instruction.from].len() - instruction.count;
            let drained = part(&mut self, &instruction, start);
            self.stacks[instruction.to].extend(drained);
        });
        self
    }

    fn display(&self, part: usize) {
        print!("Part {}: ", part);
        self.stacks
            .iter()
            .for_each(|v| print!("{}", v.last().unwrap()));
        println!();
    }
}

fn part1(stacks: &mut Stacks, instruction: &Instruction, start: usize) -> Vec<char> {
    stacks.stacks[instruction.from]
        .drain(start..)
        .rev()
        .collect()
}

fn part2(stacks: &mut Stacks, instruction: &Instruction, start: usize) -> Vec<char> {
    stacks.stacks[instruction.from].drain(start..).collect()
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
