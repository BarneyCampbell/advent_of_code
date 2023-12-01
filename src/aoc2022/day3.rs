
pub fn part1(filepath: &str) -> i32 {
    if let Ok(lines) = aoc::read_lines_optional::<String>(filepath) {
        lines.iter().map(get_score).sum()
    }
    else {
        0
    }
}

fn get_score(rucksack: &Option<String>) -> i32 {
    match rucksack {
        Some(items) => find_match(items.split_at(items.len() / 2)),
        None        => 0
    }
}

fn find_match(halves: (&str, &str)) -> i32 {
    for thing1 in halves.0.chars() {
        for thing2 in halves.1.chars() {
            if thing1 == thing2 {
                if thing1.is_ascii_uppercase() {
                    return thing1 as i32 - 38;
                }
                else {
                    return thing1 as i32 - 96;
                }
            }
        }
    };

    0
}

pub fn part2(filepath: &str) -> i32 {
    let mut priority = 0;

    if let Ok(lines) = aoc::read_lines::<String>(filepath) {
        let mut counter = 2;

        while counter < lines.len() {
            priority += triple_match(
                (&lines[counter].clone(),
                 &lines[counter-1].clone(),
                 &lines[counter-2].clone())
            );
            counter += 3;
        };

        priority
    }
    else {
        0
    }
}

fn triple_match(rucksacks: (&str, &str, &str)) -> i32 {
    for thing1 in rucksacks.0.chars() {
        for thing2 in rucksacks.1.chars() {
            for thing3 in rucksacks.2.chars() {
                if thing1 == thing3 && thing1 == thing2 && thing2 == thing3 {
                    if thing1.is_ascii_uppercase() {
                        return thing1 as i32 - 38;
                    }
                    else {
                        return thing1 as i32 - 96;
                    }
                }
            };
        };
    };

    0
}

pub fn main() {
    println!("Day 3 part 1: {}", part1("data/day3.txt"));
    println!("Day 3 part 2: {}", part2("data/day3.txt"));
}
