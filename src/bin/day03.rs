// https://adventofcode.com/2021/day/3

use aoc_rust_2021::{file_to_vec};
use std::{num::ParseIntError, collections::HashMap};
fn main() {
    let filename = "input/day03.txt";
    let input = file_to_vec(filename).unwrap();
    let part_1_result = part_1(&input).unwrap();
    println!("Part 1: {}", part_1_result);

    // let part_2_result = part_2(&input).unwrap();
    // println!("Part 2: {}", part_2_result);
}

fn part_1(input: &Vec<String>) -> Result<i32, ParseIntError> {
    let n_lines = input.len();
    let one_counts = get_counts_per_position(input);
    let majority = construct_binary_string_from_counts(one_counts, n_lines);
    let minority = invert_binary(&majority);
    let majority = i32::from_str_radix(&majority, 2)?;
    let minority = i32::from_str_radix(&minority, 2)?;
    let result = majority * minority;
    Ok(result)
}

fn get_counts_per_position(input: &Vec<String>) -> HashMap<usize, i32> {
    let mut one_counts: HashMap<usize, i32> = HashMap::new();
    for line in input.iter() {
        for (i_char, char) in line.chars().into_iter().enumerate() {
            if char == '1' {
                one_counts.entry(i_char).and_modify(|count| *count += 1).or_insert(1);
            }
        }
    }
    one_counts
}

fn construct_binary_string_from_counts(one_counts: HashMap<usize, i32>, n_lines: usize) -> String {
    let mut output = String::new();
    let cutoff: i32 = (n_lines / 2).try_into().unwrap();
    for i_pos in 0..one_counts.len() {
        if one_counts[&i_pos] >  cutoff {
            output.push_str("1")
        } else {
            output.push_str("0")
        }
    }
    output
}

fn invert_binary(input: &str) -> String {
    let output = input.to_owned();
    output.replace("1", "X").replace("0", "1").replace("X", "0")
}
#[cfg(test)]
mod tests {
    use super::*;
    // Part 1
    #[test]
    fn part_1_sample_input() {
        let filename = "input/day03_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_1(&input).unwrap();
        assert_eq!(result, 198);
    }
    #[test]
    fn part_1_full_input() {
        let filename = "input/day03.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_1(&input).unwrap();
        assert_eq!(result, 3320834);
    }
}