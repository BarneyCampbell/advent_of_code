use std::{collections::{BinaryHeap, HashSet}, cmp::Ordering};

#[derive(PartialEq, Clone, Copy, Eq, Debug, Hash)]
enum Direction {
    Up, Down, Left, Right

}
impl Direction {
    fn opposes(&self, other: Self) -> bool {
        match self {
            Direction::Up => other == Direction::Down,
            Direction::Down => other == Direction::Up,
            Direction::Left => other == Direction::Right,
            Direction::Right => other == Direction::Left,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Position {
    x: usize,
    y: usize,
    steps: u32,
    value: u32,
    consecutive: u32,
    dir: Direction,
    parent: Box<Option<Position>>
}
impl Position {
    fn take_step(&self, dir: Direction) -> Self {
        let mut consecutive = 0;
        if dir == self.dir { 
            consecutive = self.consecutive + 1;
        }
        match dir {
            Direction::Up => Self { y: self.y - 1, dir, steps: self.steps+1, consecutive, parent: Box::new(Some(self.clone())), ..*self},
            Direction::Down => Self { y: self.y +1, dir, steps: self.steps+1, consecutive, parent: Box::new(Some(self.clone())), ..*self},
            Direction::Left => Self { x: self.x - 1, dir, steps: self.steps+1, consecutive, parent: Box::new(Some(self.clone())),..*self},
            Direction::Right => Self { x: self.x + 1, dir, steps: self.steps+1, consecutive, parent: Box::new(Some(self.clone())),..*self}
        }
    }
}
impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.value == other.value {
            return self.steps.partial_cmp(&other.steps);
        }
        if self.value > other.value {
            return Some(Ordering::Less);
        }
        if self.value < other.value {
            return Some(Ordering::Greater);
        }

        None
    }
}
impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering { 
        if self.value == other.value {
            return self.steps.cmp(&other.steps);
        }
        if self.value > other.value {
            return Ordering::Less;
        }
        else {
            return Ordering::Greater;
        }
    }
}

type Map = Vec<Vec<u32>>;

fn part1(filepath: &str) -> u32 {
    let map: Map = aoc::read_char_vec(filepath).expect("Something went wrong reading the file")
        .into_iter()
        .map(|v| v.into_iter()
            .map(|c| c.to_digit(10)
                .unwrap())
            .collect())
        .collect();

    let mut visited: HashSet<Position> = HashSet::new();
    let mut queue = BinaryHeap::from([Position { x: 0, y: 0, dir: Direction::Right, steps: 0, value: map[0][0], consecutive: 0, parent: Box::new(None) }]);

    while queue.len() > 0 {
        let from = queue.pop().unwrap();
        if visited.contains(&from) { continue; }
        println!("({}, {}) {}", from.x, from.y, from.value);
        vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right].iter()
            .for_each(|dir| {
                if !dir.opposes(from.dir) && *dir != from.dir && from.consecutive <= 3 {
                    match dir {
                        Direction::Up => if from.y == 0 { return () },
                        Direction::Left => if from.x == 0 { return () },
                        _ => ()
                    }
                    let mut new_position = from.take_step(*dir);
                    new_position.value += map[new_position.y][new_position.x];

                    queue.push(new_position);
                }
            });
        if from.x == map[0].len() && from.y == map.len() {
            return from.value;
        }
        visited.insert(from);
    }

    println!("{} {}", map.len()-1, map[0].len()-1);
   // if let Some(score) = search(&map, Position { x: 0, y: 0, dir: Direction::Right }, (map.len()-1, map[0].len()-1), 10, 0) {
   //     return score;
   // }

    0
}

pub fn main() {
    let filepath = "data/day17small.txt";

    println!("Day 17 part 1: {}", part1(filepath));
}
