#![allow(dead_code)]
use crate::solutions::sol_trait::Solution;
use std::collections::{HashMap, VecDeque};

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

            while let Some((state, presses)) = queue.pop_front() {
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
        let mut total_presses = 0;

        for line in &data {
            let n_bits = line.joltage.len();
            let patterns = build_patterns(&line.switches, n_bits, 8);

            let mut cache = HashMap::<Vec<u16>, Option<i64>>::new();

            let presses = solve_recurse(line.joltage.clone(), &patterns, &mut cache)
                .expect("No solution found");

            total_presses += presses;
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

fn solve_recurse(
    target: Vec<u16>,
    patterns: &Vec<Vec<Vec<u16>>>,
    cache: &mut HashMap<Vec<u16>, Option<i64>>,
) -> Option<i64> {
    if let Some(v) = cache.get(&target) {
        return *v;
    }

    // base case
    if target.iter().all(|&v| v == 0) {
        return Some(0);
    }

    let mask = parity_mask(&target);
    let mut best: Option<i64> = None;

    for pattern in &patterns[mask as usize] {
        let Some(after) = apply_pattern(&target, pattern) else {
            continue;
        };

        // must now be all even
        if after.iter().any(|v| v & 1 == 1) {
            continue;
        }

        let half: Vec<u16> = after.iter().map(|v| v / 2).collect();

        if let Some(sub) = solve_recurse(half, patterns, cache) {
            let cost = pattern.len() as i64 + 2 * sub;
            best = Some(best.map_or(cost, |b| b.min(cost)));
        }
    }

    cache.insert(target.clone(), best);
    best
}

fn build_patterns(switches: &[u16], n_bits: usize, max_depth: usize) -> Vec<Vec<Vec<u16>>> {
    let max_state = 1 << n_bits;

    let mut dp = vec![vec![Vec::<u64>::new(); max_state]; max_depth + 1];

    dp[0][0].push(0);

    for d in 0..max_depth {
        let (prev, next) = dp.split_at_mut(d + 1);
        let curr_layer = &prev[d];
        let next_layer = &mut next[0];

        for state in 0..max_state {
            for &mask in &curr_layer[state] {
                for (i, &s) in switches.iter().enumerate() {
                    let next_state = state ^ s as usize;
                    let next_mask = mask ^ (1u64 << i);

                    if !next_layer[next_state].contains(&next_mask) {
                        next_layer[next_state].push(next_mask);
                    }
                }
            }
        }
    }

    let mut patterns = vec![Vec::<Vec<u16>>::new(); max_state];

    for d in 0..=max_depth {
        for state in 0..max_state {
            for &mask in &dp[d][state] {
                patterns[state].push(mask_to_pattern(mask, switches));
            }
        }
    }

    patterns
}

fn mask_to_pattern(mask: u64, switches: &[u16]) -> Vec<u16> {
    let mut pattern = Vec::new();
    let mut m = mask;

    while m != 0 {
        let i = m.trailing_zeros() as usize;
        m &= m - 1;
        pattern.push(switches[i]);
    }

    pattern
}

fn parity_mask(target: &[u16]) -> u16 {
    let mut mask = 0u16;
    for (i, &v) in target.iter().enumerate() {
        if v & 1 == 1 {
            mask |= 1 << i;
        }
    }
    mask
}

fn apply_pattern(target: &[u16], pattern: &[u16]) -> Option<Vec<u16>> {
    let mut next = target.to_vec();

    for &switch in pattern {
        for i in 0..next.len() {
            if (switch >> i) & 1 == 1 {
                if next[i] == 0 {
                    return None;
                }
                next[i] -= 1;
            }
        }
    }

    Some(next)
}
