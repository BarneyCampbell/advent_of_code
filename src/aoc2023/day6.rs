use std::iter::zip;


pub fn part1(filepath: &str) -> i32 {
    if let Ok(input) = aoc::read_entire::<String>(filepath) {
        let mut ans = 1;
        let Some((time, distance)) = input.split_once("\n") else { unreachable!() };

        let Some((_, times)) = time.split_once(":") else { unreachable!() };
        let Some((_, distances)) = distance.split_once(":") else { unreachable!() };

        for (d, t) in zip(distances.split_whitespace().map(|n| n.parse::<i32>().unwrap()),
            times.split_whitespace().map(|n| n.parse::<i32>().unwrap()))
            {
                let mut winning_times = 0;

                for mil_per_mil in 1..t {
                    let remaining = t - mil_per_mil;
                    let travelled = mil_per_mil * remaining;

                    if travelled > d {
                        winning_times += 1;
                    }
                }

                if winning_times != 0 {
                    ans *= winning_times;
                }
            }

        return ans;
    }

    0
}

pub fn part2(filepath: &str) -> i64 {
    if let Ok(input) = aoc::read_entire::<String>(filepath) {
        let Some((time, distance)) = input.split_once("\n") else { unreachable!() };

        let Some((_, times)) = time.split_once(":") else { unreachable!() };
        let Some((_, distances)) = distance.split_once(":") else { unreachable!() };

        let full_time: i64 = times.split_whitespace()
            .collect::<Vec<&str>>()
            .join("")
            .parse()
            .unwrap();

        let full_distance: i64 = distances.split_whitespace()
            .collect::<Vec<&str>>()
            .join("")
            .parse()
            .unwrap();

        let mut winning_times = 0;

        for mil_per_mil in 1..full_time {
            let remaining = full_time - mil_per_mil;
            let travelled = mil_per_mil * remaining;

            if travelled > full_distance {
                winning_times += 1;
            }
        }

        return winning_times;
    }

    0
}

pub fn main() {
    let filepath = "data/day6.txt";

    println!("Day 6 part 1: {}", part1(filepath));
    println!("Day 6 part 2: {}", part2(filepath));
}
