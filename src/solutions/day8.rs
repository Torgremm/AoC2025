#![allow(dead_code)]

use crate::solutions::sol_trait::Solution;
use std::time::Instant;

pub struct Day8;

impl Solution for Day8{
    fn get_answer1() -> i64 {
        let start = Instant::now();
        assert_eq!(Self::solve1(&Self::get_example_input()), 0);
        let sol = Self::solve1(&Self::get_input());
        println!("Part 1 solved in: {:?}", start.elapsed());
        sol
    }

    fn get_answer2() -> i64 {
        let start = Instant::now();
        assert_eq!(Self::solve2(&Self::get_example_input()), 0);
        let sol = Self::solve2(&Self::get_input());
        println!("Part 2 solved in: {:?}", start.elapsed());
        sol
    }

    fn solve1(input: &str) -> i64 {
        0
    }

    fn solve2(input: &str) -> i64 {
        0
    }

    fn get_example_input() -> String {
        "".to_string()
    }
    
}