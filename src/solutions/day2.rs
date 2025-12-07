#![allow(dead_code)]

use crate::solutions::sol_trait::Solution;
use std::time::Instant;
use std::collections::HashSet;

pub struct Day2;

impl Solution for Day2{
    fn get_answer1() -> i64 {
        let input = Self::get_input();
        assert_eq!(Self::solve1(&Self::get_example_input()), 1227775554);
        let start = Instant::now();
        let sol = Self::solve1(input.as_str());
        println!("Part 1 solved in: {:?}", start.elapsed());
        sol
    }

    fn get_answer2() -> i64 {
        let input = Self::get_input();
        assert_eq!(Self::solve2(&Self::get_example_input()), 4174379265);
        let start = Instant::now();
        let sol = Self::solve2(input.as_str());
        println!("Part 2 solved in: {:?}", start.elapsed());
        sol
    }

    fn solve1(input: &str) -> i64 {
        let data: Vec<(i64, i64)> = input
            .split(',')
            .map(|d| {
                let mut nums = d.trim().split('-');
                let a = nums.next().unwrap().parse::<i64>().unwrap();
                let b = nums.next().unwrap().parse::<i64>().unwrap();
                (a, b)
            })
            .collect();
        
        let mut sum = 0;

        for (start, end) in data {
            let len0 = start.to_string().len();
            let len1 = end.to_string().len();

            let mut process_len = |len: usize| {
                if len % 2 != 0 {
                    return;
                }
                let half_len = len / 2;
                let max_half = 10i64.pow(half_len as u32) - 1;
                let multiplier = 10i64.pow(half_len as u32) + 1;

                for id in 0..=max_half {
                    let val = id * multiplier;
                    if val >= start && val <= end {
                        sum += val;
                    }
                }
            };
            
            process_len(len0);
            if len1 != len0 {
                process_len(len1);
            }
        }

        sum
    }


    fn solve2(input: &str) -> i64 {
        let data: Vec<(i64,i64)> = input
            .split(',')
            .map(|d| {
                let mut nums = d.trim().split('-');
                let a = nums.next().unwrap().parse::<i64>().unwrap();
                let b = nums.next().unwrap().parse::<i64>().unwrap();
                (a,b)
            }).collect();

        let mut sum = 0;
        let mut seen = HashSet::new();

        for (start, end) in data {
            let len0 = start.to_string().len();
            let len1 = end.to_string().len();

            for len in len0..=len1 {
                for n in 1..=len/2 {
                    if len % n != 0 { 
                        continue; 
                    }
                    let repeat_times = len / n;
                    let max_block = 10i64.pow(n as u32) - 1;

                    for id in 0..=max_block {
                        let val = id.to_string().repeat(repeat_times).parse::<i64>().unwrap();
                        if val >= start && val <= end {
                            if seen.insert(val) {
                                sum += val;
                            }
                        }
                    }
                }
            }
        }

        sum
    }

    fn get_example_input() -> String {
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124".to_string()
    }
    
}
