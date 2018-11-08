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

pub fn convert_address(address: u16) -> [bool; 15] {
        let mut result = [false; 15];
        for i in 0..15 {
                result[i] = (1 << i) & address != 0;
        }
        result
}

pub fn convert16(x: i16) -> [bool; 16] {
        let mut result = [false; 16];
        for i in 0..15 {
                result[i] = (1 << i) & x != 0;
        }
        result[15] = x < 0;
        result
}

pub fn convert16_str(x: &str) -> [bool; 16] {
        let mut res = [false; 16];
        for i in 0..16 {
                res[i] = &x[(15 - i)..(15 - i + 1)] == "1";
        }
        res
}

pub fn load_hack_binary(content: &str) -> Vec<[bool; 16]> {
        let mut result = vec![];
        for line in content.trim().split('\n') {
                assert_eq!(line.len(), 16, "{}", line);
                let mut input = [false; 16];
                for (i, c) in line.chars().rev().enumerate() {
                        input[i] = c == '1';
                        assert!(c == '1' || c == '0');
                }
                result.push(input);
        }
        result
}
