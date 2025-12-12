#![allow(dead_code)]
use crate::solutions::sol_trait::Solution;
pub struct Day12;

struct Present {
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


impl Present {
    fn new(pattern: Vec<&str>) -> Self {
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
            pattern: bool_pattern
        }
    }

    fn area(&self) -> usize {
        self.pattern.iter().flatten().filter(|&&b| b).count()
    }

    fn rotate(&self) -> Present {
        let size_y = self.pattern.len();
        let size_x = self.pattern[0].len();
        let mut new_pattern = vec![vec![false; size_y]; size_x];

        for y in 0..size_y {
            for x in 0..size_x {
                new_pattern[x][size_y - y - 1] = self.pattern[y][x];
            }
        }

        Present {
            pattern: new_pattern
        }
    }
}

impl Solution for Day12{
    fn get_answer1() -> i64 {
        crate::solve_with_time!(1, 2)
    }

    fn get_answer2() -> i64 {
        crate::solve_with_time!(2, 0)
    }

    fn solve1(input: &str) -> i64 {
        let data = parse_data(input);
        0
    }

    fn solve2(input: &str) -> i64 {
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


fn is_placement_line(s: &str) -> bool {
    let s = s.trim();
    if let Some((size, ids)) = s.split_once(':') {
        let mut it = size.split('x').map(|t| t.trim().parse::<usize>());
        it.next().is_some_and(|r| r.is_ok())
            && it.next().is_some_and(|r| r.is_ok())
            && !ids.trim().is_empty()
    } else {
        false
    }
}

fn parse_placement_line(line: &str) -> PresentPlacement {
    let (size, ids) = line.split_once(':').expect("bad placement line");
    let mut sz = size.trim().split('x').map(|s| s.parse::<usize>().unwrap());
    let (x, y) = (sz.next().unwrap(), sz.next().unwrap());
    let present_ids = ids
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    PresentPlacement { present_ids, x, y }
}

fn parse_presents(input: &str) -> Vec<Present> {
    let mut presents = Vec::new();
    let mut lines = input.lines().peekable();

    while let Some(&line) = lines.peek() {
        let line = line.trim();
        if line.is_empty() {
            lines.next();
            continue;
        }
        if is_placement_line(line) {
            break; 
        }
        if line.ends_with(':') {
            lines.next();
            let mut pattern = Vec::new();
            while let Some(&pat) = lines.peek() {
                let t = pat.trim();
                if t.is_empty() || t.ends_with(':') || is_placement_line(t) {
                    break;
                }
                pattern.push(pat);
                lines.next();
            }
            if lines.peek().is_some_and(|l| l.trim().is_empty()) {
                lines.next();
            }
            presents.push(Present::new(pattern));
        } else {
            lines.next();
        }
    }

    presents
}

fn parse_placements(input: &str) -> Vec<PresentPlacement> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .filter(|l| is_placement_line(l))
        .map(|l| parse_placement_line(l))
        .collect()
}