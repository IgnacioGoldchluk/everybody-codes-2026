use crate::solutions::solution;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Day4Solver;

#[derive(Default)]
struct Time {
    id: u64,
    quest: u64,
    part: u64,
    time: u64,
}

impl FromStr for Time {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self::default();

        for pair in s.split_whitespace() {
            let (key, value) = pair.split_once('=').unwrap();
            let value: u64 = value.parse().unwrap();

            match key {
                "id" => result.id = value,
                "quest" => result.quest = value,
                "part" => result.part = value,
                "time" => result.time = value,
                _ => return Err(format!("Unknown key: {key}")),
            }
        }

        Ok(result)
    }
}

fn part2(input: &str) -> String {
    let times: Result<Vec<Time>, _> = input.lines().map(Time::from_str).collect();
    let times = times.unwrap();

    let mut acc: Vec<(u64, HashSet<(u64, u64)>)> = vec![(0, HashSet::new()); max_id(&times)];

    for time in times {
        acc[time.id as usize].0 += time.time;
        acc[time.id as usize].1.insert((time.quest, time.part));
    }

    acc.sort_by_key(|(total_time, parts)| (-(parts.len() as i32), *total_time));

    // Now the "0" time (id=0) will appear at the last position, meaning we have to increase
    // the real index by 1
    acc.iter()
        .enumerate()
        .fold(0, |acc, (idx, (total, _))| acc + (idx as u64 + 1) * (total))
        .to_string()
}

fn part1(input: &str) -> String {
    let times: Result<Vec<Time>, _> = input.lines().map(Time::from_str).collect();
    let times = times.unwrap();

    let mut acc = vec![0; max_id(&times)];

    for time in times {
        acc[time.id as usize] += time.time
    }

    acc.sort();

    acc.iter()
        .enumerate()
        .fold(0, |acc, (idx, total)| acc + (idx as u64) * (total))
        .to_string()
}

fn max_id(times: &[Time]) -> usize {
    times.iter().map(|t| t.id).max().unwrap() as usize + 1
}

impl solution::Solver for Day4Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1),
            part2: part2(&input.part2),
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
        let p1 = r#"id=1 quest=1 part=1 time=187
id=3 quest=1 part=1 time=413
id=5 quest=1 part=3 time=744
id=2 quest=1 part=3 time=745
id=4 quest=1 part=2 time=558
id=1 quest=1 part=2 time=299
id=1 quest=1 part=3 time=362
id=4 quest=1 part=3 time=1268
id=4 quest=1 part=1 time=457
id=5 quest=1 part=1 time=257
id=2 quest=1 part=1 time=192
id=3 quest=1 part=3 time=1168
id=5 quest=1 part=2 time=346
id=3 quest=1 part=2 time=455
id=2 quest=1 part=2 time=562"#;

        let p2 = r#"id=2 quest=1 part=1 time=100
id=4 quest=2 part=1 time=150
id=1 quest=2 part=1 time=140
id=1 quest=1 part=2 time=180
id=2 quest=2 part=1 time=130
id=4 quest=1 part=1 time=110
id=1 quest=2 part=2 time=210
id=2 quest=2 part=2 time=240
id=1 quest=2 part=3 time=330
id=2 quest=1 part=2 time=170
id=3 quest=1 part=1 time=90
id=4 quest=1 part=2 time=175
id=3 quest=2 part=1 time=120
id=1 quest=1 part=1 time=120
id=3 quest=2 part=2 time=220
id=2 quest=2 part=3 time=350
id=3 quest=1 part=2 time=160"#;

        let input = solution::Input {
            part1: p1.into(),
            part2: p2.into(),
            part3: "".into(),
        };

        let solution = Day4Solver.solve(input);
        assert_eq!(solution.part1, "27598");
        assert_eq!(solution.part2, "6470");
        assert_eq!(solution.part3, "");
    }
}
