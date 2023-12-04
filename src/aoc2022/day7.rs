use std::str::FromStr;
use std::collections::HashMap;
use std::thread::current;

struct ParseCommandErr;

#[derive(Clone)]
struct File {
    name: String,
    size: usize
}

#[derive(Clone)]
struct Directory<'a> {
    name: String,
    files: Vec<File>,
    subdirectories: HashMap<String, Directory<'a>>,
    dotdot: Option<&'a Directory<'a>>,
    size: usize
}

enum CommandType {
    CD,
    LS,
    FILE,
    DIR
}

struct Command {
    kind: CommandType,
    keyword: Option<String>,
    data: Option<String>
}

impl FromStr for Command {
    type Err = ParseCommandErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().nth(0) == Some('$') {
            let info = s.split(' ').collect::<Vec<&str>>();

            match info[1] {
                "ls" => return Ok(Command { kind: CommandType::LS, keyword: None, data: None}),
                "cd" => return Ok(Command { kind: CommandType::CD, keyword: Some(info[2].to_string()), data: None}),
                _    => return Err(ParseCommandErr),
            }
        }
        else {
            let info = s.split(' ').collect::<Vec<&str>>();

            match info[0] {
                "dir" => return Ok(Command { kind: CommandType::DIR, keyword: Some(info[1].to_string()), data: None }),
                _     => return Ok(Command { kind: CommandType::FILE, keyword: Some(info[1].to_string()), data: Some(info[0].to_string()) })
            }
        }
    }
}

fn part1(filepath: &str) -> i32 {
    let mut current_directory = Directory {
        name: String::from("/"),
        files: vec![],
        subdirectories: HashMap::new(),
        dotdot: None,
        size: 0,
    };

    if let Ok(lines) = aoc::read_lines::<Command>(filepath) {
        for command in lines.iter() {
            if let Some(c) = command {
                match c.kind {
                    CommandType::CD => {
                        if let Some(dir) = &c.keyword {
                            if current_directory.subdirectories.contains_key(dir) {
                                current_directory = current_directory.subdirectories.get(dir).unwrap().clone();
                            }
                            else {
                                let new_dir = Directory {
                                    name: String::from(dir),
                                    files: vec![],
                                    subdirectories: HashMap::new(),
                                    dotdot: Some(&current_directory),
                                    size: 0
                                };

                                current_directory.subdirectories.insert(String::from(dir), new_dir);
                            }
                        }
                    },
                    _ => println!("Not implemented")
                }
                // match c.kind {
                //     CommandType::LS => println!("ls"),
                //     CommandType::CD => println!("cd {:?}", c.keyword.clone().unwrap()),
                //     CommandType::FILE => println!("file {} ~{}", c.keyword.clone().unwrap(), c.data.clone().unwrap()),
                //     CommandType::DIR => println!("dir {}", c.keyword.clone().unwrap())
                // }
            }
        }
    }
    0
}

pub fn main() {
    println!("Day 7 part 1: {}", part1("data/day7small.txt"));
    println!("Day 7 part 2: {}", "todo");
}
