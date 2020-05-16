use common::files;
use std::fmt::{Display, Formatter, Result};

fn main() {
    let instructions: Vec<Instruction> = files::get_file_lines("day08.txt").iter()
        .map(|l| Instruction::from_line(l))
        .collect();

    let mut screen = Screen::new(50, 6);

    for i in instructions {
        screen.apply_instruction(&i);
    }

    let count = screen.pixels.iter()
        .filter(|&p| p == &true)
        .count();

    println!("part1: {}", count);
    println!("part2:\n{}", screen);
}

#[derive(Debug)]
struct Screen {
    width: usize,
    height: usize,
    pixels: Vec<bool>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Screen {
            width,
            height,
            pixels: vec![false; width * height],
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.pixels[(x * self.height) + y]
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        self.pixels[(x * self.height) + y] = value;
    }

    fn apply_instruction(&mut self, i: &Instruction) {
        match i {
            Instruction::Rect { width, height } => {
                for x in 0..*width {
                    for y in 0..*height {
                        self.set(x, y, true);
                    }
                }
            },
            Instruction::RotateRow { index, distance } => {
                for _ in 0..*distance {
                    let mut previous = self.get(self.width - 1, *index);
                    for x in 0..self.width {
                        let current = self.get(x, *index);
                        self.set(x, *index, previous);
                        previous = current;
                    }
                }
            },
            Instruction::RotateColumn { index, distance } => {
                for _ in 0..*distance {
                    let mut previous = self.get(*index, self.height - 1);
                    for y in 0..self.height {
                        let current = self.get(*index, y);
                        self.set(*index, y, previous);
                        previous = current;
                    }
                }
            },
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut result = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let c = match self.get(x, y) {
                    true => '#',
                    false => '.',
                };

                result.push_str(&format!("{} ", c));
            }
            result.push('\n');
        }

        write!(f, "{}", result)
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Rect { width: usize, height: usize },
    RotateRow { index: usize, distance: usize },
    RotateColumn { index: usize, distance: usize },
}

impl Instruction {
    fn from_line(line: &String) -> Instruction {
        let split: Vec<&str> = line.split(" ").collect();

        match split[0] {
            "rect" => {
                let dimensions_split: Vec<&str> = split[1].split("x").collect();

                Instruction::Rect {
                    width: dimensions_split[0].parse::<usize>().unwrap(),
                    height: dimensions_split[1].parse::<usize>().unwrap(),
                }
            },
            "rotate" => {
                let index_split: Vec<&str> = split[2].split("=").collect();
                let index = index_split[1].parse::<usize>().unwrap();
                let distance = split[4].parse::<usize>().unwrap();

                match split[1] {
                    "row" => {
                        Instruction::RotateRow {
                            index,
                            distance,
                        }
                    },
                    "column" => {
                        Instruction::RotateColumn {
                            index,
                            distance,
                        }
                    },
                    x => panic!("Invalid rotation type \"{}\" specified.", x),
                }
            },
            x => panic!("Invalid instruction \"{}\" specified.", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Instruction::{Rect, RotateRow, RotateColumn};

    #[test]
    fn test_instruction_from_line() {
        assert_eq!(
            Instruction::from_line(&String::from("rect 4x1")),
            Rect { width: 4, height: 1 }
        );
        assert_eq!(
            Instruction::from_line(&String::from("rotate row y=0 by 5")),
            RotateRow { index: 0, distance: 5 }
        );
        assert_eq!(
            Instruction::from_line(&String::from("rotate column x=30 by 1")),
            RotateColumn { index: 30, distance: 1 }
        );
    }
}