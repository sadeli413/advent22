use clap::Parser;

fn main() {
    let instructions: Vec<Instruction> = std::fs::read_to_string(Args::parse().filename)
        .expect("Could not read file.")
        .lines()
        .map(Instruction::from_line)
        .collect();

    let mut grid = Grid::new(2);
    instructions
        .iter()
        .for_each(|instruction| grid.exec_instruction(instruction));
    println!("Part 1: {}", count(&grid));

    let mut grid = Grid::new(10);
    instructions
        .iter()
        .for_each(|instruction| grid.exec_instruction(instruction));
    println!("Part 2: {}", count(&grid));
}

fn count(grid: &Grid) -> usize {
    grid.grid
        .iter()
        .map(|row| row.iter().filter(|&&b| b).count())
        .sum()
}

#[derive(Debug)]
enum Instruction {
    L(usize),
    R(usize),
    U(usize),
    D(usize),
}

impl Instruction {
    fn from_line(line: &str) -> Instruction {
        let mut line = line.split_whitespace();
        let instruction = line.next().unwrap();
        let num = line.next().unwrap().parse().unwrap();
        match instruction {
            "L" => Instruction::L(num),
            "R" => Instruction::R(num),
            "U" => Instruction::U(num),
            "D" => Instruction::D(num),
            _ => panic!("Invalid Instruction"),
        }
    }
}

struct Grid {
    grid: Vec<Vec<bool>>,
    rope: Vec<(usize, usize)>,
}

impl Grid {
    fn new(len: usize) -> Grid {
        Grid {
            grid: vec![vec![true]],
            rope: vec![(0, 0); len],
        }
    }

    fn exec_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::L(n) => self.move_l(*n),
            Instruction::R(n) => self.move_r(*n),
            Instruction::U(n) => self.move_u(*n),
            Instruction::D(n) => self.move_d(*n),
        };
        // self.display();
    }

    fn move_l(&mut self, n: usize) {
        (0..n).for_each(|_| {
            self.rope[0].0 = self.rope[0].0.checked_sub(1).unwrap_or_else(|| {
                self.grid.iter_mut().for_each(|row| row.insert(0, false));
                self.rope.iter_mut().skip(1).for_each(|knot| knot.0 += 1);
                self.rope[0].0
            });
            self.tail_follows();
        })
    }

    fn move_r(&mut self, n: usize) {
        (0..n).for_each(|_| {
            self.rope[0].0 += 1;
            if self.rope[0].0 >= self.grid[0].len() {
                self.grid.iter_mut().for_each(|row| row.push(false));
            }
            self.tail_follows();
        })
    }

    fn move_u(&mut self, n: usize) {
        (0..n).for_each(|_| {
            self.rope[0].1 = self.rope[0].1.checked_sub(1).unwrap_or_else(|| {
                self.grid.insert(0, vec![false; self.grid[0].len()]);
                self.rope.iter_mut().skip(1).for_each(|knot| knot.1 += 1);
                self.rope[0].1
            });
            self.tail_follows();
        })
    }

    fn move_d(&mut self, n: usize) {
        (0..n).for_each(|_| {
            self.rope[0].1 += 1;
            if self.rope[0].1 >= self.grid.len() {
                self.grid.push(vec![false; self.grid[0].len()]);
            }
            self.tail_follows();
        })
    }

    fn tail_follows(&mut self) {
        (1..self.rope.len()).for_each(|i| {
            let (tx, ty) = self.rope[i];
            let (hx, hy) = self.rope[i - 1];
            let tx = tx as i32;
            let ty = ty as i32;
            let hx = hx as i32;
            let hy = hy as i32;
            let delta_x = hx - tx;
            let delta_y = hy - ty;

            if delta_x == 2 {
                self.rope[i].0 += 1;
                self.rope[i].1 = weird_add(self.rope[i].1, delta_y);
            } else if delta_x == -2 {
                self.rope[i].0 -= 1;
                self.rope[i].1 = weird_add(self.rope[i].1, delta_y);
            } else if delta_y == 2 {
                self.rope[i].1 += 1;
                self.rope[i].0 = weird_add(self.rope[i].0, delta_x);
            } else if delta_y == -2 {
                self.rope[i].1 -= 1;
                self.rope[i].0 = weird_add(self.rope[i].0, delta_x);
            }
        });

        self.grid[self.rope[self.rope.len() - 1].1][self.rope[self.rope.len() - 1].0] = true;
    }

    // fn display(&self) {
    //     self.grid.iter().enumerate().for_each(|(y, row)| {
    //         row.iter().enumerate().for_each(|(x, _)| {
    //             if self.rope[0] == (x, y) {
    //                 print!("H ")
    //             } else {
    //                 self.rope.iter().enumerate().skip(1).find_map(|(i, (tx, ty))| {
    //                     (tx == &x && ty == &y)
    //                         .then_some(i)
    //                 })
    //                 .map_or_else(|| print!(". "), |i| print!("{} ", i));
    //             }
    //         });
    //         println!();
    //     });
    //     println!();
    //     println!()
    // }
}

fn weird_add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.unsigned_abs().min(1) as usize
    } else {
        u + i.min(1) as usize
    }
}

#[derive(Parser)]
struct Args {
    filename: String,
}
