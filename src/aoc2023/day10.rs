use std::str::FromStr;

struct ParsePipesErr;

enum Tile {
    NorthSouth, // |
    EastWest,   // -
    NorthEast,  // L 
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F 
    Ground,     // .
    Start,      // S
}

impl Tile {
    fn with_from(&self, direction: Compass) -> Compass {
        match self {
            Self::NorthSouth    => if direction == Compass::North { Compass::South } else { Compass::North },
            Self::EastWest      => if direction == Compass::East { Compass::West } else { Compass::East },
            Self::NorthEast     => if direction == Compass::North { Compass::East } else { Compass::North },
            Self::NorthWest     => if direction == Compass::North { Compass::West } else { Compass::North },
            Self::SouthWest     => if direction == Compass::South { Compass::West } else { Compass::South },
            Self::SouthEast     => if direction == Compass::South { Compass::East } else { Compass::East },
            _                   => Compass::North
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Compass {
    North, South, East, West
}

struct Position {
    steps: u32,
    prev_direction: Compass, 
    north: i32,
    east: i32
}

impl Position {
    fn take_step(&mut self, direction: Tile) {
        let to = direction.with_from(self.prev_direction);

        match to {
            Compass::North => self.north += 1,
            Compass::South => self.north -= 1,
            Compass::East  => self.east += 1,
            Compass::West  => self.east -= 1
        }
    }
}

struct Pipes {
    start: (i32, i32),
    map: Vec<Vec<Tile>>
}


impl FromStr for Pipes {
    type Err = ParsePipesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::<Vec<Tile>>::new();
        let mut start: (i32, i32) = (-1, -1);
        let mut north = 0;
        let mut east = 0;

        for row in s.split("\n") {
            let mut map_row = Vec::<Tile>::new();

            for tile in row.chars() {
                map_row.push(match tile {
                    '|' => Tile::NorthSouth,
                    '-' => Tile::EastWest,
                    'L' => Tile::NorthEast,
                    'J' => Tile::NorthWest,
                    '7' => Tile::SouthWest,
                    'F' => Tile::SouthEast,
                    'S' =>  {
                        start = (north, east);
                        Tile::Start
                    },
                    _   => Tile::Ground
                });
                east += 1;
            }
            north += 1;
            map.push(map_row);
        }

        Ok(Self { start, map })
    }
}

fn part1(filepath: &str) -> i32 {
    if let Ok(map) = aoc::read_entire::<Pipes>(filepath) {

    }

    0
}

pub fn main() {
    let filepath = "data/day10.rs";

    println!("Dat 10 part 1: {}", part1(filepath));
    println!("Dat 10 part 2: todo");

}
