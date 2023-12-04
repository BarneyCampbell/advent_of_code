// row-1,col-1 | row-1 | row-1,col+1
// row  ,col-1 | xxxxx | row  ,col+1
// row+1,col+1 | row+1 | row+1,col+1

use std::collections::HashSet;

fn is_symolic(c: char) -> bool {
    (c.is_ascii_punctuation()) && c != '.'
}


pub fn part1(filepath: &str) -> i64 {
    let mut total: i64 = 0;

    if let Ok(lines) = aoc::read_char_vec(filepath) {
        let mut row = 0;

        while row < lines.len() {
            let mut col = 0;
            let mut num: String = String::from("");
            
            while col < lines[row].len() {
                if lines[row][col].is_numeric() {
                    num.push(lines[row][col]);
                }
                else {
                    num = String::from("");
                }

                let part: bool = if lines[row][col].is_numeric() {
                    let mut is_useful = false;
                    let mut row_check = if row == 0 { 0 } else { row-1 };
                    let mut col_check = if col == 0 { 0 } else { col-1 };

                    for _ in 0..3 {
                        if row_check > row+1 || row_check >= lines.len() { break }
                        for _ in 0..3 {
                            if col_check > col+1 || col_check >= lines[row].len() || is_useful { break }
                            is_useful = is_symolic(lines[row_check][col_check]);
                            col_check += 1;
                        }
                        col_check = if col == 0 { 0 } else { col-1 };
                        row_check += 1;

                        if is_useful { break }
                    }

                    is_useful
                } else {
                    false
                };

                if part {
                    // consume number
                    while lines[row][col].is_numeric() {
                        col += 1;
                        if col >= lines[row].len() || !lines[row][col].is_numeric() { break }
                        num.push(lines[row][col]);
                    }

                    total += num.parse().unwrap_or(0);

                    num = String::from("");
                }

                col += 1;
            }
            row += 1;
        }
    }

    total
}

struct Item {
    value: i32,
    width: usize,
    adjacent: HashSet<(usize, usize)>
}

pub fn part2(filepath: &str) -> i32 {
    let mut total = 0;

    if let Ok(lines) = aoc::read_char_vec(filepath) {
        let mut items: Vec<Vec<Item>> = Vec::new();
        let mut gears: HashSet<(usize, usize)> = HashSet::new();
        let mut row: usize = 0;

        for line in &lines {
            let mut line_items: Vec<Item> = Vec::new();
            let mut col: usize = 0;

            while col < line.len() {
                if line[col].is_numeric() {
                    let row_start = if row == 0 { row } else { row - 1 };
                    let col_start = if col == 0 { col } else { col - 1 };

                    let mut width = 0;
                    let mut num: String = String::from("");

                    while line[col].is_numeric() {
                        num.push(line[col]);
                        width += 1;
                        col += 1;

                        if col >= line.len() { break }
                    }

                    let mut item = Item {value: num.parse::<i32>().unwrap(), width, adjacent: HashSet::new()};

                    let mut row_search: usize = row_start;
                    let mut col_search: usize = col_start;

                    while row_search <= row+1 {
                        if row_search >= lines.len() { break }

                        while col_search <= col_start + item.width+1 {
                        if col_search >= line.len() { break }

                            if lines[row_search][col_search] == '*' {
                                gears.insert((row_search, col_search));
                                item.adjacent.insert((row_search, col_search)); 
                            }
                            col_search += 1;
                        }
                        row_search += 1;
                        col_search = col_start;
                    }

                    if item.adjacent.len() != 0 {
                        line_items.push(item);
                    }
                }
                col += 1;
            }
            items.push(line_items);
            row += 1;
        }

        let itemset = aoc::flatten(items);

        for g in gears {
            let mut engine_count = 0;
            let mut ratio_total = 1;

            for item in &itemset {
                if item.adjacent.contains(&g) {
                    engine_count += 1;
                    ratio_total *= item.value;
                }
            }
            if engine_count == 2 {
                total += ratio_total;
            }
        }

    }

    total
}

pub fn main() {
    let filepath = "data/day3.txt";

    println!("Day 3 part 1: {}", part1(filepath));
    println!("Day 3 part 2: {}", part2(filepath));
}
