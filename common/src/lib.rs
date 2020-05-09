pub mod files {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    pub fn get_file_bytes(file_name: &str) -> Vec<u8> {
        let mut file = File::open(format!("input/{}", file_name))
            .expect(format!("Couldn't find file with name: {}", file_name).as_str());

        let mut buf = Vec::new();
        file.read_to_end(&mut buf).expect("Error reading file bytes.");

        buf
    }

    pub fn get_file_lines(file_name: &str) -> Vec<String> {
        let file = File::open(format!("input/{}", file_name))
            .expect(format!("Couldn't find file with name: {}", file_name).as_str());

        BufReader::new(file)
            .lines()
            .map(|l| l.expect("Error parsing line."))
            .collect()
    }
}
