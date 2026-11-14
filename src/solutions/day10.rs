use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::solutions::solution;

pub struct Day10Solver;

#[derive(Clone, PartialEq)]
enum CaveCell {
    Empty,
    Wall,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum GhostDirection {
    Up,
    Down,
    Left,
    Right,
}

type Point = (usize, usize);
type Cave = HashMap<Point, CaveCell>;
type States = HashMap<u64, (u64, Vec<u64>)>;

struct Ghost {
    pub position: Point,
    pub direction: GhostDirection,
    pub collisions: u64,
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
        self.direction = match self.direction {
            GhostDirection::Down => GhostDirection::Left,
            GhostDirection::Left => GhostDirection::Up,
            GhostDirection::Up => GhostDirection::Right,
            GhostDirection::Right => GhostDirection::Down,
        };
    }

    pub fn rotate_180(&mut self) {
        self.direction = match self.direction {
            GhostDirection::Down => GhostDirection::Up,
            GhostDirection::Left => GhostDirection::Right,
            GhostDirection::Up => GhostDirection::Down,
            GhostDirection::Right => GhostDirection::Left,
        };
    }

    pub fn is_alive(&self, max_collisions: u64) -> bool {
        self.collisions < max_collisions
    }
}

impl Hash for Ghost {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.direction.hash(state);
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

fn part2(input: &str) -> String {
    max_rounds(input, max_collisions_p2())
}

fn part3(input: &str) -> String {
    max_rounds(input, 1_000_000_000_000)
}

fn max_rounds(input: &str, max_collisions: u64) -> String {
    let (cave, mut ghosts) = build_cave_and_ghosts(input);
    let mut rounds: u64 = 0;

    let mut cycle_states: States = HashMap::new();

    while !ghosts.is_empty() {
        let mut hasher = DefaultHasher::new();
        ghosts.hash(&mut hasher);
        let state_hash = hasher.finish();

        let cycle_info = cycle_states.get(&state_hash).cloned();
        if let Some((prev_round, prev_collisions)) = cycle_info {
            let cycle_len = rounds - prev_round;
            let deltas: Vec<u64> = ghosts
                .iter()
                .zip(prev_collisions.iter())
                .map(|(g, &c)| g.collisions - c)
                .collect();

            let min_k = ghosts
                .iter()
                .enumerate()
                .filter(|(i, _)| deltas[*i] > 0)
                .map(|(i, g)| (max_collisions - 1 - g.collisions) / deltas[i])
                .min();

            if let Some(k) = min_k
                && k > 0
            {
                rounds += k * cycle_len;
                for (i, ghost) in ghosts.iter_mut().enumerate() {
                    ghost.collisions += k * deltas[i];
                }
                cycle_states.clear();
            }
        } else {
            let collisions: Vec<u64> = ghosts.iter().map(|g| g.collisions).collect();
            cycle_states.insert(state_hash, (rounds, collisions));
        }

        let prev_ghost_count = ghosts.len();
        let mut position_count: HashMap<Point, u8> = HashMap::new();

        for ghost in ghosts.iter_mut() {
            match cave.get(&ghost.next_position()) {
                Some(CaveCell::Wall) => {
                    ghost.collisions += 1;
                    ghost.rotate_90();
                }
                Some(CaveCell::Empty) => {
                    ghost.advance();
                }
                None => {
                    panic!("Unreachable");
                }
            }

            if ghost.is_alive(max_collisions) {
                *position_count.entry(ghost.position).or_default() += 1;
            }
        }

        for (position, n_ghosts) in position_count {
            if n_ghosts < 2 {
                continue;
            }

            ghosts
                .iter_mut()
                .filter(|ghost| ghost.position == position)
                .for_each(|ghost| {
                    ghost.rotate_180();
                    ghost.collisions += 1;
                })
        }

        ghosts.retain(|ghost| ghost.is_alive(max_collisions));
        rounds += 1;

        if ghosts.len() < prev_ghost_count {
            cycle_states.clear();
        }
    }

    rounds.to_string()
}

fn part1(input: &str) -> String {
    let (cave, mut ghosts) = build_cave_and_ghosts(input);

    let mut collisions = 0;

    for _round in 0..10 {
        let mut position_count: HashMap<Point, u8> = HashMap::new();
        for ghost in ghosts.iter_mut() {
            match cave.get(&ghost.next_position()) {
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
fn max_collisions_p2() -> u64 {
    10
}

#[cfg(not(test))]
fn max_collisions_p2() -> u64 {
    10_000
}

impl solution::Solver for Day10Solver {
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

        let p3 = r#"#######################
###########.###########
#######.^>.^.<^.#######
#####..<.>.v^>>^^<#####
####..v<^v.>.>v..^.####
###.................###
###.................###
##.......<...<.<....>##
##...v....>....v.v<.^##
##........###........##
##v......#####.......##
#.<......#####......>.#
##.......#####.......##
##........###........##
##...................##
##...................##
###.................###
###.................###
####...............####
#####.............#####
#######....v....#######
###########.###########
#######################"#;

        let input = solution::Input {
            part1: p1.into(),
            part2: p1.into(),
            part3: p3.into(),
        };

        let solution = Day10Solver.solve(input);
        assert_eq!(solution.part1, "29");
        assert_eq!(solution.part2, "53");
        assert_eq!(solution.part3, "13314695352588");
    }
}
