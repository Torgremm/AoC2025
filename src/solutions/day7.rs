#![allow(dead_code)]
use crate::solutions::sol_trait::Solution;
pub struct Day7;

impl Solution for Day7 {
    fn get_answer1() -> i64 {
        crate::solve_with_time!(1, 21)
    }

    fn get_answer2() -> i64 {
        crate::solve_with_time!(2, 40)
    }

    fn solve1(input: &str) -> i64 {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let rows = map.len();
        let cols = map[0].len();

        let start = map[0].iter().position(|&c| c == 'S').unwrap();

        let mut current_row = vec![false; cols];
        let mut next_row = vec![false; cols];
        current_row[start] = true;

        let mut split_count = 0;

        for row in 1..rows {
            for col in 0..cols {
                if current_row[col] {
                    if map[row][col] == '^' {
                        if col > 0 {
                            next_row[col - 1] = true;
                        }
                        if col + 1 < cols {
                            next_row[col + 1] = true;
                        }
                        split_count += 1;
                    } else {
                        next_row[col] = true;
                    }
                }
            }
            current_row = next_row;
            next_row = vec![false; cols];
        }
        split_count
    }

    fn solve2(input: &str) -> i64 {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let rows = map.len();
        let cols = map[0].len();

        let start = map[0].iter().position(|&c| c == 'S').unwrap();

        let mut current_row = vec![0usize; cols];
        let mut next_row = vec![0usize; cols];
        current_row[start] = 1;

        for row in 1..rows {
            for col in 0..cols {
                let current_count = current_row[col];

                if current_count > 0 {
                    if map[row][col] == '^' {
                        if col > 0 {
                            next_row[col - 1] += current_count;
                        }
                        if col + 1 < cols {
                            next_row[col + 1] += current_count;
                        }
                    } else {
                        next_row[col] += current_count;
                    }
                }
            }
            current_row = next_row;
            next_row = vec![0usize; cols];
        }

        current_row.iter().sum::<usize>() as i64
    }

    fn get_example_input() -> String {
        ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            .to_string()
    }
}
