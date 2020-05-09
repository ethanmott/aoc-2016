use common::files;
use std::borrow::{BorrowMut, Borrow};

fn main() {
    let lines = files::get_file_lines("day02.txt");

    println!("part1: {}", solve(Keypad::new_part1().borrow_mut(), lines.borrow()));
    println!("part2: {}", solve(Keypad::new_part2().borrow_mut(), lines.borrow()));
}

fn solve(keypad: &mut Keypad, lines: &Vec<String>) -> String {
    let mut code = String::new();

    for line in lines {
        line.chars()
            .map(|c| Direction::from_char(c))
            .for_each(|d| keypad.do_move(d));

        code.push(keypad.current_char());
    }

    code
}

#[derive(Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'U' => Direction::UP,
            'R' => Direction::RIGHT,
            'D' => Direction::DOWN,
            'L' => Direction::LEFT,
            x => panic!("Invalid direction provided: {}", x),
        }
    }
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Keypad<'a> {
    x: usize,
    y: usize,
    buttons: &'a [&'a [Option<char>]]
}

impl<'a> Keypad<'a> {
    fn new_part1() -> Keypad<'static> {
        Keypad {
            x: 1,
            y: 1,
            buttons: &[
                &[Some('1'), Some('2'), Some('3')],
                &[Some('4'), Some('5'), Some('6')],
                &[Some('7'), Some('8'), Some('9')],
            ]
        }
    }

    fn new_part2() -> Keypad<'static> {
        Keypad {
            x: 0,
            y: 2,
            buttons: &[
                &[None,      None,      Some('1'), None,      None     ],
                &[None,      Some('2'), Some('3'), Some('4'), None     ],
                &[Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
                &[None,      Some('A'), Some('B'), Some('C'), None     ],
                &[None,      None,      Some('D'), None,      None     ],
            ]
        }
    }

    fn do_move(&mut self, d: Direction) {
        match d {
            Direction::UP => {
                if self.y > 0 && self.button_at(self.x, self.y - 1).is_some() {
                    self.y -= 1;
                }
            },
            Direction::DOWN => {
                if self.y < self.buttons.len() - 1 && self.button_at(self.x, self.y + 1).is_some() {
                    self.y += 1;
                }
            },
            Direction::RIGHT => {
                if self.x < self.buttons.get(self.y).unwrap().len() - 1 && self.button_at(self.x + 1, self.y).is_some() {
                    self.x += 1;
                }
            },
            Direction::LEFT => {
                if self.x > 0 && self.button_at(self.x - 1, self.y).is_some() {
                    self.x -= 1;
                }
            },
        }
    }

    fn current_char(&self) -> char {
        self.button_at(self.x, self.y).unwrap()
    }

    fn button_at(&self, x: usize, y: usize) -> Option<char> {
        self.buttons[y][x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = vec![
            "ULL".to_string(),
            "RRDDD".to_string(),
            "LURDL".to_string(),
            "UUUUD".to_string(),
        ];

        assert_eq!(
            solve(Keypad::new_part1().borrow_mut(), input.borrow()),
            "1985".to_string()
        );
        assert_eq!(
            solve(Keypad::new_part2().borrow_mut(), input.borrow()),
            "5DB3".to_string()
        );
    }
}