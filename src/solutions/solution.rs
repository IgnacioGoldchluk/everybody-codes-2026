use std::fmt;

pub struct Solution {
    pub part1: String,
    pub part2: String,
    pub part3: String,
}

pub struct Input {
    pub part1: String,
    pub part2: String,
    pub part3: String,
}

pub trait Solver {
    fn solve(&self, input: Input) -> Solution;
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Part1: {}, Part2: {}, Part3: {}",
            self.part1, self.part2, self.part3
        )
    }
}
