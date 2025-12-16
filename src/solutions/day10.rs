#![allow(dead_code)]
use crate::solutions::sol_trait::Solution;
use std::collections::{VecDeque,HashSet};

pub struct Day10;

#[derive(Debug)]
struct LineData {
    desired_lights: u16,
    switches: Vec<u16>,
    joltage: Vec<u16>,
}

impl Solution for Day10 {
    fn get_answer1() -> i64 {
        crate::solve_with_time!(1, 7)
    }

    fn get_answer2() -> i64 {
        crate::solve_with_time!(2, 33)
    }

    fn solve1(input: &str) -> i64 {
        let data = parse_data(input);
        let mut total_presses: i64 = 0;

        'lines: for line in &data {
            let mut queue = VecDeque::new();
            let mut visited = vec![false; 1 << 16];
            queue.push_back((0u16, 0 as i64));
            visited[0 as usize] = true;

            while let Some((state,presses)) = queue.pop_front() {
                for s in &line.switches {
                    let new_state = state ^ *s;
                    let new_presses = presses + 1;

                    if new_state == line.desired_lights {
                        total_presses += new_presses;
                        continue 'lines;
                    }

                    if !visited[new_state as usize] {
                        visited[new_state as usize] = true;
                        queue.push_back((new_state, new_presses));
                    }
                }
            }
        }

        total_presses
    }

    fn solve2(input: &str) -> i64 {
        let data = parse_data(input);
        let mut total_presses: i64 = 0;

        'lines: for line in &data {
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            let start = vec![0; line.joltage.len()];
            visited.insert(start.clone());
            queue.push_back((start, 0 as i64));

            while let Some((state,presses)) = queue.pop_front() {
                for s in &line.switches {
                    let mut valid_state = true;
                    let mut new_state = Vec::new();
                    line.joltage.iter().enumerate().for_each(|(i, max_j)| {
                        let bit = s & (1 << i);
                        let new: u16 = state[i] + (bit > 0) as u16;
                        if new > *max_j {
                            valid_state = false;
                        }
                        new_state.push(new);
                    });

                    if !valid_state {
                        continue;
                    }

                    let new_presses = presses + 1;

                    if new_state == line.joltage {
                        total_presses += new_presses;
                        continue 'lines;
                    }

                    if visited.insert(new_state.clone()) {
                        queue.push_back((new_state, new_presses));
                    }
                }
            }
        }
        total_presses
    }

    fn get_example_input() -> String {
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
            .to_string()
    }
}

fn parse_data(input: &str) -> Vec<LineData> {
    let data: Vec<Vec<&str>> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect();

    data.into_iter()
        .map(|tokens| {
            let first = tokens.first().unwrap();
            let lights = first.trim_start_matches('[').trim_end_matches(']');

            let desired_lights: u16 = lights
                .chars()
                .enumerate()
                .fold(0, |acc, (i, c)| if c == '#' { acc | (1 << i) } else { acc });

            let last = tokens.last().unwrap();
            let joltage_inner = &last[1..last.len() - 1];
            let joltage = joltage_inner
                .split(',')
                .map(|t| t.trim().parse::<u16>().expect("Invalid usize in {}"))
                .collect::<Vec<u16>>();

            let mut switches = Vec::new();
            for tok in tokens.iter().skip(1).take(tokens.len().saturating_sub(2)) {
                let switch_data = tok.trim_start_matches('(').trim_end_matches(')').split(',');
                let switch: u16 = switch_data
                    .map(|c| c.trim().parse::<usize>().unwrap())
                    .fold(0, |acc, c| acc | 1 << c);
                switches.push(switch);
            }

            LineData {
                desired_lights,
                switches,
                joltage,
            }
        })
        .collect::<Vec<LineData>>()
}
