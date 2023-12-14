use std::iter::zip;

use aoc::data_structures::Vec2D;

fn part1(filepath: &str) -> usize {
    let mirrors_vec = aoc::read_entire::<String>(filepath).expect("Something went wrong while reading");
    let mirrors = Vec2D::from(mirrors_vec.split("\n\n")
        .map(|i| i.split("\n")
            .collect::<Vec<&str>>())
        .map(|v| v.iter().map(|i| i.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()).collect());
    //mirrors_vec.split(|c| c == &vec!['\n'])
    //    .map(|v| v.into_iter().filter(|i| i != &&vec!['\n']))
    //    .map(|v| Vec2D::from(v.collect()).unwrap());

    let mut n_rows = 0;
    let mut n_cols = 0;

        while let Ok(ref mirror) = mirrors {
        for (left, right) in zip(0..(mirror.width-1), 1..(mirror.width)) {
            let (mut l, mut r) = (left, right);
            let mut mirrored = false;

            while l > 0 && r <= mirror.width {
                mirrored = mirror.col(l) == mirror.col(r);

                if l == 0 { break }
                l -= 1;
                r += 1;

                if !mirrored { break }
            }

            if mirrored {
                println!("adding {} cols", left+1);
                n_cols += left+1;
            }
        }

        for (left, right) in zip(0..(mirror.height), 1..(mirror.height)) {
            let (mut l, mut r) = (left, right);
            let mut mirrored = false;

            while r < mirror.height {
                mirrored = mirror.row(l) == mirror.row(r);

                if l == 0 { break }
                l -= 1;
                r += 1;

                if !mirrored { break }
            }

            if mirrored {
                println!("adding {} rows", right);
                n_rows += right;
            }
        }
    }

    n_cols + (100*n_rows)
}

pub fn main() {
    let filepath = "data/day13.txt";

    println!("Day 13 part 1: {}", part1(filepath));
}
