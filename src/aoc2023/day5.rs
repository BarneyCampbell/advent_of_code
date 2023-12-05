use core::ops::Range;
use std::{str::FromStr, collections::HashMap};

struct ParseAlmanacError;
#[derive(Debug)]
struct ParseMapError;

struct Almanac {
    seeds: Vec<u32>,
    maps: HashMap<String, Map>
}

impl FromStr for Almanac {
    type Err = ParseAlmanacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seeds_str, maps_str) = s.split_once("\n")
                             .unwrap();

        let seeds: Vec<u32> = seeds_str.split(": ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let mut maps: HashMap<String, Map> = HashMap::new();

        for map in maps_str.split("\n\n") {
            let res = map.parse::<Map>().unwrap();
            maps.insert(res.source.clone(), res);
        }

        Ok(Almanac { seeds, maps })
    }
}

struct Map {
    source: String,
    destination: String,
    source_range: Vec<Range<usize>>,
    destination_range: Vec<Range<usize>>
}

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (source_dest, ranges) = s.split_once(":\n").unwrap();

        let source_destination: Vec<&str> = source_dest.split_whitespace().nth(0).unwrap().split("-to-").collect();
        let (source, destination) = (source_destination[0].to_string(), source_destination[1].to_string());

        let mut source_range: Vec<Range<usize>> = Vec::new();
        let mut destination_range: Vec<Range<usize>> = Vec::new();

        for r in ranges.split("\n") {
            let set: Vec<usize> = r.split_whitespace()
                .map(|item| item.parse().unwrap() )
                .collect();

            if set.len() == 0 {
                continue;
            }

            source_range.push((set[0])..(set[0]+set[2]));
            destination_range.push((set[1])..(set[1]+set[2]));
        }

        Ok(Map {
            source,
            destination,
            source_range,
            destination_range
        })
    }

}

pub fn part1(filepath: &str) -> i32 {
    if let Ok(almanac) = aoc::read_entire::<Almanac>(filepath) {
        for map in almanac.maps.keys() {
            println!("{}{:?} -> {}{:?}", almanac.maps.get(map).unwrap().source, almanac.maps.get(map).unwrap().source_range, almanac.maps.get(map).unwrap().destination, almanac.maps.get(map).unwrap().destination_range);
        }
    }

    0
}

pub fn main() {
    let filepath = "data/day5small.txt";
    println!("Day 5 part 1: {}", part1(filepath));
    println!("Day 5 part 2: {}", "todo");
}
