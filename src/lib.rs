use std::fs;
use std::io;
use std::io::{BufRead, BufReader};

pub fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

pub fn string_to_int(input: &Vec<String>) -> Vec<i32> {
    // Using an iterator, convert each string to an integer
    let numbers: Vec<i32> = input.iter().map(|s| s.parse().unwrap()).collect();
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_read_input() {
        let _vec = file_to_vec("input/day01_sample.txt").unwrap();
    }

    #[test]
    #[should_panic]
    fn panics_on_bad_filename() {
        let _vec = file_to_vec("input/not-a-real-file.txt").unwrap();
    }
}