use std::collections::HashMap;

fn hash(inp: &str) -> u32
{
    let mut current = 0;
    for char in inp.chars() {
        current += u32::from(char);
        current *= 17;
        current = current % 256;
    }

    current
}

fn part1(filepath: &str) -> u32 {
    let instructions = aoc::read_entire::<String>(filepath).expect("Something went wrong reading the file");

    let mut result = 0;
    for instruction in instructions.split(",") {
        result += hash(instruction.trim());
    }

    result
}

#[derive(Clone)]
struct Lens {
    name: String,
    focal_length: u32
}

fn part2(filepath: &str) -> u32 {
    let instructions = aoc::read_entire::<String>(filepath).expect("Something went wrong reading the file");

    let mut boxes = HashMap::<u32, Vec<Lens>>::new();

    for instruction in instructions.split(",") {
        if let Some((split_eq, num)) = instruction.trim().split_once("=") {
            let box_index = hash(split_eq);

            if let Some(vec) = boxes.get_mut(&box_index) {
                if vec.into_iter().filter(|item| item.name == split_eq.to_string()).count() == 0 {
                    vec.push(Lens {name: split_eq.into(), focal_length: num.parse().unwrap()});
                }
                else {
                    vec.iter_mut()
                    .for_each(|item| if item.name == split_eq.to_string() {
                            item.focal_length = num.parse().unwrap()
                    });
                }
            }
            else {
                boxes.insert(box_index, vec![Lens {name: split_eq.into(), focal_length: num.parse().unwrap()}]);
            }
        }
        else if let Some((split_minus, _)) = instruction.trim().split_once("-") {
            let box_index = hash(split_minus);

            if boxes.contains_key(&box_index) {
                boxes.insert(
                    box_index,
                    boxes.get(&box_index)
                        .unwrap()
                        .into_iter()
                        .filter(|item| item.name != split_minus.to_string())
                        .cloned()
                        .collect()
                );
            }
        }
    }

    let mut score = 0;
    for (index, items) in boxes {
        let mut slot = 1;
        for item in items {
            score += (index+1) * (slot) * item.focal_length;
            slot += 1;
        }
    }

    score
}

pub fn main() {
    let filepath = "data/day15.txt";

    println!("Day 15 part 1: {}", part1(filepath));
    println!("Day 15 part 2: {}", part2(filepath));
}
