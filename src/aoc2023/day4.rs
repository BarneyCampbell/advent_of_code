use std::{collections::HashSet, str::FromStr, fmt::Display};


struct Card {
    id: u32,
    copies: u32,
    winners: HashSet<u32>,
    choices: HashSet<u32>
}

struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sides = s.split(":");
        
        let id: u32 = sides.next()
                           .unwrap()
                           .split_whitespace()
                           .nth(1)
                           .unwrap()
                           .parse()
                           .unwrap();

        let numbers: Vec<&str> = sides.next()
                                      .unwrap()
                                      .split(" | ")
                                      .collect();
        
        let mut winners: HashSet<u32> = HashSet::new();
        let mut choices: HashSet<u32> = HashSet::new();

        for num in numbers[0].split_whitespace() {
            winners.insert(num.parse().unwrap());
        }

        for num in numbers[1].split_whitespace() {
            choices.insert(num.parse().unwrap());
        }

        Ok(Card {id, winners, choices, copies: 1})
    }
}

impl Default for Card {
    fn default() -> Self {
        Card {id: 0, winners: HashSet::new(), choices: HashSet::new(), copies: 0}
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{:?},{:?}", self.id, self.winners, self.choices)
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
    let mut cards: Vec<Card> = Vec::new();
    let mut total = 0;

    if let Ok(res) = aoc::read_lines::<Card>(filepath) {
        cards = res;
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
