use common::files;
use std::collections::HashMap;

fn main() {
    let lines = files::get_file_lines("day06.txt");

    println!("part1: {}", solve_part1(&lines));
    println!("part1: {}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<String>) -> String {
    let maps = build_frequency_maps(&lines);
    let mut result = String::new();

    for map in maps {
        let (c, _) = map.iter()
            .max_by_key(|(_, &count)| count)
            .unwrap();

        result.push(c.clone());
    }

    result
}

fn solve_part2(lines: &Vec<String>) -> String {
    let maps = build_frequency_maps(&lines);
    let mut result = String::new();

    for map in maps {
        let (c, _) = map.iter()
            .min_by_key(|(_, &count)| count)
            .unwrap();

        result.push(c.clone());
    }

    result
}

fn build_frequency_maps(lines: &Vec<String>) -> Vec<HashMap<char, i32>> {
    let mut maps = Vec::new();

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if maps.get(i).is_none() {
                maps.push(HashMap::new());
            }
            let mut frequency_map = maps.get_mut(i).unwrap();

            *frequency_map.entry(c).or_insert(0) += 1;
        }
    }

    maps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let lines = vec![
            "eedadn".to_string(),
            "drvtee".to_string(),
            "eandsr".to_string(),
            "raavrd".to_string(),
            "atevrs".to_string(),
            "tsrnev".to_string(),
            "sdttsa".to_string(),
            "rasrtv".to_string(),
            "nssdts".to_string(),
            "ntnada".to_string(),
            "svetve".to_string(),
            "tesnvt".to_string(),
            "vntsnd".to_string(),
            "vrdear".to_string(),
            "dvrsen".to_string(),
            "enarar".to_string(),
        ];

        assert_eq!(solve_part1(&lines), "easter".to_string());
    }

    #[test]
    fn test_solve_part2() {
        let lines = vec![
            "eedadn".to_string(),
            "drvtee".to_string(),
            "eandsr".to_string(),
            "raavrd".to_string(),
            "atevrs".to_string(),
            "tsrnev".to_string(),
            "sdttsa".to_string(),
            "rasrtv".to_string(),
            "nssdts".to_string(),
            "ntnada".to_string(),
            "svetve".to_string(),
            "tesnvt".to_string(),
            "vntsnd".to_string(),
            "vrdear".to_string(),
            "dvrsen".to_string(),
            "enarar".to_string(),
        ];

        assert_eq!(solve_part2(&lines), "advent".to_string());
    }
}