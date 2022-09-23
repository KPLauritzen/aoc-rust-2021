// https://adventofcode.com/2021/day/1

use aoc_rust_2021::{file_to_vec, string_to_int};
fn main() {
    let filename = "input/day01.txt".to_string();
    let input = file_to_vec(filename).unwrap();
    let part_1_result = part_1(input.clone());
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(input.clone());
    println!("Part 2: {}", part_2_result);
}

fn part_1(input: Vec<String>) -> i32 {
    let depths = string_to_int(input);
    // iterate over neighboring values and count increases
    let mut count = 0;
    for i in 0..depths.len() {
        // First measurement does not have a previous value
        if i == 0 {
            continue;
        }
        if depths[i] > depths[i - 1] {
            count += 1;
        }
    }
    count
}

fn part_2(input: Vec<String>) -> i32 {
    let depths = string_to_int(input);
    let mut count = 0;
    // Task input is to sum neighboring 3 values and compare to next sliding window.
    // This is equivalent to comparing the last value of the current window to the
    // first value of the previous window.
    for i in 0..depths.len() {
        // Initial window of 3 cannot be compared with prev values
        if i < 3 {
            continue;
        }
        if depths[i] > depths[i - 3] {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    // Part 1
    #[test]
    fn part_1_sample_input() {
        let filename = "input/day01_sample.txt".to_string();
        let input = file_to_vec(filename).unwrap();
        let result = part_1(input);
        assert_eq!(result, 7);
    }
    #[test]
    fn part_1_full_input() {
        let filename = "input/day01.txt".to_string();
        let input = file_to_vec(filename).unwrap();
        let result = part_1(input);
        assert_eq!(result, 1766);
    }

    // Part 2
    #[test]
    fn part_2_sample_input() {
        use crate::part_2;
        let filename = "input/day01_sample.txt".to_string();
        let input = file_to_vec(filename).unwrap();
        let result = part_2(input);
        assert_eq!(result, 5);
    }
    #[test]
    fn part_2_full_input() {
        use crate::part_2;
        let filename = "input/day01.txt".to_string();
        let input = file_to_vec(filename).unwrap();
        let result = part_2(input);
        assert_eq!(result, 1797);
    }
}
