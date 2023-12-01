use std::{str::FromStr};

struct ParsePairError;

#[derive(Debug)]
struct Pair {
    elf_one: ElfRange,
    elf_two: ElfRange,
    overlap: bool
}

#[derive(Debug, Clone, Copy)]
struct ElfRange {
    start: i32,
    end: i32
}

impl ElfRange {
    pub fn contains(&self, other: ElfRange) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn touches(&self, other: ElfRange) -> bool {
         self.start <= other.start && self.end >= other.start ||
         self.start >= other.start && self.end <= other.start ||
         self.start <= other.end && self.end >= other.end
    }
}

impl Pair {
    pub fn overlap(&self) -> bool {
        self.elf_one.touches(self.elf_two) || self.elf_two.touches(self.elf_one)
    }
}

impl FromStr for Pair {
    type Err = ParsePairError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elves = s.split(',');
        let mut elf_ranges: Vec<ElfRange> = vec![];
        let overlap: bool;

        for elf in elves {
            let start = match elf.split('-').next() {
                Some(x) => x.parse::<i32>().ok().unwrap(),
                None    => return Err(ParsePairError)
            };
            let end = match elf.split('-').last() {
                Some(x) => x.parse::<i32>().ok().unwrap(),
                None    => return Err(ParsePairError)
            };
            elf_ranges.push(ElfRange {start, end});
        }
        overlap = elf_ranges[0].contains(elf_ranges[1]) || elf_ranges[1].contains(elf_ranges[0]);

        Ok(Pair { elf_one: elf_ranges[0].clone(), elf_two: elf_ranges[1].clone(), overlap })
    }
}

pub fn part1(filepath: &str) -> i32 {
    if let Ok(pairs) = aoc::read_lines_optional::<Pair>(filepath) {
        pairs.iter().fold(0, |acc, pair| {
            match pair {
                Some(p) => acc + if p.overlap { 1 } else { 0 },
                None    => acc + 0
            }
        })
    }
    else { 0 }
}

pub fn part2(filepath: &str) -> i32 {
    if let Ok(pairs) = aoc::read_lines_optional::<Pair>(filepath) {
        pairs.iter().fold(0, |acc, pair| {
            match pair {
                Some(p) => acc + if p.overlap() { 1 } else { 0 },
                None    => acc + 0
            }
        })
    }
    else { 0 }
}

pub fn main() {
    println!("Day 4 part 1: {}", part1("data/day4.txt"));
    println!("Day 4 part 2: {}", part2("data/day4.txt"));
}
