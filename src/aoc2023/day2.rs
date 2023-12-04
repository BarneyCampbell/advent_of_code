use std::{str::FromStr, collections::HashMap};

struct Game {
    id: i32,
    sets: Vec<String>
}
struct ParseGameErr;

impl FromStr for Game {
    type Err = ParseGameErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let game: Vec<&str> = s.trim().split(": ").collect();

        let id_str: Vec<&str> = game[0].split_whitespace().collect();
        let id: i32 = id_str[id_str.len() - 1].parse().unwrap();

        let sets = game[1].split(";").map(|s| String::from(s)).collect();

        Ok(Self { id, sets })
    }
}

impl Default for Game {
    fn default() -> Self {
        Self { id: -1, sets: Vec::new() }
    }
}

fn check_valid(r: i32, g: i32, b: i32, game: &Game) -> bool {
    let counts = get_cube_counts(game);

    counts["red"] <= r && counts["green"] <= g && counts["blue"] <= b
}

pub fn part1(filepath:  &str) -> i32 {
    let mut count = 0;

    if let Ok(lines) = aoc::read_lines::<Game>(filepath) {
        for game in lines {
            if check_valid(12, 13, 14, &game) {
                count += game.id;
            }
        }
    }

    count
}

fn get_cube_counts(game: &Game) -> HashMap<&str, i32> {
    let mut counts = HashMap::from([
        ("red", 0),
        ("blue", 0),
        ("green", 0)
    ]);

    for set in &game.sets {
        let cubes = set.split("; ");

        for colour in cubes {
            let sep = colour.split(", ");

            for entry in sep {
                let vals: Vec<&str> = entry.split_whitespace().collect();

                if let Ok(value) = vals[0].parse::<i32>() {
                    if value > counts[vals[1]] {
                        counts.insert(vals[1], value);
                    }
                }
            }
        }
    }

    counts
}

pub fn part2(filepath:  &str) -> i32 {
    let mut res: i32 = 0;
    if let Ok(lines) = aoc::read_lines::<Game>(filepath) {
        for game in lines {
            let counts = get_cube_counts(&game);

            res += counts.values().product::<i32>();
        }
    }

    res
}

pub fn main() {
    let filepath = "data/day2.txt";

    println!("Day 2 part 1: {}", part1(filepath));
    println!("Day 2 part 1: {}", part2(filepath));
}
