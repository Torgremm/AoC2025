#![allow(dead_code)]

use crate::solutions::sol_trait::Solution;
use std::time::Instant;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Day8;

pub fn get_distance(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    dx * dx + dy * dy + dz * dz
}

pub fn connect(a: usize, b: usize, connections: &mut Vec<Vec<usize>>) {
    let mut a_constellation: Option<usize> = None;
    let mut b_constellation: Option<usize> = None;

    for (i, constellation) in connections.iter().enumerate() {
        if constellation.contains(&a) {
            a_constellation = Some(i);
        }
        if constellation.contains(&b) {
            b_constellation = Some(i);
        }
    }

    match (a_constellation, b_constellation) {
        (Some(ai), Some(bi)) => {
            if ai != bi {
                let mut b_constellation_data = connections[bi].clone();
                connections[ai].append(&mut b_constellation_data);
                connections.remove(bi);
            }
        }
        (Some(ai), None) => {
            connections[ai].push(b);
        }
        (None, Some(bi)) => {
            connections[bi].push(a);
        }
        (None, None) => {
            connections.push(vec![a, b]);
        }
    }

}


impl Solution for Day8{
    fn get_answer1() -> i64 {
        let start = Instant::now();
        //assert_eq!(Self::solve1(&Self::get_example_input()), 40);
        let sol = Self::solve1(&Self::get_input());
        println!("Part 1 solved in: {:?}", start.elapsed());
        sol
    }

    fn get_answer2() -> i64 {
        let start = Instant::now();
        assert_eq!(Self::solve2(&Self::get_example_input()), 25272);
        let sol = Self::solve2(&Self::get_input());
        println!("Part 2 solved in: {:?}", start.elapsed());
        sol
    }

    fn solve1(input: &str) -> i64 {
        let data: Vec<(i64, i64, i64)> = input.lines().map(|line| {
            let nums: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            (nums[0], nums[1], nums[2])
        }).collect();

        let length = data.len();
        let mut heap: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();

        for i in 0..length {
            for j in (i+1)..length {
                let dist = get_distance(&data[i], &data[j]);
                let edge = (dist, i, j);
                if heap.len() < 1000 {
                    heap.push(edge);
                } else if dist < heap.peek().unwrap().0{
                    heap.pop();
                    heap.push(edge);
                }
            }
        }

        let mut connections: Vec<Vec<usize>> = Vec::new();
        for _ in 0..heap.len() {
            let (_, a, b) = heap.pop().unwrap();
            connect(a, b, &mut connections);
        }

        let mut count: Vec<usize> = connections.iter().map(|d| d.len()).collect();
        count.sort_by(|a, b| b.cmp(a));
        (count[0] * count[1] * count[2]) as i64

    }

    fn solve2(input: &str) -> i64 {
        let data: Vec<(i64, i64, i64)> = input.lines().map(|line| {
            let nums: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            (nums[0], nums[1], nums[2])
        }).collect();

        let length = data.len();
        let mut heap: BinaryHeap<Reverse<(i64, usize, usize)>> = BinaryHeap::new();

        for i in 0..length {
            for j in (i+1)..length {
                let dist = get_distance(&data[i], &data[j]);
                let edge = (dist, i, j);
                heap.push(Reverse(edge));
            }
        }

        let mut connections: Vec<Vec<usize>> = Vec::new();
            
        while let Some(Reverse((_, a, b))) = heap.pop() {
            connect(a, b, &mut connections);
            let max_size = connections.iter().map(|g| g.len()).max().unwrap();
            if max_size == length {
                return data[a].0 * data[b].0;
            }
        }
        0
    }

    fn get_example_input() -> String {
        "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
".to_string()
    }
    
}