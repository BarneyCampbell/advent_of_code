
#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up, Down, Left, Right, None
}

struct Position {
    x: usize,
    y: usize
}

impl Position {
    fn clone_take_step(&self, direction: Direction) -> Option<Self> {
        let mut x = self.x;
        let mut y = self.y;

        match direction {
            Direction::Up => if y == 0 { return None } else { y -= 1 },
            Direction::Down => y += 1,
            Direction::Left => if x == 0 { return None } else { x -= 1 },
            Direction::Right => x += 1,
            _ => ()
        }
        Some(Self { x, y })
    }
}

type Scores = Vec<Vec<u32>>;

fn traverse(maze: &Vec<Vec<char>>, mut position: Position, start_direction: Direction) -> Vec<Scores> {
    let mut scores_res = Vec::<Scores>::new();
    let mut scores: Vec<Vec<u32>> = vec![vec![0; maze[0].len()]; maze.len()];
    let mut direction = start_direction;

    loop {
        let tile = maze[position.y][position.x];
        println!("{} {} @ {tile}", position.x, position.y);
        direction = match tile {
            '/' => match direction {
                Direction::Up    => Direction::Right,
                Direction::Down  => Direction::Left,
                Direction::Left  => Direction::Down,
                Direction::Right => Direction::Up,
                _ => Direction::None
            },
            '\\' => match direction {
                Direction::Up    => Direction::Left,
                Direction::Down  => Direction::Right,
                Direction::Left  => Direction::Up,
                Direction::Right => Direction::Down,
                _ => Direction::None
            },
            '-' => match direction {
                Direction::Left  => Direction::Left,
                Direction::Right => Direction::Right,
                _ => {
                    let mut scores_l = Vec::<Scores>::new();
                    let mut scores_r = Vec::<Scores>::new();

                    if let Some(position_l) = position.clone_take_step(Direction::Left) {
                        scores_l = traverse(maze, position_l, Direction::Left);
                    }
                    if let Some(position_r) = position.clone_take_step(Direction::Right) {
                        scores_r = traverse(maze, position_r, Direction::Right);
                    }

                    for score in scores_l { 
                        scores_res.push(score);
                    }

                    for score in scores_r { 
                        scores_res.push(score);
                    }
                    Direction::None
                }
            },
            '|' => match direction {
                Direction::Up  => Direction::Up,
                Direction::Down => Direction::Down,
                _ => {
                    let mut scores_u = Vec::<Scores>::new();
                    let mut scores_d = Vec::<Scores>::new();

                    if let Some(position_u) = position.clone_take_step(Direction::Up) {
                        scores_u = traverse(maze, position_u, Direction::Up);
                    }
                    if let Some(position_d) = position.clone_take_step(Direction::Down) {
                        scores_d = traverse(maze, position_d, Direction::Down);
                    }

                    for score in scores_u { 
                        scores_res.push(score);
                    }

                    for score in scores_d { 
                        scores_res.push(score);
                    }
                    Direction::None
                }
            },
            _    => {
                scores[position.y][position.x] += 1;
                direction.clone()
            }
        };
        if direction == Direction::None { return scores_res; }

        match direction {
            Direction::Up => if position.y == 0 { break } else { position.y -= 1 },
            Direction::Down => position.y += 1,
            Direction::Left => if position.x == 0 { break } else {position.x -= 1 },
            Direction::Right => position.x += 1,
            _ => ()
        }

        if position.x > maze[0].len() || position.y > maze.len() { break }
    }

    scores_res
}

fn part1(filepath: &str) -> u32 {
    let maze = aoc::read_char_vec(filepath).expect("Error reading file");

    for score_vec in traverse(&maze, Position { x: 0, y: 0 }, Direction::Right).iter() {
        for score in score_vec {
            for s in score {
                print!("{s}");
            }
        }
        println!("");
    }
    println!("oh...");

    0
}

pub fn main() {
    let filepath = "data/day16small.txt";

    println!("Day 16 part 1: {}", part1(filepath));
}
