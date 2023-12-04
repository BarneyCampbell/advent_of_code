use std::str::FromStr;

struct ParsePairError;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    Rock = 1, //lose
    Paper = 2, //draw
    Scissors = 3 //win
}

fn loss_score(opp: &Move) -> i32 {
    match opp {
        Move::Rock => 3,
        Move::Paper => 1,
        Move::Scissors => 2
    }
}

fn win_score(opp: &Move) -> i32 {
    match opp {
        Move::Rock => 2,
        Move::Paper => 3,
        Move::Scissors => 1
    }
}

#[derive(Debug, Clone, Copy)]
struct Pair {
    opponent: Move,
    you: Move
}

impl Pair {
    pub fn you_win(&self, part2: bool) -> i32 {
        if !part2 {
            let mut score = self.you as i32;

            if self.you == Move::Rock && self.opponent == Move::Scissors ||
            self.you == Move::Paper && self.opponent == Move::Rock ||
            self.you == Move::Scissors && self.opponent == Move::Paper {
                score += 6;
            }
            else if self.you == self.opponent {
                score += 3;
            }

            score
        }
        else {
            match self.you {
                Move::Rock => loss_score(&self.opponent),
                Move::Paper => self.opponent as i32 + 3,
                Move::Scissors => win_score(&self.opponent) + 6
            }
        }
    }
}

impl FromStr for Pair {
    type Err = ParsePairError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves: Vec<&str> = s.split(" ").collect();
        let opp_move: Move;
        let you_move: Move;

        if moves.len() != 2 {        
            return Err(ParsePairError);
        }
        else {
            opp_move = match moves.iter().next() {
                Some(&"A") => Move::Rock,
                Some(&"B") => Move::Paper,
                Some(&"C") => Move::Scissors,
                _          => return Err(ParsePairError)
            };

            you_move = match moves.iter().last() {
                Some(&"X") => Move::Rock,
                Some(&"Y") => Move::Paper,
                Some(&"Z") => Move::Scissors,
                _          => return Err(ParsePairError)
            };
            return Ok(Pair {opponent: opp_move, you: you_move});
        }
    }
}

pub fn part1(filepath: &str) -> i32 {
    let mut score = 0;

    if let Ok(pairs) = aoc::read_lines_optional::<Pair>(filepath) {
        for pair in pairs {
            let result = match pair {
                Some(p) => p.you_win(false),
                None    => 0
            };

            score += result;
        }
    }

    score
}

pub fn part2(filepath: &str) -> i32 {
    let mut score = 0;

    if let Ok(pairs) = aoc::read_lines_optional::<Pair>(filepath) {
        for pair in pairs {
            let result = match pair {
                Some(p) => p.you_win(true),
                None    => 0
            };

            score += result;
        }
    }

    score
}

pub fn main() {
    println!("Day 2 part 1: {}", part1("data/day2.txt"));
    println!("Day 2 part 1: {}", part2("data/day2.txt"));
}
