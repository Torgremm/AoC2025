#![allow(dead_code)]
use bevy::utils::petgraph::data;

use crate::solutions::sol_trait::Solution;
pub struct Day12;

#[derive(Clone)]
struct Present {
    id: usize,
    pattern: Vec<Vec<bool>>,
}

struct PresentPlacement {
    present_ids: Vec<usize>,
    x: usize,
    y: usize,
}


struct ProblemInstance {
    presents: Vec<Present>,
    placements: Vec<PresentPlacement>, 
}

impl PartialEq for Present {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
    }
}


impl Present {
    fn new(pattern: Vec<&str>, id: usize) -> Self {
        let size_y = pattern.len();
        let size_x = pattern[0].len();
        let mut bool_pattern = vec![vec![false; size_x]; size_y];

        for (y, line) in pattern.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    bool_pattern[y][x] = true;
                }
            }
        }

        Present {
            id,
            pattern: bool_pattern
        }
    }

    fn area(&self) -> usize {
        self.pattern.iter().flatten().filter(|&&b| b).count()
    }

    fn flip(&self) -> Present {
        let size_y = self.pattern.len();
        let size_x = self.pattern[0].len();
        let mut new_pattern = vec![vec![false; size_x]; size_y];

        for y in 0..size_y {
            for x in 0..size_x {
                new_pattern[y][size_x - x - 1] = self.pattern[y][x];
            }
        }

        Present {
            id: self.id,
            pattern: new_pattern,
        }
    }

    fn rotate(&self) -> Present {
        let size_y = self.pattern.len();
        let size_x = self.pattern[0].len();
        let mut new_pattern = vec![vec![false; size_y]; size_x];

        for y in 0..size_y {
            for x in 0..size_x {
                new_pattern[x][size_y - 1 - y] = self.pattern[y][x];
            }
        }

        Present {
            id: self.id,
            pattern: new_pattern,
        }
    }
}

impl Solution for Day12{
    fn get_answer1() -> i64 {
        crate::solve_with_time!(1, 1)
    }

    fn get_answer2() -> i64 {
        crate::solve_with_time!(2, 0)
    }

    fn solve1(input: &str) -> i64 {
        let data = parse_data(input);
        let mut success_count = 0;
        for placement in data.placements {
            let area = placement.x * placement.y;
            if area >= placement.present_ids.iter().sum::<usize>() * 9 {
                success_count += 1;
            }
        }
        success_count
    }

    fn solve2(_input: &str) -> i64 {
        0
    }

    fn get_example_input() -> String {
        "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2".to_string()
    }

}


fn parse_data(input: &str) -> ProblemInstance {
    ProblemInstance {
        presents: parse_presents(input),
        placements: parse_placements(input),
    }
}

fn parse_presents(input: &str) -> Vec<Present> {
    let mut presents = Vec::new();

    let blocks: Vec<String> = input
        .split(|c| c == '\n' || c == '\r') 
        .collect::<Vec<_>>()
        .split(|line| line.trim().is_empty()) 
        .filter(|block| !block.is_empty())   
        .map(|block| block.join("\n"))       
        .collect::<Vec<String>>();

    for block in blocks {
        let mut lines = block.lines();
        if let Some(first_line) = lines.next() {
            let id = first_line.trim_end_matches(':').parse::<usize>().ok();
            if let Some(id) = id {
                let pattern = lines.collect::<Vec<&str>>();

                if pattern.is_empty() {
                    continue;
                }
                presents.push(Present::new(pattern, id));
            }
        }
    }

    presents
}

fn parse_placements(input: &str) -> Vec<PresentPlacement> {
    let block = input.split("\n\n").last().unwrap();
    block.lines().filter(|line| line.contains("x"))
    .map(
        |line|{
            let mut v = line.split_whitespace();
            let mut parts = v.next().unwrap().trim_end_matches(':').split('x');
            let x = parts.next().unwrap().parse::<usize>().unwrap();
            let y = parts.next().unwrap().parse::<usize>().unwrap();
            let present_ids = v.map(|d| d.parse().unwrap()).collect::<Vec<usize>>();

            PresentPlacement {
                present_ids,
                x,
                y,
            }

        }
    ).collect::<Vec<PresentPlacement>>()
}

