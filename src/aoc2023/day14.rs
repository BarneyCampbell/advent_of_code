use std::{fmt::Display, collections::HashMap};

use aoc::data_structures::Vec2D;
use itertools::zip_eq;

fn part1(filepath: &str) -> usize {
    let mut controls = Vec2D::from(aoc::read_char_vec(filepath).expect("Error reading file")).expect("Error creating vec2d");

    for i in 0..controls.width {
        let _ = controls.replace_col(roll_col(controls.col(i), false), i);
    }

    let mut score_total = 0;

    for col in controls.cols() {
        for (i, score) in zip_eq(0..col.len(), (1..col.len()+1).rev()) {
            if col[i] == 'O' {
                score_total += score;
            }
        }
    }

    score_total
}

fn part2(filepath: &str) -> usize {
    let mut controls = Vec2D::from(aoc::read_char_vec(filepath).expect("Error reading file")).expect("Error creating vec2d");

    let mut states = HashMap::<Vec<char>, usize>::new();
    let (mut cycle_start, mut cycle_end) = (0, 0);

    for x in 0..1000000000 {
        for i in 0..controls.width {
            let _ = controls.replace_col(roll_col(controls.col(i), false), i);
        }
        for i in 0..controls.height {
            let _ = controls.replace_row(roll_col(controls.row(i), false), i);
        }
        for i in 0..controls.width {
            let _ = controls.replace_col(roll_col(controls.col(i), true), i);
        }
        for i in 0..controls.height {
            let _ = controls.replace_row(roll_col(controls.row(i), true), i);
        }
        if let Some(pos) = states.get(&controls.data) {
            cycle_start = *pos;
            cycle_end = x;
            break;
        }
        states.insert(controls.data.clone(), x);
    }

    let mut cycles = cycle_end;
    while cycles < 1000000000 {
        cycles += cycle_end - cycle_start;
    }
    cycles -= cycle_end - cycle_start;
    cycles += 2; // 😬???

    while cycles <= 1000000000 {
        for i in 0..controls.width {
            let _ = controls.replace_col(roll_col(controls.col(i), false), i);
        }
        for i in 0..controls.height {
            let _ = controls.replace_row(roll_col(controls.row(i), false), i);
        }
        for i in 0..controls.width {
            let _ = controls.replace_col(roll_col(controls.col(i), true), i);
        }
        for i in 0..controls.height {
            let _ = controls.replace_row(roll_col(controls.row(i), true), i);
        }
        cycles += 1;
    }

    let mut score_total = 0;

    for col in controls.cols() {
        for (i, score) in zip_eq(0..col.len(), (1..col.len()+1).rev()) {
            if col[i] == 'O' {
                score_total += score;
            }
        }
    }

    score_total
}

fn _print<T>(controls: &Vec2D<T>) where T : Clone + Display {
    for row in controls.rows() {
        for item in row {
            print!("{}", item);
        }
        println!("");
    }
    println!("-------------------------------");
}

fn roll_col(mut col: Vec<char>, rev: bool) -> Vec<char> {
    if rev {
        col.reverse();
    }
    let mut i = 1;
    while i < col.len() {
        let mut j = i;
        if col[i] == '#' || col[i] == '.' { i += 1; continue; }

        while col[j] == 'O' && col[j-1] == '.' {
            if col[j] == 'O' {
                col.swap(j, j-1);
            }
            if j == 1 { break }
            j -= 1;
        }
        i += 1;
    }

    if rev {
        col.reverse()
    }
    col
}

pub fn main() {
    let filepath = "data/day14.txt";

    println!("Day 14 part 1: {}", part1(filepath));
    println!("Day 14 part 2: {}", part2(filepath));
}
