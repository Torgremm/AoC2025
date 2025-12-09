#![allow(dead_code)]
use crate::solutions::sol_trait::Solution;
use std::collections::BinaryHeap;
pub struct Day9;

pub fn get_area(a: &(i64, i64), b: &(i64, i64)) -> i64 {
    let dx = (a.0 - b.0 + 1).abs();
    let dy = (a.1 - b.1 + 1).abs();

    dx * dy
}

impl Solution for Day9{
    fn get_answer1() -> i64 {
        crate::solve_with_time!(1, 50)
    }

    fn get_answer2() -> i64 {
        crate::solve_with_time!(2, 24)
    }

    fn solve1(input: &str) -> i64 {
        let data: Vec<(i64,i64)> = input.lines().map(
            |line|{
                let dd: Vec<i64> = line.split(',').map(|c| c.parse().unwrap()).collect();
                (dd[0], dd[1])
            }
        ).collect();
        let mut maxArea: i64 = 0;
        let length = data.len();

        for i in 0..length{
            for j in i+1..length{
                let area = get_area(&data[i],&data[j]);
                
                if area > maxArea {
                    maxArea = area;
                }
            }
        }
        maxArea
    }

    fn solve2(input: &str) -> i64 {
        let data: Vec<(i64,i64)> = input.lines().map(
            |line|{
                let dd: Vec<i64> = line.split(',').map(|c| c.parse().unwrap()).collect();
                (dd[0], dd[1])
            }
        ).collect();
        
        let mut maxArea: i64 = 0;
        let mut data_iter = data.iter().peekable();
        let mut point_a = data_iter.next().expect("data is size 0");

        while let Some(&point_a) = data_iter.next() {
            let point_b = match data_iter.peek().cloned() {
                Some(p) => p,
                None => break,
            }; 
            
        }
        maxArea
    }

    fn get_example_input() -> String {
        "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3".to_string()
    }

}
