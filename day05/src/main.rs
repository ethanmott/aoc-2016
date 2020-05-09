fn main() {
    let input = "reyedfim";

    println!("part1: {}", solve_part1(input));
    println!("part2: {}", solve_part2(input));
}

fn solve_part1(prefix: &str) -> String {
    let mut i = -1;

    let mut result = String::new();

    for _ in 0..8 {
        let mut md5 = String::new();

        while !md5.starts_with("00000") {
            i += 1;

            let candidate = format!("{}{}", prefix, i);
            md5 = format!("{:?}", md5::compute(candidate));
        }

        result.push(md5.chars().nth(5).unwrap());
    }

    result
}

fn solve_part2(prefix: &str) -> String {
    let mut i = -1;

    let mut result = String::from("########");

    while result.contains("#") {
        let mut md5 = String::new();

        while !md5.starts_with("00000") {
            i += 1;

            let candidate = format!("{}{}", prefix, i);
            md5 = format!("{:?}", md5::compute(candidate));
        }

        let sixth_char = md5.chars().nth(5).unwrap();
        if !sixth_char.is_numeric() {
            continue;
        }

        let index = sixth_char.to_digit(10).unwrap() as usize;
        if index < 8 && result.chars().nth(index).unwrap() == '#' {
            result.replace_range(index..index+1, &md5.chars().nth(6).unwrap().to_string());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1("abc"), "18f47a30".to_string());
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2("abc"), "05ace8e3".to_string());
    }
}