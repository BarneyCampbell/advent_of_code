use std::{str::FromStr , collections::HashSet, hash::Hash};

struct ParseBufferError;

struct Buffer {
    contents: String
}

impl Buffer {
    pub fn find_marker(&self, marker_size: usize) -> i32 {
        for (window, i) in self.contents.as_bytes().windows(marker_size).zip(marker_size..self.contents.len()) {
            if unique_elements(window) {
                return i.try_into().unwrap();
            }
        }
        -1
    }
}

fn unique_elements<T>(iter: T) -> bool
where T: IntoIterator,
      T::Item: Eq + Hash {
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

impl FromStr for Buffer {
    type Err = ParseBufferError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Buffer { contents: s.to_string() })
    }
}

pub fn part1(filepath: &str) -> i32 {
    if let Ok(buffer) = aoc::read_entire::<Buffer>(filepath) {
        buffer.find_marker(4)
    }
    else { 0 }
}

pub fn part2(filepath: &str) -> i32 {
    if let Ok(buffer) = aoc::read_entire::<Buffer>(filepath) {
        buffer.find_marker(14)
    }
    else { 0 }
}

pub fn main() {
    println!("Day 6 part 1: {}", part1("data/day6.txt"));
    println!("Day 6 part 2: {}", part2("data/day6.txt"));
}
