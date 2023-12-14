use std::str::FromStr;
use aoc::data_structures::Vec2D;

struct ParsePipesErr;

#[derive(Clone, Copy, Debug)]
enum Tile {
    NorthSouth, // |
    EastWest,   // -
    NorthEast,  // L 
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F 
    Ground,     // .
}

impl Tile {
    fn with_from(&self, direction: Compass) -> Option<Compass> {
        match self {
            Self::NorthSouth    => match direction {
                Compass::South => Some(Compass::South),
                Compass::North => Some(Compass::North),
                _              => None
            },
            Self::EastWest      => match direction {
                Compass::East => Some(Compass::East),
                Compass::West => Some(Compass::West),
                _             => None
            },
            Self::NorthEast     => match direction {
                Compass::West  => Some(Compass::North),
                Compass::South => Some(Compass::East),
                _              => None
            },
            Self::NorthWest     => match direction {
                Compass::East  => Some(Compass::North),
                Compass::South => Some(Compass::West),
                _              => None
            },
            Tile::SouthWest     => match direction {
                Compass::East  => Some(Compass::South),
                Compass::North => Some(Compass::West),
                _              => None
            },
            Self::SouthEast     => match direction {
                Compass::West  => Some(Compass::South),
                Compass::North => Some(Compass::East),
                _              => None
            },
            _                   => None
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Compass {
    North, South, East, West
}

#[derive(Clone, Copy)]
struct Position {
    steps: u32,
    prev_direction: Compass, 
    north: usize,
    east: usize
}

impl Position {
    fn take_step(&mut self, direction: Tile) {
        if let Some(to) = direction.with_from(self.prev_direction) {
            self.prev_direction = to;
            self.steps += 1;

            match to {
                Compass::North => self.north -= 1,
                Compass::South => self.north += 1,
                Compass::East  => self.east += 1,
                Compass::West  => self.east -= 1
            }
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.north == other.north && self.east == other.east
    }
}

struct Pipes {
    start: Position,
    map: Vec<Vec<Tile>>
}


impl FromStr for Pipes {
    type Err = ParsePipesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::<Vec<Tile>>::new();
        let mut start = Position {
            steps: 0,
            prev_direction: Compass::North,
            east: 0,
            north: 0
        };
        let mut north: usize = 0;
        let mut east: usize;

        for row in s.split("\n") {
            east = 0;
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
                        start = Position {
                            steps: 0,
                            prev_direction: Compass::North,
                            east, north
                        };
                        Tile::SouthEast // shhhh
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

fn part1(filepath: &str) -> u32 {
    if let Ok(map) = aoc::read_entire::<Pipes>(filepath) {
        let mut position = map.start.clone();

        loop {
            position.take_step(map.map[position.north][position.east]);
            if position == map.start { break }
        }

        return position.steps / 2;
    }

    0
}

fn part2(filepath: &str) -> usize {
    let Ok(mut map) = Vec2D::from(aoc::read_char_vec(filepath).unwrap()) else { panic!() };
    for row in 0..(map.height-0) {
        for col in 0..(map.width-0) {
            if map.get(row, col).unwrap() != &'I' && map.get(row, col).unwrap() != &'O' && map.get(row, col).unwrap() != &'.' {
                continue;
            }

            let item_row = map.row(row);
            let item_col = map.col(col);

            if is_contained(item_row, col) && is_contained(item_col, row) {
                let _ = map.set('I', row, col);
            }
            else {
                let _ = map.set('O', row, col);
            }
        }
    }

    for row in 0..(map.height-0) {
        for col in 0..(map.width-0) {
            if map.get(row, col).unwrap() != &'I' && map.get(row, col).unwrap() != &'O' && map.get(row, col).unwrap() != &'.' {
                continue;
            }

            let item_row = map.row(row);
            let item_col = map.col(col);

            if is_contained(item_row, col) && is_contained(item_col, row) {
                let _ = map.set('I', row, col);
            }
            else {
                let _ = map.set('O', row, col);
            }
        }
    }

    for row in 0..(map.height) {
        for col in 0..(map.width) {
            if map.get(row, col).unwrap() != &'O' {
                continue;
            }

            if row > 0 && col > 0 {
                if let Some('I') = map.get(row-1, col-1) {
                    let _ = map.set('O', row-1, col-1);
                }
            }
            if row > 0 {

                if let Some('I') = map.get(row-1, col+1) {
                    let _ = map.set('O', row-1, col+1);
                }
            }
            if col > 0 {
                if let Some('I') = map.get(row+1, col-1) {
                    let _ = map.set('O', row+1, col-1);
                }
            }
            if let Some('I') = map.get(row+1, col+1) {
                let _ = map.set('O', row+1, col+1);
            }
        }
    }
    map.into_iter().filter(|c| c == &'I').count()
}

fn is_contained(segment: Vec<char>, i: usize) -> bool {
    let mut left = i;
    let mut right = i;
    let (mut found_l, mut found_r) = (false, false);

    while !found_l && left > 0 {
        left -= 1;

        if !found_l && segment[left] != '.' && segment[left] != 'I' {
            found_l = true;

            if segment[left] == 'O' {
                return false;
            }
        }
    }

    while !found_r && right < segment.len()-1 {
        right += 1;

        if !found_r && segment[right] != '.' && segment[right] != 'I' {
            found_r = true;

            if segment[right] == 'O' {
                return false;
            }
        }

    }

    found_l && found_r
    
}

pub fn main() {
    let filepath = "data/day10.txt";

    println!("Dat 10 part 1: {}", part1(filepath));
    println!("Dat 10 part 2: {}", part2(filepath));

}
