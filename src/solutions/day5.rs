use crate::solutions::solution;
use std::collections::HashMap;

pub struct Day5Solver;

const LAND: char = '#';
const SAND: char = '=';
const TREE: char = '@';

type Point = (i64, i64);
type LandMap = HashMap<Point, char>;

fn part1(input: &str) -> String {
    count_tiles(&build_land_map(input), LAND).to_string()
}

fn part2(input: &str) -> String {
    let mut land_map = build_land_map(input);
    build_sand_map(&mut land_map);

    count_tiles(&land_map, SAND).to_string()
}

fn part3(input: &str) -> String {
    let mut land_map = build_land_map(input);
    build_sand_map(&mut land_map);

    let land_points: Vec<Point> = land_map
        .iter()
        .filter_map(|(point, value)| if *value == LAND { Some(*point) } else { None })
        .collect();

    apply_trees(land_points, &mut land_map);

    for _ in 0..2 {
        for point in can_grow_tree(&land_map) {
            land_map.insert(point, TREE);
        }
    }

    count_tiles(&land_map, TREE).to_string()
}

fn can_grow_tree(land_map: &LandMap) -> Vec<Point> {
    land_map
        .iter()
        .filter_map(|(point, value)| {
            if *value != LAND {
                return None;
            }
            let mut trees = 0;
            for neighbor in neighbors(point) {
                let neighbor_tile = land_map.get(&neighbor);
                if neighbor_tile == Some(&SAND) {
                    return None;
                }
                if neighbor_tile == Some(&TREE) {
                    trees += 1;
                }
            }

            if trees >= 4 { Some(*point) } else { None }
        })
        .collect()
}

fn apply_trees(land_points: Vec<Point>, land_map: &mut LandMap) {
    for point in land_points {
        if is_tree_pattern(&point, land_map) {
            land_map.insert(point, TREE);
        }
    }
}

fn is_tree_pattern(point: &Point, land_map: &LandMap) -> bool {
    let (x, y) = *point;

    for dx in -2i64..=2 {
        for dy in -2i64..=2 {
            if dx.abs() == 2 && dy.abs() == 2 {
                // Ignore corners
                continue;
            }

            let neighbor_value = land_map.get(&(x + dx, y + dy));
            if neighbor_value != Some(&LAND) && neighbor_value != Some(&TREE) {
                return false;
            }
        }
    }

    true
}

fn build_sand_map(land_map: &mut LandMap) {
    let initials: Vec<Point> = land_map
        .iter()
        .filter(|(point, value)| **value == LAND && is_next_to_water(point, land_map))
        .map(|(point, _)| *point)
        .collect();

    let mut queue: Vec<Point> = Vec::new();

    for new_sand_point in &initials {
        land_map.insert(*new_sand_point, SAND);
        queue.extend(neighbors(new_sand_point));
    }

    while let Some(p) = queue.pop() {
        let p_neighbors = neighbors(&p);

        let is_sand = p_neighbors
            .iter()
            .filter(|neighbor| land_map.get(*neighbor) == Some(&SAND))
            .count()
            >= 5;

        if is_sand {
            land_map.insert(p, SAND);

            for neighbor in p_neighbors {
                if land_map.get(&neighbor) == Some(&LAND) {
                    queue.push(neighbor);
                }
            }
        }
    }
}

fn count_tiles(map: &LandMap, character: char) -> usize {
    map.values().filter(|chr| **chr == character).count()
}

fn is_next_to_water(point: &Point, land_map: &LandMap) -> bool {
    neighbors(point)
        .iter()
        .any(|neighbor| land_map.get(neighbor) != Some(&LAND))
}

fn build_land_map(input: &str) -> LandMap {
    let mut land_map = grid(input);
    let (rows, cols) = land_map.keys().max().unwrap();

    let center = (rows / 2, cols / 2);
    land_map.insert(center, LAND);

    let mut queue = neighbors(&center);

    while let Some(p) = queue.pop() {
        let p_neighbors = neighbors(&p);

        let alive = p_neighbors
            .iter()
            .filter(|p| {
                let val = land_map.get(p);
                val == Some(&LAND) || val == Some(&'1')
            })
            .count()
            >= 5;

        if alive {
            land_map.insert(p, LAND);

            for neighbor in p_neighbors {
                match land_map.get(&neighbor) {
                    None => (),
                    Some(&LAND) => (),
                    Some(_) => queue.push(neighbor),
                }
            }
        }
    }

    land_map
}

fn neighbors(point: &Point) -> Vec<Point> {
    let (x, y) = *point;

    vec![
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ]
}

fn grid(input: &str) -> LandMap {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .map(move |(col, chr)| ((row as i64, col as i64), chr))
        })
        .collect()
}

impl solution::Solver for Day5Solver {
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
        let p1 = r#"010100100100001101000
111101001000001110000
101111011000101010011
001000101010000100000
000110100110011001100
000001110111111000000
101100010101011110000
001101100000001010011
001001001111010001001
100011011110011010101
001100101110010010000"#;

        let input = solution::Input {
            part1: p1.into(),
            part2: p1.into(),
            part3: p1.into(),
        };

        let solution = Day5Solver.solve(input);
        assert_eq!(solution.part1, "137");
        assert_eq!(solution.part2, "69");
        assert_eq!(solution.part3, "14");
    }
}
