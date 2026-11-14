use std::collections::HashMap;

use crate::solutions::solution;

pub struct Day10Solver;

#[derive(Clone, PartialEq)]
enum CaveCell {
    Empty,
    Wall,
}

#[derive(Clone, Copy)]
enum GhostDirection {
    Up,
    Down,
    Left,
    Right,
}

type Point = (usize, usize);
type Cave = HashMap<Point, CaveCell>;

struct Ghost {
    pub position: Point,
    pub direction: GhostDirection,
    pub collisions: usize,
}

impl Ghost {
    pub fn new(position: Point, direction: GhostDirection) -> Self {
        Self {
            position,
            direction,
            collisions: 0,
        }
    }

    pub fn advance(&mut self) {
        self.position = self.next_position();
    }

    pub fn next_position(&self) -> Point {
        let (x, y) = self.position;

        match self.direction {
            GhostDirection::Down => (x + 1, y),
            GhostDirection::Up => (x - 1, y),
            GhostDirection::Left => (x, y - 1),
            GhostDirection::Right => (x, y + 1),
        }
    }

    pub fn rotate_90(&mut self) {
        let new_direction = match self.direction {
            GhostDirection::Down => GhostDirection::Left,
            GhostDirection::Left => GhostDirection::Up,
            GhostDirection::Up => GhostDirection::Right,
            GhostDirection::Right => GhostDirection::Down,
        };

        self.direction = new_direction;
    }

    pub fn rotate_180(&mut self) {
        let new_direction = match self.direction {
            GhostDirection::Down => GhostDirection::Up,
            GhostDirection::Left => GhostDirection::Right,
            GhostDirection::Up => GhostDirection::Down,
            GhostDirection::Right => GhostDirection::Left,
        };

        self.direction = new_direction;
    }
}

fn build_cave_and_ghosts(input: &str) -> (Cave, Vec<Ghost>) {
    let mut cave = HashMap::new();
    let mut ghosts = vec![];

    for (row, line) in input.lines().enumerate() {
        for (col, character) in line.char_indices() {
            match character {
                '#' => {
                    cave.insert((row, col), CaveCell::Wall);
                }
                '.' => {
                    cave.insert((row, col), CaveCell::Empty);
                }
                '>' => {
                    ghosts.push(Ghost::new((row, col), GhostDirection::Right));
                    cave.insert((row, col), CaveCell::Empty);
                }
                '<' => {
                    ghosts.push(Ghost::new((row, col), GhostDirection::Left));
                    cave.insert((row, col), CaveCell::Empty);
                }
                '^' => {
                    ghosts.push(Ghost::new((row, col), GhostDirection::Up));
                    cave.insert((row, col), CaveCell::Empty);
                }
                'v' => {
                    ghosts.push(Ghost::new((row, col), GhostDirection::Down));
                    cave.insert((row, col), CaveCell::Empty);
                }
                _ => panic!("Unreachable"),
            }
        }
    }

    (cave, ghosts)
}

fn part1(input: &str) -> String {
    let (cave, mut ghosts) = build_cave_and_ghosts(input);

    let mut collisions = 0;

    for _round in 0..10 {
        let mut position_count: HashMap<Point, u8> = HashMap::new();
        for ghost in ghosts.iter_mut() {
            let next_position = ghost.next_position();
            match cave.get(&next_position) {
                Some(CaveCell::Wall) => {
                    collisions += 1;
                    ghost.rotate_90();
                }
                Some(CaveCell::Empty) => {
                    ghost.advance();
                }
                None => panic!("Unreachable"),
            }
            *position_count.entry(ghost.position).or_default() += 1;
        }

        for (position, n_ghosts) in position_count {
            if n_ghosts < 2 {
                continue;
            }

            collisions += n_ghosts;

            ghosts
                .iter_mut()
                .filter(|ghost| ghost.position == position)
                .for_each(|ghost| ghost.rotate_180())
        }
    }

    collisions.to_string()
}

#[cfg(test)]
fn max_collisions() -> u32 {
    10
}

#[cfg(not(test))]
fn max_collisions() -> u32 {
    10_000
}

impl solution::Solver for Day10Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1),
            part2: "".into(),
            part3: "".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_solve() {
        let p1 = r#"###########
#####.#####
###.....###
##.......##
##.>>.<<.##
#v...>...v#
##...v...##
##.......##
###.....###
#####.#####
###########"#;

        let input = solution::Input {
            part1: p1.into(),
            part2: "".into(),
            part3: "".into(),
        };

        let solution = Day10Solver.solve(input);
        assert_eq!(solution.part1, "29");
        assert_eq!(solution.part2, "");
        assert_eq!(solution.part3, "");
    }
}
