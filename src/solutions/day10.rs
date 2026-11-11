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
type Ghost = (Point, GhostDirection);

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
                    ghosts.push(((row, col), GhostDirection::Right));
                    cave.insert((row, col), CaveCell::Empty);
                }
                '<' => {
                    ghosts.push(((row, col), GhostDirection::Left));
                    cave.insert((row, col), CaveCell::Empty);
                }
                '^' => {
                    ghosts.push(((row, col), GhostDirection::Up));
                    cave.insert((row, col), CaveCell::Empty);
                }
                'v' => {
                    ghosts.push(((row, col), GhostDirection::Down));
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
            let next_point = next_position(ghost);
            match cave.get(&next_point) {
                Some(CaveCell::Wall) => {
                    collisions += 1;
                    ghost.1 = rotate_90_clockwise(&ghost.1);
                }
                Some(CaveCell::Empty) => {
                    ghost.0 = next_point;
                }
                None => panic!("Unreachable"),
            }
            *position_count.entry(ghost.0).or_default() += 1;
        }

        for (position, n_ghosts) in position_count {
            if n_ghosts < 2 {
                continue;
            }

            collisions += n_ghosts;

            for ghost in ghosts.iter_mut().filter(|(p, _)| *p == position) {
                ghost.1 = rotate_180(&ghost.1);
            }
        }
    }

    collisions.to_string()
}

fn rotate_180(direction: &GhostDirection) -> GhostDirection {
    match direction {
        GhostDirection::Down => GhostDirection::Up,
        GhostDirection::Left => GhostDirection::Right,
        GhostDirection::Up => GhostDirection::Down,
        GhostDirection::Right => GhostDirection::Left,
    }
}

fn rotate_90_clockwise(direction: &GhostDirection) -> GhostDirection {
    match direction {
        GhostDirection::Down => GhostDirection::Left,
        GhostDirection::Left => GhostDirection::Up,
        GhostDirection::Up => GhostDirection::Right,
        GhostDirection::Right => GhostDirection::Down,
    }
}

fn next_position(ghost: &Ghost) -> Point {
    let ((x, y), direction) = *ghost;

    match direction {
        GhostDirection::Down => (x + 1, y),
        GhostDirection::Up => (x - 1, y),
        GhostDirection::Left => (x, y - 1),
        GhostDirection::Right => (x, y + 1),
    }
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
