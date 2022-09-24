// https://adventofcode.com/2021/day/3

use aoc_rust_2021::file_to_vec;
use std::{collections::HashMap, num::ParseIntError};
fn main() {
    let filename = "input/day03.txt";
    let input = file_to_vec(filename).unwrap();
    let part_1_result = part_1(&input).unwrap();
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input).unwrap();
    println!("Part 2: {}", part_2_result);
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
                one_counts
                    .entry(i_char)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }
    one_counts
}

fn construct_binary_string_from_counts(one_counts: HashMap<usize, i32>, n_lines: usize) -> String {
    let mut output = String::new();
    let cutoff: i32 = (n_lines / 2).try_into().unwrap();
    for i_pos in 0..one_counts.len() {
        if one_counts[&i_pos] > cutoff {
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

// PART 2
fn part_2(input: &Vec<String>) -> Result<i32, ParseIntError> {
    let oxygen_rating = get_oxygen_rating(input)?;
    let co2_rating = get_co2_rating(input)?;
    let result = oxygen_rating * co2_rating;
    Ok(result)
}
#[derive(Debug, PartialEq, Eq, Hash)]
enum BitCount {
    Ones,
    Zeroes,
    Equal,
}

fn get_oxygen_rating(input: &Vec<String>) -> Result<i32, ParseIntError> {
    let mut remaining_inputs = input.clone();
    let mut current_bit: usize = 0;

    let bit_criterion = HashMap::from([
        (BitCount::Ones, '1'),
        (BitCount::Zeroes, '0'),
        (BitCount::Equal, '1'),
    ]);
    loop {
        let most_common = get_common_bit(&remaining_inputs, &current_bit);
        let selection_bit = bit_criterion.get(&most_common).unwrap();
        remaining_inputs = select_inputs(remaining_inputs, selection_bit, &current_bit);

        if remaining_inputs.len() == 1 {
            break;
        } else {
            current_bit += 1;
        }
    }
    let rating_binary = &remaining_inputs[0];
    let rating_decimal = i32::from_str_radix(&rating_binary, 2)?;
    Ok(rating_decimal)
}

fn get_common_bit(input: &Vec<String>, bit_index: &usize) -> BitCount {
    let mut one_counts: usize = 0;
    let cutoff: f32 = (input.len() as f32) / 2.0;

    for line in input.iter() {
        //println!("Line: {}, index: {}", line, bit_index);
        if line.chars().nth(*bit_index).unwrap() == '1' {
            one_counts += 1
        }
    }
    if (one_counts as f32) < cutoff {
        BitCount::Zeroes
    } else if (one_counts as f32) > cutoff {
        BitCount::Ones
    } else {
        BitCount::Equal
    }
}

fn select_inputs(input: Vec<String>, selection_bit: &char, bit_index: &usize) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for line in input.iter() {
        if line.chars().nth(*bit_index).unwrap() == selection_bit.to_owned() {
            output.push(line.to_owned())
        }
    }
    output
}

fn get_co2_rating(input: &Vec<String>) -> Result<i32, ParseIntError> {
    let mut remaining_inputs = input.clone();
    let mut current_bit: usize = 0;

    let bit_criterion = HashMap::from([
        (BitCount::Ones, '0'),
        (BitCount::Zeroes, '1'),
        (BitCount::Equal, '0'),
    ]);
    loop {
        let most_common = get_common_bit(&remaining_inputs, &current_bit);
        let selection_bit = bit_criterion.get(&most_common).unwrap();
        remaining_inputs = select_inputs(remaining_inputs, selection_bit, &current_bit);

        if remaining_inputs.len() == 1 {
            break;
        } else {
            current_bit += 1;
        }
    }
    let rating_binary = &remaining_inputs[0];
    let rating_decimal = i32::from_str_radix(&rating_binary, 2)?;
    Ok(rating_decimal)
}
#[cfg(test)]
mod tests {
    use std::vec;

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

    // Part 2
    #[test]
    fn part_2_sample_input() {
        let filename = "input/day03_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_2(&input).unwrap();
        assert_eq!(result, 230);
    }

    #[test]
    fn part_2_full_input() {
        let filename = "input/day03.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_2(&input).unwrap();
        assert_eq!(result, 4481199);
    }

    #[test]
    fn part_2_sample_input_oxygen_rating() {
        let filename = "input/day03_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = get_oxygen_rating(&input).unwrap();
        assert_eq!(result, 23);
    }

    #[test]
    fn part_2_sample_input_co2_rating() {
        let filename = "input/day03_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = get_co2_rating(&input).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_common_bit() {
        let input_vec = vec!["100".to_owned(), "110".to_owned()];
        let actual_pos_0 = get_common_bit(&input_vec, &0);
        let expected_pos_0 = BitCount::Ones;
        assert_eq!(actual_pos_0, expected_pos_0);

        let actual_pos_1 = get_common_bit(&input_vec, &1);
        let expected_pos_1 = BitCount::Equal;
        assert_eq!(actual_pos_1, expected_pos_1);

        let actual_pos_2 = get_common_bit(&input_vec, &2);
        let expected_pos_2 = BitCount::Zeroes;
        assert_eq!(actual_pos_2, expected_pos_2);

        let input_vec = vec!["100".to_owned(), "110".to_owned(), "011".to_owned()];
        let actual_pos_0 = get_common_bit(&input_vec, &0);
        let expected_pos_0 = BitCount::Ones;
        assert_eq!(actual_pos_0, expected_pos_0);
    }
    #[test]
    fn test_select_inputs() {
        let input_vec = vec!["100".to_owned(), "010".to_owned()];
        let selection_bit = '1';

        let expected = vec!["100".to_owned()];
        let actual = select_inputs(input_vec, &selection_bit, &0);
        assert_eq!(actual, expected)
    }
}
