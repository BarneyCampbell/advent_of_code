use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
struct ParseNodeErr;
struct ParseTreeErr;

#[derive(Clone)]
struct Node {
    left: String,
    right: String
}

impl FromStr for Node {
    type Err = ParseNodeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = strip_brackets(s).split_once(", ") {
            return Ok(Node { left: left.to_string(), right: right.to_string() });
        }

        Err(ParseNodeErr)
    }
}

fn strip_brackets(str: &str) -> &str {
    let mut chars = str.chars();

    chars.next();
    chars.next_back();

    chars.as_str()
}

struct Tree {
    route: String,
    branches: HashMap<String, Node>
}

impl FromStr for Tree {
    type Err = ParseTreeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((route, tree)) = s.split_once("\n\n") {
            let mut branches: HashMap<String, Node> = HashMap::new();

            for branch in tree.lines() {
                let (name, node) = branch.split_once(" = ").unwrap();
                
                branches.insert(name.to_string(), node.parse().unwrap());
            }
            return Ok(Tree { route: route.to_string(), branches });
        }

        Err(ParseTreeErr)
    }
}


pub fn part1(filepath: &str) -> i32 {
    if let Ok(tree) = aoc::read_entire::<Tree>(filepath) {
        let mut steps = 0;
        let mut finished = false;
        let mut node = tree.branches.get("AAA").unwrap();

        while !finished {
            for instruction in tree.route.chars() {
                steps += 1;
                let next_node = match instruction {
                    'R' => &node.right,
                    'L' => &node.left,
                    _ => unreachable!()
                };

                if next_node == "ZZZ" {
                    finished = true;
                    break;
                }

                node = tree.branches.get(&next_node.to_string()).unwrap();
            }
        }

        return steps;
    }

    0
}

pub fn part2(filepath: &str) -> u64 {
    if let Ok(tree) = aoc::read_entire::<Tree>(filepath) {
        let mut steps = 0;
        let mut finished = false;
        let mut nodes: Vec<Node> = tree.branches.keys()
            .filter(|key| key.as_bytes()[2] == b'A')
            .map(|key| tree.branches.get(key).unwrap().clone()).collect();
        let mut step_arr = vec![0; nodes.len()];

        while !finished {
            for instruction in tree.route.chars() {
                steps += 1;

                let next_nodes = nodes.iter().map(|node| {
                    match instruction {
                        'R' => &node.right,
                        'L' => &node.left,
                        _ => unreachable!()
                    }
                });

                let _ = next_nodes.enumerate().map(|(i, node)| {
                    if node.as_bytes()[2] == b'Z' {
                        step_arr[i] = steps;
                    }
                }).collect::<Vec<()>>();

                if step_arr.iter().filter(|n| **n == 0).count() == 0 {
                    finished = true;
                    break;
                }

                nodes = nodes.iter().map(|node| {
                    match instruction {
                        'R' => tree.branches.get(&node.right).unwrap().clone(),
                        'L' => tree.branches.get(&node.left).unwrap().clone(),
                        _ => unreachable!()
                    }
                }).collect();
            }
        }

       return aoc::maths::lcm(step_arr);
    }

    0
}

pub fn main() {
    let filepath = "data/day8.txt";

    println!("Day 8 part 1: {}", part1(filepath));
    println!("Day 8 part 2: {}", part2(filepath));
}
