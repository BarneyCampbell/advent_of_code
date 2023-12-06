use std::{str::FromStr, collections::HashMap, sync::mpsc, thread};

struct ParseAlmanacError;
#[derive(Debug)]
struct ParseMapError;

#[derive(Clone)]
struct Almanac {
    seeds: Vec<usize>,
    maps: HashMap<String, Map>
}

impl FromStr for Almanac {
    type Err = ParseAlmanacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seeds_str, maps_str) = s.split_once("\n")
                             .unwrap();

        let seeds: Vec<usize> = seeds_str.split(": ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let mut maps: HashMap<String, Map> = HashMap::new();

        for map in maps_str.split("\n\n") {
            let res = map.parse::<Map>().unwrap();
            maps.insert(res.source.clone(), res);
        }

        Ok(Almanac { seeds, maps })
    }
}

#[derive(Clone)]
struct Map {
    source: String,
    destination: String,
    ranges: Vec<Range>,
}

#[derive(Clone)]
struct Range {
    source_start: usize,
    destination_start: usize,
    size: usize
}

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (source_dest, ranges) = s.split_once(":\n").unwrap();

        let source_destination: Vec<&str> = source_dest.split_whitespace().nth(0).unwrap().split("-to-").collect();
        let (source, destination) = (source_destination[0].to_string(), source_destination[1].to_string());

        let mut ranges_vec: Vec<Range> = Vec::new();

        for r in ranges.split("\n") {
            let set: Vec<usize> = r.split_whitespace()
                .map(|item| item.parse().unwrap() )
                .collect();

            if set.len() == 0 {
                continue;
            }

            ranges_vec.push(Range {size: set[2], source_start: set[1], destination_start: set[0]});
        }

        return Ok(Map {
            source,
            destination,
            ranges: ranges_vec
        });
    }

}

fn get_mapped(almanac: &Almanac, seeds: Vec<usize>) -> Vec<usize> {
    let mut seeds = seeds;
    let mut current_stage = String::from("seed");

    for _ in 0..almanac.maps.keys().len() {
        let mut new_stage = String::from("");

        for i in 0..seeds.len() {
            let mut current_value = seeds[i];

            if let None = almanac.maps.get(&current_stage) { break }
            for range in &almanac.maps.get(&current_stage).unwrap().ranges {
                if (range.source_start..range.source_start+range.size).contains(&&(seeds[i] as usize)) {
                    let offset = seeds[i] as usize - range.source_start;

                    current_value = range.destination_start + offset;
                    new_stage = almanac.maps.get(&current_stage).unwrap().destination.clone();
                    break;
                }
                else {
                    current_value = seeds[i];
                }
            }
            seeds[i] = current_value;
        }
        current_stage = new_stage;
    }

    seeds
}

pub fn part1(filepath: &str) -> i32 {
    if let Ok(almanac) = aoc::read_entire::<Almanac>(filepath) {
        let mapped = get_mapped(&almanac, almanac.seeds.clone());

        let mut min = usize::MAX;

        for seed in mapped {
            if seed < min {
                min = seed;
            }
        }

        return min.try_into().unwrap();
    }

    0
}

pub fn part2(filepath: &str) -> i32 {
    if let Ok(almanac) = aoc::read_entire::<Almanac>(filepath) {
        let mut seeds: Vec<Vec<usize>> = Vec::new();
        let mut pair_pos = 1;

        for pair in almanac.seeds.windows(2) {
            pair_pos += 1;
            if pair_pos % 2 != 0 { continue; }
            seeds.push((pair[0]..(pair[0] + pair[1])).map(|n| usize::from(n)).collect());
        }
        let seeds_len = seeds.len();

        let (tx, rx) = mpsc::channel();

        for seed in seeds {
            let transmitter = tx.clone();
            let almanac = almanac.clone();

            thread::spawn(move || {
                let mapped = get_mapped(&almanac, seed);

                let mut min = usize::MAX;

                for seed in mapped {
                    if seed < min {
                        min = seed;
                    }
                }

                transmitter.send(min).unwrap();
            });
        }

        let mut i = 0;
        let mut mins: Vec<usize> = Vec::new();

        for message in rx {
            println!("{}", message);
            mins.push(message);
            i += 1;
            //if i <= seeds_len { break }
        }


        println!("{:?}", mins);
        let mut min = usize::MAX;
        for m in mins {
            if m < min {
                min = m;
            }
        }
        return min.try_into().unwrap();
    }

    0
}

pub fn main() {
    let filepath = "data/day5small.txt";
    println!("Day 5 part 1: {}", part1(filepath));
    println!("Day 5 part 2: {}", part2(filepath));
}
