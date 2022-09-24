// https://adventofcode.com/2021/day/2
use aoc_rust_2021::file_to_vec;
use std::num::ParseIntError;
fn main() {
    let filename = "input/day02.txt";
    let input = file_to_vec(filename).unwrap();
    let part_1_result = part_1(&input).unwrap();
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&input).unwrap();
    println!("Part 2: {}", part_2_result);
}

struct Position {
    horizontal: i32,
    depth: i32,
}

enum Action {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn parse_commands(input: &Vec<String>) -> Result<Vec<Action>, ParseIntError> {
    let mut actions = Vec::new();
    for i in 0..input.len() {
        let split: Vec<&str> = input[i].split_whitespace().collect();
        let action_string = split[0];
        let value: i32 = split[1].parse()?;

        let action = match action_string {
            "up" => Action::Up(value),
            "down" => Action::Down(value),
            "forward" => Action::Forward(value),
            _ => todo!(),
        };
        actions.push(action);
    }
    Ok(actions)
}
fn part_1(input: &Vec<String>) -> Result<i32, ParseIntError> {
    let actions = parse_commands(input)?;
    let mut pos = Position {
        horizontal: 0,
        depth: 0,
    };
    for action in actions {
        match action {
            Action::Up(n) => pos.depth -= n,
            Action::Down(n) => pos.depth += n,
            Action::Forward(n) => pos.horizontal += n,
        }
    }
    Ok(pos.horizontal * pos.depth)
}

// PART 2
struct PositionWithAim {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn part_2(input: &Vec<String>) -> Result<i32, ParseIntError> {
    let actions = parse_commands(input)?;
    let mut pos = PositionWithAim {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    for action in actions {
        match action {
            Action::Up(n) => pos.aim -= n,
            Action::Down(n) => pos.aim += n,
            Action::Forward(n) => {
                pos.horizontal += n;
                pos.depth += pos.aim * n
            }
        }
    }
    Ok(pos.horizontal * pos.depth)
}
#[cfg(test)]
mod tests {
    use super::*;
    // Part 1
    #[test]
    fn part_1_sample_input() {
        let filename = "input/day02_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_1(&input).unwrap();
        assert_eq!(result, 150);
    }
    #[test]
    fn part_1_full_input() {
        let filename = "input/day02.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_1(&input).unwrap();
        assert_eq!(result, 1524750);
    }
    // Part 2
    #[test]
    fn part_2_sample_input() {
        let filename = "input/day02_sample.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_2(&input).unwrap();
        assert_eq!(result, 900);
    }
    #[test]
    fn part_2_full_input() {
        let filename = "input/day02.txt";
        let input = file_to_vec(filename).unwrap();
        let result = part_2(&input).unwrap();
        assert_eq!(result, 1592426537);
    }
}
