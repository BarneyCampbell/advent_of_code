use std::str::FromStr;

struct ParseHistoryErr;

#[derive(Default)]
struct History {
    sequences: Vec<Vec<i32>>
}

impl FromStr for History {
    type Err = ParseHistoryErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sequences: Vec<Vec<i32>> = Vec::new();
        sequences.push(s.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect());

        let mut seq_index = 0;

        while sequences[seq_index].iter().filter(|item| **item != 0).count() != 0 {
            let differences: Vec<i32> = sequences[seq_index].windows(2)
                .map(|window| window[1] - window[0])
                .collect();

            sequences.push(differences);
            seq_index += 1;
        }

        Ok( History { sequences })
    }
}

impl History {
    pub fn get_next(&mut self) -> i32 {
        let mut new_col: Vec<i32> = vec![0];
        let mut downwards = self.sequences.clone();
        downwards.reverse();

        for seq in downwards {
            new_col.push(seq.last().unwrap() + new_col.last().unwrap())
        }

        self.sequences.push(new_col);

        *self.sequences.last().unwrap().last().unwrap()
    }

    pub fn get_prev(&self) -> i32 {
        let mut new_col: Vec<i32> = vec![0];
        let mut downwards = self.sequences.clone();
        downwards.reverse();

        for seq in downwards {
            new_col.push(seq.first().unwrap() - new_col.last().unwrap())
        }

        *new_col.last().unwrap()  
    }
}

fn part1(filepath: &str) -> i32 {
    if let Ok(histories) = aoc::read_lines::<History>(filepath) {
        let mut res = 0;

        for mut history in histories {
            res += history.get_next();
        }

        return res;
    }

    0
}

fn part2(filepath: &str) -> i32 {
    if let Ok(histories) = aoc::read_lines::<History>(filepath) {
        let mut res = 0;

        for history in histories {
            res += history.get_prev();
        }

        return res;
    }

    0
}

pub fn main() {
    let filepath = "data/day9.txt";

    println!("Day 9 part 1: {}", part1(filepath));
    println!("Day 9 part 2: {}", part2(filepath));
}
