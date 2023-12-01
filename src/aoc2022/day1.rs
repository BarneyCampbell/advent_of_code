use aoc;

pub fn part1(filepath: &str) -> i32 {
    let mut largest: i32 = 0;

    if let Ok(lines) = aoc::read_lines_optional::<i32>(filepath) {
        let mut acc = 0;

        for num in lines {
            match num {
                Some(n) => acc += n,
                None    => { largest = if acc > largest { acc } else { largest }; acc = 0 },
            }
        }
    }

    largest
}

#[derive(Debug)]
struct Top {
    one: i32,
    two: i32,
    three: i32,
}

impl Top {
    pub fn insert(self, num: i32) -> Self {
        if num > self.one {
            //Top { one: num, ..self }
            Top { one: num, two: self.one, three: self.two } 
        }
        else if num > self.two {
            //Top { two: num, ..self }
            Top { one: self.one, two: num, three: self.two } 
        }
        else if num > self.three {
            //Top { three: num, ..self }
            Top { one: self.one, two: self.two, three: num } 
        }
        else {
            //Top { ..self }
            self
        }
    }

    pub fn total(&self) -> i32 {
        vec![self.one, self.two, self.three].iter().sum()
    }
}

pub fn part2(filepath: &str) -> i32 {
    let mut top = Top { one: 0, two: 0, three: 0 };
    if let Ok(lines) = aoc::read_lines_optional::<i32>(filepath) {
        let mut acc = 0;

        for num in lines {
            match num {
                Some(n) => acc += n,
                None    => { top = top.insert(acc) ; acc = 0 },
            }
        }
    }
    top.total()
}

pub fn main() {
    let filepath = "data/day1.txt";
    println!("Day 1 part 1: {}", part1(filepath));
    println!("Day 1 part 2: {}", part2(filepath));
}
