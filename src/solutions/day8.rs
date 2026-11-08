use crate::solutions::solution;
use std::collections::{HashMap, HashSet};

type Point = (i32, i32);
type Grid = HashMap<Point, i8>;

const TARGET_WARRIORS: usize = 100000;

pub struct Day8Solver;

#[derive(Clone, Debug)]
enum Direction {
    LeftDown,  // ↙
    RightDown, // ↘
    Left,      // <-
    RightUp,   // ↗
    LeftUp,    // ↖
    Right,     // ->
}

fn next_point(point: &Point, shadow_diff: i32, direction: &Direction) -> Point {
    let &(x, y) = point;

    match direction {
        Direction::Left => (x, y - shadow_diff * 2),
        Direction::Right => (x, y + shadow_diff * 2),
        Direction::LeftDown => (x + shadow_diff, y - shadow_diff),
        Direction::RightDown => (x + shadow_diff, y + shadow_diff),
        Direction::RightUp => (x - shadow_diff, y + shadow_diff),
        Direction::LeftUp => (x - shadow_diff, y - shadow_diff),
    }
}

fn points(max_row: i32, max_col: i32, direction: &Direction) -> Box<dyn Iterator<Item = Point>> {
    match direction {
        Direction::Left | Direction::LeftDown => {
            Box::new((0..=max_row).flat_map(move |r| (0..=max_col).rev().map(move |c| (r, c))))
        }
        Direction::Right | Direction::RightDown => {
            Box::new((0..=max_row).flat_map(move |r| (0..=max_col).map(move |c| (r, c))))
        }
        Direction::RightUp => Box::new(
            (0..=max_row)
                .rev()
                .flat_map(move |r| (0..=max_col).map(move |c| (r, c))),
        ),
        Direction::LeftUp => Box::new(
            (0..=max_row)
                .rev()
                .flat_map(move |r| (0..=max_col).rev().map(move |c| (r, c))),
        ),
    }
}

fn part1(input: &str) -> String {
    list_hits(&grid(input), &Direction::RightDown)
        .len()
        .to_string()
}

fn part2(input: &str) -> String {
    let ent_grid = grid(input);
    let directions = [
        Direction::Left,
        Direction::Right,
        Direction::LeftDown,
        Direction::RightDown,
        Direction::LeftUp,
        Direction::RightUp,
    ];

    directions
        .iter()
        .map(|direction| list_hits(&ent_grid, direction).len())
        .sum::<usize>()
        .to_string()
}

fn part3(input: &str) -> String {
    // Generate warriors
    let mut directions = [
        Direction::RightDown,
        Direction::LeftDown,
        Direction::Left,
        Direction::LeftUp,
        Direction::RightUp,
        Direction::Right,
    ]
    .iter()
    .cycle();

    let mut warriors = 0;
    let mut hits = 0;

    let mut ent_grid = grid(input);

    while warriors < TARGET_WARRIORS {
        let new_hits = list_hits(&ent_grid, directions.next().unwrap());
        hits += new_hits.len();

        warriors += update_grid(&mut ent_grid, &new_hits);
    }

    hits.to_string()
}

/// Updates the grid in place and returns the number of generated warriors
fn update_grid(ent_grid: &mut Grid, hits: &HashSet<Point>) -> usize {
    let mut new_warriors = 0;

    for hit_point in hits.iter() {
        let val = ent_grid.get_mut(hit_point).unwrap();

        *val += 1;

        if *val == 10 {
            // Will be -2 this round, then -1, then 0, and then back to 1
            *val = -3;
            new_warriors += 1;
        }
    }

    for value in ent_grid.values_mut() {
        if *value < 1 {
            *value += 1;
        }
    }

    new_warriors
}

fn list_hits(ent_grid: &Grid, direction: &Direction) -> HashSet<Point> {
    let (max_row, max_col) = limits(ent_grid);

    let mut hit: HashMap<Point, bool> = HashMap::new();

    for point in points(max_row, max_col, direction) {
        if let Some(height) = ent_grid.get(&point)
            && *hit.get(&point).unwrap_or(&true)
            && *height > 0
        {
            hit.insert(point, true);

            let mut shadow = *height - 1;

            while shadow > 0 {
                let next_p = next_point(&point, (height - shadow) as i32, direction);

                match ent_grid.get(&next_p) {
                    None => {
                        shadow = 0;
                    }
                    Some(new_height) if *new_height > shadow => {
                        shadow = 0;
                    }
                    Some(_) => {
                        hit.insert(next_p, false);
                        shadow -= 1;
                    }
                }
            }
        }
    }

    hit.iter()
        .filter_map(|(point, &touched)| {
            if touched && ent_grid[point] > 0 {
                Some(*point)
            } else {
                None
            }
        })
        .collect()
}

fn grid(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices().filter_map(move |(col, value)| {
                value
                    .to_digit(10)
                    .map(|digit| ((row as i32, col as i32), digit as i8))
            })
        })
        .collect()
}

fn limits(grid: &Grid) -> Point {
    let mut max_row = 0;
    let mut max_col = 0;

    for (row, col) in grid.keys() {
        max_row = max_row.max(*row);
        max_col = max_col.max(*col);
    }

    (max_row, max_col)
}

impl solution::Solver for Day8Solver {
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
        let p1 = r#"....1.9.6.1.7....
...9.2.3.3.8.1...
..7.8.3.5.1.4.3..
.4.6.7.4.1.1.8.9.
4.5.3.6.5.8.9.4.9
.9.5.1.5.6.2.8.9.
..7.7.6.4.6.2.4..
...9.3.8.3.8.4...
....1.6.6.2.8...."#;

        let input = solution::Input {
            part1: p1.into(),
            part2: p1.into(),
            part3: p1.into(),
        };

        let solution = Day8Solver.solve(input);
        assert_eq!(solution.part1, "32");
        assert_eq!(solution.part2, "187");
        assert_eq!(solution.part3, "899889");
    }
}
