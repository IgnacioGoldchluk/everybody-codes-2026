use crate::solutions::solution;

pub struct Day1Solver;

fn part1(input: &str) -> String {
    input
        .trim()
        .split(",")
        .map(|num| num.parse::<u32>().unwrap())
        .map(next_square_distance)
        .sum::<u32>()
        .to_string()
}

/// Returns the distance to the next perfect square
fn next_square_distance(num: u32) -> u32 {
    for idx in 1.. {
        let sq = idx * idx;
        if sq == num {
            return 0;
        } else if (sq) > num {
            return sq - num;
        }
    }

    panic!("Unreachable");
}

/// Returns the distance to the next frame perimeter
fn next_perimeter_distance(num: u32) -> u32 {
    for idx in 3.. {
        // Top and bottom, then on the sides it's 2 less because
        // we don't count the edges
        let p = (idx * 2) + ((idx - 2) * 2);
        if p == num {
            return 0;
        } else if p > num {
            return p - num;
        }
    }

    panic!("Unreachable");
}

/// Returns the distance to the next almost filled square
fn next_almost_filled_square(num: u32) -> u32 {
    for idx in (3..).step_by(2) {
        let sq = (idx * idx) - 1;
        if sq == num {
            return 0;
        } else if sq > num {
            return sq - num;
        }
    }

    panic!("Unreachable");
}

fn part2(input: &str) -> String {
    input
        .trim()
        .split(",")
        .map(|num| num.parse::<u32>().unwrap())
        .map(next_perimeter_distance)
        .sum::<u32>()
        .to_string()
}

fn part3(input: &str) -> String {
    input
        .trim()
        .split(",")
        .map(|num| num.parse::<u32>().unwrap())
        .map(next_almost_filled_square)
        .sum::<u32>()
        .to_string()
}

impl solution::Solver for Day1Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1),
            part2: part2(&input.part2),
            part3: part3(&input.part3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_solve() {
        let p1 = "3,11,12,16,24,25";
        let p2 = "3,11,12,16,24,25";
        let p3 = "3,11,12,16,24,25";
        let input = solution::Input {
            part1: p1.into(),
            part2: p2.into(),
            part3: p3.into(),
        };

        let solution = Day1Solver.solve(input);
        assert_eq!(solution.part1, "11");
        assert_eq!(solution.part2, "9");
        assert_eq!(solution.part3, "61");
    }
}
