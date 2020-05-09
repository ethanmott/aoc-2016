use common::files;

fn main() {
    let lines = files::get_file_lines("day03.txt");

    println!("part1: {}", solve_part1(&lines));
    println!("part2: {}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<String>) -> u32 {
    let mut count = 0;

    for line in lines {
        let sides: Vec<u32> = line.split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        if is_valid_triangle(&sides) {
            count = count + 1;
        }
    }

    count
}

fn solve_part2(lines: &Vec<String>) -> u32 {
    let mut count = 0;

    for chunk in lines.chunks(3) {
        let sides: Vec<u32> = chunk.iter()
            .flat_map(|line| line.split_whitespace())
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        for x in 0..3 {
            let triangle: Vec<u32> = sides.iter()
                .skip(x)
                .step_by(3)
                .cloned()
                .collect();

            if is_valid_triangle(&triangle) {
                count = count + 1;
            }
        }
    }

    count
}

fn is_valid_triangle(sides: &Vec<u32>) -> bool {
    return sides[0] + sides[1] > sides[2]
        && sides[0] + sides[2] > sides[1]
        && sides[1] + sides[2] > sides[0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_triangle() {
        assert_eq!(is_valid_triangle(&vec![5, 10, 25]), false);
        assert_eq!(is_valid_triangle(&vec![5, 10, 5]), false);
        assert_eq!(is_valid_triangle(&vec![5, 9, 5]), true);
    }

    #[test]
    fn test_solve_part1() {
        let lines = vec![
            "101 301 501".to_string(),
            "102 302 502".to_string(),
            "103 303 503".to_string(),
            "201 401 601".to_string(),
            "202 402 602".to_string(),
            "203 403 603".to_string(),
        ];

        assert_eq!(solve_part1(&lines), 3);
    }

    #[test]
    fn test_solve_part2() {
        let lines = vec![
            "101 301 501".to_string(),
            "102 302 502".to_string(),
            "103 303 503".to_string(),
            "201 401 601".to_string(),
            "202 402 602".to_string(),
            "203 403 603".to_string(),
        ];

        assert_eq!(solve_part2(&lines), 6);
    }
}