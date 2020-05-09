use common::files;
use regex::Regex;
use std::collections::{HashMap};
use std::cmp::Ordering;

fn main() {
    let lines = files::get_file_lines("day04.txt");

    let rooms: Vec<Room> = lines.iter()
        .map(|l| Room::from_line(l))
        .collect();

    let sector_sum: u32 = rooms.iter()
        .filter(|r| r.is_real())
        .map(|r| r.sector_id)
        .sum();

    println!("part1: {}", sector_sum);

    let part2_sector_id = rooms.iter()
        .filter(|r| r.is_real())
        .find(|r| r.decrypted_name == "northpole object storage".to_string())
        .map(|r| r.sector_id);

    println!("part2: {:?}", part2_sector_id);
}

#[derive(Debug)]
struct Room {
    name: String,
    decrypted_name: String,
    sector_id: u32,
    checksum: String,
}

pub fn decrypt_name(name: &str, sector_id: u32) -> String {
    name
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                (b'a' + (c as u8 + (sector_id % 26) as u8 - b'a') % 26) as char
            } else if c == '-' {
                ' '
            } else {
                c
            }
        })
        .collect()
}

impl Room {
    fn from_line(line: &String) -> Self {
        let re = Regex::new(r"^(.*)-([0-9]+)\[(.*)]$").unwrap();

        let caps = re.captures(line).unwrap();

        let name = caps.get(1).unwrap().as_str();
        let sector_id = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let checksum = caps.get(3).unwrap().as_str().to_string();
        let decrypted_name = decrypt_name(&name, sector_id);

        Room {
            name: name.replace("-", ""),
            decrypted_name,
            sector_id,
            checksum,
        }
    }

    fn is_real(&self) -> bool {
        if self.checksum.len() > 5 {
            return false;
        }

        let mut frequency_map = HashMap::new();

        for c in self.name.chars() {
            *frequency_map.entry(c).or_insert(0) += 1;
        }

        let mut previous = self.checksum.chars().next().unwrap();

        for current in self.checksum.chars() {
            if frequency_map.get(&current).is_none() {
                return false;
            }

            let previous_char_count = frequency_map.get(&previous).unwrap();
            let current_char_count = frequency_map.get(&current).unwrap();

            if current_char_count > previous_char_count {
                return false;
            }

            if previous_char_count == current_char_count && current.cmp(&previous) == Ordering::Less {
                return false;
            }

            previous = current;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room() {
        assert_eq!(Room::from_line(&"aaaaa-bbb-z-y-x-123[abxyz]".to_string()).is_real(), true);
        assert_eq!(Room::from_line(&"a-b-c-d-e-f-g-h-987[abcde]".to_string()).is_real(), true);
        assert_eq!(Room::from_line(&"not-a-real-room-404[oarel]".to_string()).is_real(), true);
        assert_eq!(Room::from_line(&"totally-real-room-200[decoy]".to_string()).is_real(), false);
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt_name("qzmt-zixmtkozy-ivhz", 343), "very encrypted name");
    }
}