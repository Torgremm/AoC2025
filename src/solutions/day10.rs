#![allow(dead_code)]
use crate::solutions::sol_trait::Solution;
use std::collections::HashSet;
pub struct Day10;


#[derive(Debug)]
struct LineData {
    desired_lights: HashSet<usize>,
    switches: Vec<HashSet<usize>>,
    joltage: Vec<usize>,
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
            
            let desired_lights: HashSet<usize> = lights
                .chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '#' { Some(i) } else { None })
                .collect();


            let last = tokens.last().unwrap();
            let joltage_inner = &last[1..last.len() - 1];
            let joltage = joltage_inner
                .split(',')
                .map(|t| t.trim().parse::<usize>().expect("Invalid usize in {}"))
                .collect::<Vec<usize>>();

            let mut switches = Vec::new();
            for tok in tokens.iter().skip(1).take(tokens.len().saturating_sub(2)) {
                let inner = &tok[1..tok.len() - 1];
                let nums = inner
                    .split(',')
                    .map(|t| t.trim().parse::<usize>().expect("Invalid usize in ()"))
                    .collect::<HashSet<usize>>();
                switches.push(nums);
            }

            LineData {
                desired_lights,
                switches,
                joltage,
            }
        })
        .collect::<Vec<LineData>>()
}

impl Solution for Day7{
    fn get_answer1() -> i64 {
        crate::solve_with_time!(1, 0)
    }

    fn get_answer2() -> i64 {
        crate::solve_with_time!(2, 0)
    }

    fn solve1(input: &str) -> i64 {
        let data = parse_data(input);

        for line in &data {

            let mut swtchs = line.switches.iter().peekable();
            let mut new_swtchs = Vec::new();
            while let Some(swtch) = swtchs.next() {
                let sym = swtchs.next().unwrap().symmetric_difference(swtchs.peek().unwrap()).collect::<HashSet<_>>();
                if sym == line.desired_lights {
                    break;
                }
                new_swtchs.push(sym.into_iter().cloned().collect());
            }
            
        }

        0

    }

    fn solve2(input: &str) -> i64 {
        let data = parse_data(input);
        0

    }

    fn get_example_input() -> String {
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string()
    }

}
