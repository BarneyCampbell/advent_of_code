use std::{collections::HashSet, str::FromStr};


struct Card {
    copies: u32,
    winners: HashSet<u32>,
    choices: HashSet<u32>
}

struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<&str> = s.split(": ")
                                  .nth(1)
                                  .unwrap()
                                  .split(" | ")
                                  .collect();
        
        let winners: HashSet<u32> = numbers[0].split_whitespace()
                                              .collect::<Vec<&str>>()
                                              .iter()
                                              .map(|elem| elem.parse().unwrap())
                                              .collect();

        let choices: HashSet<u32> = numbers[1].split_whitespace()
                                              .collect::<Vec<&str>>()
                                              .iter()
                                              .map(|elem| elem.parse().unwrap())
                                              .collect();


        Ok(Card {winners, choices, copies: 1})
    }
}

impl Default for Card {
    fn default() -> Self {
        Card {winners: HashSet::new(), choices: HashSet::new(), copies: 0}
    }
}

pub fn part1(filepath: &str) -> i32 {
    let mut score:i32 = 0;

    if let Ok(cards) = aoc::read_lines::<Card>(filepath) {
        for card in cards {
            let count = card.winners.intersection(&card.choices).count() as u32;

            score += if count == 0 {0} else {2_i32.pow(count-1)};
        }
    }

    score
}

pub fn part2(filepath: &str) -> i32 {
    let mut total = 0;

    if let Ok(mut cards) = aoc::read_lines::<Card>(filepath) {
        let mut cur: usize = 0;
        while cur < cards.len() {
            let count = cards[cur].winners.intersection(&cards[cur].choices).count();

            for i in (cur+1)..=(cur+count) {
                if i >= cards.len() { break }
                cards[i].copies += 1 * cards[cur].copies;
            } 

            cur += 1;
        }

        for card in cards {
            total += card.copies;
        }
    }

    total.try_into().unwrap()
}

pub fn main() {
    let filepath = "data/day4.txt";
    println!("Day 4 part 1: {}", part1(filepath));
    println!("Day 4 part 2: {}", part2(filepath));
}
