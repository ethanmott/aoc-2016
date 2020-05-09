use common::files;
use std::collections::{HashSet};

fn main() {
    let lines = files::get_file_lines("day01.txt");

    let instructions: Vec<Instruction> = lines.get(0).unwrap()
        .split(", ")
        .map(|s| Instruction::from_str(s))
        .collect();

    println!("part1: {}", part1(&instructions));
    println!("part2: {}", part2(&instructions));

}

fn part1(instructions: &Vec<Instruction>) -> i32 {
    let mut state = State::new();

    for instruction in instructions {
        state.apply_instruction(instruction);
    }

    state.get_distance_from_hq()
}

fn part2(instructions: &Vec<Instruction>) -> i32 {
    let mut state = State::new();

    for instruction in instructions {
        state.apply_instruction(instruction);
    }

    match state.seen_twice_distance {
        Some(x) => x,
        None => panic!("No location found twice.")
    }
}

#[derive(Debug)]
enum TurnDirection {
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Instruction {
    direction: TurnDirection,
    num_blocks: i32,
}

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        Instruction {
            direction: match s.chars().nth(0) {
                Some('L') => TurnDirection::LEFT,
                Some('R') => TurnDirection::RIGHT,
                Some(c) => panic!("Invalid turn direction provided: {}", c),
                None => panic!("No character found."),
            },
            num_blocks: s[1..].parse::<i32>().unwrap(),
        }
    }
}

enum Heading {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

struct State {
    x: i32,
    y: i32,
    heading: Heading,
    seen: HashSet<(i32, i32)>,
    seen_twice_distance: Option<i32>,
}

impl State {
    fn new() -> State {
        State {
            x: 0,
            y: 0,
            heading: Heading::NORTH,
            seen: HashSet::new(),
            seen_twice_distance: None,
        }
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        self.heading = match self.heading {
            Heading::NORTH => match instruction.direction {
                TurnDirection::LEFT => Heading::WEST,
                TurnDirection::RIGHT => Heading::EAST,
            },
            Heading::EAST => match instruction.direction {
                TurnDirection::LEFT => Heading::NORTH,
                TurnDirection::RIGHT => Heading::SOUTH,
            },
            Heading::SOUTH => match instruction.direction {
                TurnDirection::LEFT => Heading::EAST,
                TurnDirection::RIGHT => Heading::WEST,
            },
            Heading::WEST => match instruction.direction {
                TurnDirection::LEFT => Heading::SOUTH,
                TurnDirection::RIGHT => Heading::NORTH,
            },
        };

        for _ in 0..instruction.num_blocks {
            match self.heading {
                Heading::NORTH => self.y += 1,
                Heading::EAST => self.x += 1,
                Heading::SOUTH => self.y -= 1,
                Heading::WEST => self.x -= 1,
            }

            let coordinates = (self.x, self.y);

            if self.seen.contains(&coordinates) && self.seen_twice_distance.is_none() {
                self.seen_twice_distance = Some(self.get_distance_from_hq());
            }
            self.seen.insert(coordinates);
        }
    }

    fn get_distance_from_hq(&self) -> i32 {
        (0 - self.x).abs() + (0 - self.y).abs()
    }
}