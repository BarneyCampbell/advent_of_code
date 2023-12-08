use std::{cmp::Ordering, iter::zip, str::FromStr, collections::HashSet};

struct ParseHandErr;

#[derive(PartialEq, PartialOrd, Debug)]
enum HandType {
    High,
    Pair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

struct Hand {
    cards: Vec<u8>,
    hand_type: HandType,
    bid: i32
}

impl FromStr for Hand {
    type Err = ParseHandErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((hand, bid_str)) = s.split_once(" ") else { unreachable!() };
        let bid: i32 = bid_str.trim().parse().unwrap();
        let mut cards: Vec<u8> = Vec::new();

        for card in hand.trim().chars() {
            if card.is_numeric() {
                cards.push(card.to_digit(10).unwrap() as u8);
            }
            else {
                cards.push(match card {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => unreachable!()
                })
            }
        }
        let hand_type = get_hand_type(hand);


        Ok(Self { cards, hand_type, bid })
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self {
            cards: vec![],
            hand_type: HandType::High,
            bid: 0
        }
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.hand_type == other.hand_type
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type < other.hand_type {
            return Ordering::Less;
        }
        else if self.hand_type > other.hand_type {
            return Ordering::Greater;
        }
        else {
            if let Some(greater) = one_gt_two(&self.cards, &other.cards) {
                return if greater { Ordering::Greater } else { Ordering::Less }
            }
            else {
                return Ordering::Equal;
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type > other.hand_type {
            return Some(Ordering::Greater);
        }
        else if self.hand_type < other.hand_type {
            return Some(Ordering::Less);
        }
        else {
            if let Some(greater) = one_gt_two(&self.cards, &other.cards) {
                return if greater { Some(Ordering::Greater) } else { Some(Ordering::Less) }
            }
            else {
                return Some(Ordering::Equal);
            }
        }
    }
}

fn one_gt_two(hand_1: &Vec<u8>, hand_2: &Vec<u8>) -> Option<bool> {
    for (card_1, card_2) in zip(hand_1, hand_2) {
        if card_1 > card_2 {
            return Some(true);
        }
        if card_1 < card_2 {
            return Some(false);
        }
    }
    None
}

fn get_hand_type(hand: &str) -> HandType {
    // Work from best down
    // " x of kind ":
    let set: HashSet<char> = hand.chars().collect();

    if set.len() == 1 {
        return HandType::FiveOfKind;
    }
    else if set.len() == 2 {
        // Four of Kind or Full house
        for card in set {
            if hand.chars().filter(|val| *val == card).count() == 4 {
                return HandType::FourOfKind;
            }
        }
        return HandType::FullHouse;
    }
    else if set.len() == 3 {
        // Three of Kind or Two Pair
        for card in set {
            if hand.chars().filter(|val| *val == card).count() == 3 {
                return HandType::ThreeOfKind;
            }
        }
        return HandType::TwoPair;
    }
    else if set.len() == 4 {
        return HandType::Pair;
    }

    HandType::High
}

pub fn part1(filepath: &str) -> i32 { 
    if let Ok(mut hands) = aoc::read_lines::<Hand>(filepath) {
        let mut winnings = 0;
        let mut rank = 1;
        hands.sort();

        for hand in hands {
            winnings += hand.bid * rank;
            rank += 1;
        }

        return winnings;
    }

    0
}

pub fn main() {
    let filepath = "data/day7.txt";

    println!("Day 7 part 1: {}", part1(filepath));
    println!("Day 7 part 2: {}", "todo");
}
