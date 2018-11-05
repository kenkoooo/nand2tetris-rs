use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn parse_test_cases(s: &str) -> Vec<Vec<&str>> {
        s.trim().split('\n')
                .map(|line| line.split('|').map(|v| v.trim()).collect())
                .collect()
}

pub fn read_file(filename: &str) -> io::Result<String> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
}

pub fn read_test_data(filename: &str) -> io::Result<Vec<Vec<String>>> {
        let text = read_file(filename)?;
        Ok(parse_test_cases(&text)
                .iter()
                .map(|t| t.iter().map(|&t| t.to_owned()).collect())
                .collect())
}
