use common::files;
use regex::Regex;

fn main() {
    let s = files::get_file_as_string("day09.txt");

    println!("part1: {}", decompress(&s).len());
    println!("part2: {}", decompress_v2(&s).len());
}

fn decompress(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '(' {
            let mut marker = String::new();

            while chars.peek().is_some() && chars.peek().unwrap() != &')' {
                marker.push(chars.next().unwrap());
            }

            let split: Vec<&str> = marker.split("x").collect();
            let num_chars = split[0].parse::<u32>().unwrap();
            let repeat_times = split[1].parse::<u32>().unwrap();

            // Consume ')'
            chars.next();

            let mut temp = String::new();
            for _ in 0..num_chars {
                temp.push(chars.next().unwrap());
            }

            for _ in 0..repeat_times {
                result.push_str(&temp);
            }
        } else if c.is_whitespace() {
            // ignore
        } else {
            result.push(c);
        }
    }

    result
}

fn decompress_v2(s: &str) -> String {
    let marker_regex = Regex::new(r"\(.*\)").unwrap();

    let mut decompressed = s.to_string();
    while marker_regex.is_match(&decompressed) {
        decompressed = decompress(&decompressed);
    }

    decompressed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress() {
        assert_eq!(decompress("ADVENT"), "ADVENT");
        assert_eq!(decompress("A(1x5)BC"), "ABBBBBC");
        assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
        assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A");
        assert_eq!(decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
    }

    #[test]
    fn test_decompress_v2() {
        assert_eq!(decompress_v2("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(decompress_v2("X(8x2)(3x3)ABCY"), "XABCABCABCABCABCABCY");
        assert_eq!(decompress_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), String::from_utf8(vec![b'A'; 241920]).unwrap());
    }
}