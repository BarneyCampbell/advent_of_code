use std::collections::HashMap;

pub fn part1(filepath: &str) -> i32 {
    let mut total = 0;

    if let Ok(lines) = aoc::read_lines::<String>(filepath) {
        let mut first: char = '0';
        let mut last: char = '0';

        for line in lines {
            for c in line.chars() {
                if c.is_numeric() {
                    first = c;
                    break;
                }
            }
            for c in line.chars().rev() {
                if c.is_numeric() {
                    last = c;
                    break;
                }
            }

            total += format!("{}{}", first, last).parse::<i32>().unwrap();
        }
    }

    total
}

fn get_digit(line: String, reverse: bool) -> i64 {
    let numbers: HashMap<&str, i32> = HashMap::<&str, i32>::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    let mut digit: i64 = 0;
    let mut left_pos = 0;
    let mut right_pos = 0;

    while left_pos <= line.len() {
        if left_pos < line.len() && char::from(line.as_bytes()[left_pos]).is_numeric() {
            digit = i64::from(char::from(line.as_bytes()[left_pos]).to_digit(10).unwrap());
        }

        while right_pos <= line.len() {
            let rev: String;

            let word = if reverse {
                rev = line.chars().rev().collect();
                &rev[(rev.len() - right_pos)..(rev.len() - left_pos)]
            }
            else {
                &line[left_pos..right_pos]
            };

            if let Some(value) = numbers.get(word) {
                digit = i64::from(*value);
                break;
            }

            right_pos += 1;
        }
        left_pos += 1;
        right_pos = left_pos;

        if digit != 0 {
            break;
        }
    }

    digit
}

pub fn part2(filepath: &str) -> i32 {
    let mut total = 0;

    if let Ok(lines) = aoc::read_lines::<String>(filepath) {
        for line in lines {
            let first = get_digit(line.clone(), false);
            let last = get_digit(line.chars().rev().collect(), true);

            total += format!("{}{}", first, last).parse::<i32>().unwrap();
        }
    }

    total
}

pub fn main() {
    let filepath = "data/day1.txt";
    println!("Day 1 part 1: {}", part1(filepath));
    println!("Day 1 part 2: {}", part2(filepath));
}
