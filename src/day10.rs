use clap::Parser;

fn main() {
    let mut cpu = Cpu::new();
    std::fs::read_to_string(Args::parse().filename)
        .expect("Could not read file.")
        .lines()
        .for_each(|line| {
            let instruction = Instruction::from_line(line);
            cpu.exec_instruction(&instruction);
        });

    println!("Part 1: {}", cpu.part1);
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn from_line(line: &str) -> Instruction {
        let mut line = line.split_whitespace();
        let ins = line.next().unwrap();
        match ins {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(line.next().unwrap().parse().unwrap()),
            _ => panic!("Invalid instruction"),
        }
    }
}

struct Cpu {
    register: i32,
    clock: i32,
    part1: i32,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            register: 1,
            clock: 0,
            part1: 0,
        }
    }

    fn exec_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => self.cycle(),
            Instruction::Addx(n) => {
                self.cycle();
                self.cycle();
                self.register += n;
            }
        }
    }

    fn cycle(&mut self) {
        self.clock += 1;

        if (self.clock - 20) % 40 == 0 {
            self.part1 += self.clock * self.register;
        }

        if (((self.clock - 1) % 40) - (self.register % 40)).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }

        if self.clock % 40 == 0 {
            println!();
        }
    }
}

#[derive(Parser)]
struct Args {
    filename: String,
}
