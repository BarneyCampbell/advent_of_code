use aoc::data_structures::Vec2D;
use itertools::Itertools;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Galaxy (i32, i32, usize);

impl Galaxy {
    fn distance(&self, other: &Self) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }

    fn distance_walk(&self, other: &Self, space: &Vec2D<char>) -> i64 {
        let (mut dx, mut dy) = (0, 0);
        let (mut x_pos, mut y_pos) = (self.0, self.1);
        let (x_dir, y_dir) = ((other.0 - self.0).signum(), (other.1 - self.1).signum());

        while x_pos != other.0 {
            x_pos += x_dir;
            dx += if *space.get(x_pos.try_into().unwrap(), y_pos.try_into().unwrap()).unwrap() == '!' {
                1000000
            }
            else {
                1
            }
        }

        while y_pos != other.1 {
            y_pos += y_dir;
            dy += if *space.get(x_pos.try_into().unwrap(), y_pos.try_into().unwrap()).unwrap() == '!' {
                1000000
            }
            else {
                1
            }
        }

        dx + dy
    }
}

fn part1(filepath: &str) -> i32 {
    if let Ok(space) = aoc::read_char_vec(filepath) {
        let mut expanded_space = Vec2D::from(space).unwrap();

        let mut y = 0;
        for _ in 0..expanded_space.height {
            if expanded_space.row(y).iter().filter(|pos| **pos != '.').count() == 0 {
                expanded_space.repeat_row('.', y);
                y += 1;
            }
            y += 1;
        }

        let mut x = 0;
        for _ in 0..expanded_space.width {
            if expanded_space.col(x).iter().filter(|pos| **pos != '.').count() == 0 {
                expanded_space.repeat_col('.', x);
                x += 1;
            }
            x += 1;
        }

        let mut galaxies = Vec::<Galaxy>::new();
        let mut id = 0;
        for y in 0..expanded_space.height {
            for x in 0..expanded_space.width {
                match expanded_space.get(y, x) {
                    Some('#') => {id+= 1; galaxies.push(Galaxy(y.try_into().unwrap(), x.try_into().unwrap(), id))},
                    _         => () 
                }
            }
        }

        let res: i32 = galaxies.iter()
            .combinations(2)
            .map(|pair| {pair[0].distance(pair[1])})
            .sum();

        return res;
    }
    0
}

fn part2(filepath: &str) -> i64 {
    if let Ok(space) = aoc::read_char_vec(filepath) {
        let mut expanded_space = Vec2D::from(space).unwrap();

        let mut y = 0;
        for _ in 0..expanded_space.height {
            if expanded_space.row(y).iter().filter(|pos| **pos != '.').count() == 0 {
                let _ = expanded_space.repeat_replace_row('!', y);
            }
            y += 1;
        }

        let mut x = 0;
        for _ in 0..expanded_space.width {
            if expanded_space.col(x).iter().filter(|pos| **pos != '.' && **pos != '!').count() == 0 {
                let _ = expanded_space.repeat_replace_col('!', x);
            }
            x += 1;
        }

        let mut galaxies = Vec::<Galaxy>::new();
        let mut id = 0;
        for y in 0..expanded_space.height {
            for x in 0..expanded_space.width {
                match expanded_space.get(y, x) {
                    Some('#') => {id+= 1; galaxies.push(Galaxy(y.try_into().unwrap(), x.try_into().unwrap(), id))},
                    _         => () 
                }
            }
        }

        let res: i64 = galaxies.iter()
            .combinations(2)
            .map(|pair| {pair[0].distance_walk(pair[1], &expanded_space)})
            .sum();

        return res;
    }

    0
}

pub fn main() {
    let filepath = "data/day11.txt";

    println!("Day 11 part 1: {}", part1(filepath));
    println!("Day 11 part 2: {}", part2(filepath));
}
