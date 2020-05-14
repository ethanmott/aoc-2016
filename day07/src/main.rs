use common::files;
use regex::Regex;

fn main() {
    let lines = files::get_file_lines("day07.txt");

    let ip_addresses: Vec<IpAddress> = lines.iter()
        .map(|l| IpAddress::from_line(l))
        .collect();

    let part1_count = ip_addresses.iter()
        .filter(|ip| ip.supports_tls())
        .count();

    let part2_count = ip_addresses.iter()
        .filter(|ip| ip.supports_ssl())
        .count();

    println!("part1: {}", part1_count);
    println!("part2: {}", part2_count);
}

#[derive(Debug)]
struct IpAddress {
    full: String,
    supernet_sequences: Vec<String>,
    hypernet_sequences: Vec<String>,
}

impl IpAddress {
    fn from_line(line: &String) -> Self {
        let re = Regex::new(r"\[([a-z]+)]").unwrap();

        let supernet_sequences: Vec<String> = re.split(line)
            .map(|s| s.to_string())
            .collect();

        let mut hypernet_sequences = Vec::new();

        let mut stripped = line.clone();
        while let Some(caps) = re.captures(&stripped) {
            hypernet_sequences.push(caps.get(1).unwrap().as_str().to_string());
            stripped = stripped.replace(caps.get(0).unwrap().as_str(), "");
        }

        IpAddress {
            full: line.clone(),
            supernet_sequences,
            hypernet_sequences,
        }
    }

    fn supports_ssl(&self) -> bool {
        let abas = get_abas(&self.supernet_sequences);

        let hypernet_contains_bab = abas.iter()
            .any(|aba| {
                let mut bab = aba[1..].to_string();
                bab.push(aba.chars().nth(1).unwrap());

                self.hypernet_sequences.iter()
                    .any(|s| s.contains(&bab))
            });

        abas.len() > 0 && hypernet_contains_bab
    }

    fn supports_tls(&self) -> bool {
        contains_abba(&self.supernet_sequences) && !contains_abba(&self.hypernet_sequences)
    }
}

fn contains_abba(strings: &Vec<String>) -> bool {
    for s in strings.iter() {
        for (i, _) in s.chars().enumerate() {
            if s.chars().nth(i + 3).is_none() {
                continue;
            }

            let bytes = &s[i..=i+3].as_bytes();

            if bytes[0] == bytes[3] && bytes[1] == bytes[2] && bytes[0] != bytes[1] {
                return true;
            }
        }
    }

    false
}

fn get_abas(strings: &Vec<String>) -> Vec<String> {
    let mut results = Vec::new();

    for s in strings.iter() {
        for (i, _) in s.chars().enumerate() {
            if s.chars().nth(i + 2).is_none() {
                continue;
            }

            let bytes = &s[i..=i+2].as_bytes();

            if bytes[0] == bytes[2] && bytes[0] != bytes[1] {
                results.push(s[i..=i+2].to_string());
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supports_tls() {
        assert_eq!(IpAddress::from_line(&String::from("abba[mnop]qrst")).supports_tls(), true);
        assert_eq!(IpAddress::from_line(&String::from("abcd[bddb]xyyx")).supports_tls(), false);
        assert_eq!(IpAddress::from_line(&String::from("aaaa[qwer]tyui")).supports_tls(), false);
        assert_eq!(IpAddress::from_line(&String::from("ioxxoj[asdfgh]zxcvbn")).supports_tls(), true);
    }

    #[test]
    fn test_supports_ssl() {
        assert_eq!(IpAddress::from_line(&String::from("aba[bab]xyz")).supports_ssl(), true);
        assert_eq!(IpAddress::from_line(&String::from("xyx[xyx]xyx")).supports_ssl(), false);
        assert_eq!(IpAddress::from_line(&String::from("aaa[kek]eke")).supports_ssl(), true);
        assert_eq!(IpAddress::from_line(&String::from("zazbz[bzb]cdb")).supports_ssl(), true);
    }
}