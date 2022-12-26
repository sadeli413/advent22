use clap::Parser;
use itertools::FoldWhile;
use itertools::Itertools;
use std::vec::IntoIter;

const RADIX: u32 = 10;

fn main() {
    let input = std::fs::read_to_string(Args::parse().filename).expect("Could not read file.");

    let mut forest: Vec<Vec<Tree>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Tree::new(c.to_digit(RADIX).unwrap().into()))
                .collect()
        })
        .collect();

    // Check left-right visibility
    forest.iter_mut().for_each(|row| {
        row.iter_mut().fold(-1, |tallest, tree| {
            if tree.height > tallest {
                tree.is_left = true;
                tree.height
            } else {
                tallest
            }
        });

        row.iter_mut().rev().fold(-1, |tallest, tree| {
            if tree.height > tallest {
                tree.is_right = true;
                tree.height
            } else {
                tallest
            }
        });
    });

    let mut forest = transpose(forest);

    // Check top-bottom visibility
    forest.iter_mut().for_each(|row| {
        row.iter_mut().fold(-1, |tallest, tree| {
            if tree.height > tallest {
                tree.is_top = true;
                tree.height
            } else {
                tallest
            }
        });

        row.iter_mut().rev().fold(-1, |tallest, tree| {
            if tree.height > tallest {
                tree.is_bottom = true;
                tree.height
            } else {
                tallest
            }
        });
    });

    println!("Part 1: {}", part1(&forest));

    // Check top-bottom view scores
    forest.iter_mut().for_each(|column| {
        let read_column = column.clone();
        column.iter_mut().enumerate().for_each(|(i, tree)| {
            tree.view_top = read_column
                .iter()
                .enumerate()
                .filter(|(j, _)| j < &i)
                .rev()
                .fold_while(0, |view, (_, other)| {
                    if tree.height > other.height {
                        FoldWhile::Continue(view + 1)
                    } else {
                        FoldWhile::Done(view + 1)
                    }
                })
                .into_inner();
        });

        column.iter_mut().enumerate().for_each(|(i, tree)| {
            tree.view_bottom = read_column
                .iter()
                .enumerate()
                .filter(|(j, _)| j > &i)
                .fold_while(0, |view, (_, other)| {
                    if tree.height > other.height {
                        FoldWhile::Continue(view + 1)
                    } else {
                        FoldWhile::Done(view + 1)
                    }
                })
                .into_inner();
        });
    });

    let mut forest = transpose(forest);

    // Check left-right view scores
    forest.iter_mut().for_each(|row| {
        let read_row = row.clone();
        row.iter_mut().enumerate().for_each(|(i, tree)| {
            tree.view_left = read_row
                .iter()
                .enumerate()
                .filter(|(j, _)| j < &i)
                .rev()
                .fold_while(0, |view, (_, other)| {
                    if tree.height > other.height {
                        FoldWhile::Continue(view + 1)
                    } else {
                        FoldWhile::Done(view + 1)
                    }
                })
                .into_inner();
        });

        row.iter_mut().enumerate().for_each(|(i, tree)| {
            tree.view_right = read_row
                .iter()
                .enumerate()
                .filter(|(j, _)| j > &i)
                .fold_while(0, |view, (_, other)| {
                    if tree.height > other.height {
                        FoldWhile::Continue(view + 1)
                    } else {
                        FoldWhile::Done(view + 1)
                    }
                })
                .into_inner();
        });
    });

    println!("Part 2: {}", part2(&forest));
}

fn part1(forest: &[Vec<Tree>]) -> usize {
    forest
        .iter()
        .map(|row| {
            row.iter()
                .map(|tree| usize::from(tree.is_visible()))
                .sum::<usize>()
        })
        .sum()
}

fn part2(forest: &[Vec<Tree>]) -> i64 {
    forest
        .iter()
        .map(|row| row.iter().map(|tree| tree.score()).max())
        .max()
        .flatten()
        .unwrap()
}

#[derive(Debug, Clone)]
struct Tree {
    height: i64,
    view_top: i64,
    view_bottom: i64,
    view_left: i64,
    view_right: i64,
    is_top: bool,
    is_bottom: bool,
    is_left: bool,
    is_right: bool,
}

impl Tree {
    fn new(height: i64) -> Tree {
        Tree {
            height,
            view_top: 0,
            view_bottom: 0,
            view_left: 0,
            view_right: 0,
            is_top: false,
            is_bottom: false,
            is_left: false,
            is_right: false,
        }
    }

    fn is_visible(&self) -> bool {
        self.is_top || self.is_bottom || self.is_left || self.is_right
    }

    fn score(&self) -> i64 {
        self.view_top * self.view_bottom * self.view_left * self.view_right
    }
}

fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let size = matrix[0].len();
    let mut iters: Vec<IntoIter<T>> = matrix.into_iter().map(|v| v.into_iter()).collect();
    (0..size)
        .map(|_| iters.iter_mut().map(|t| t.next().unwrap()).collect())
        .collect()
}

#[derive(Parser)]
struct Args {
    filename: String,
}
