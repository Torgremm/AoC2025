#![allow(dead_code)]
use crate::solutions::sol_trait::Solution;
use std::collections::{HashMap, VecDeque};
use std::time::Instant;
pub struct Day11;

impl Solution for Day11{
    fn get_answer1() -> i64 {
        crate::solve_with_time!(1, 5)
    }

    fn get_answer2() -> i64 {
        let input = Self::get_input();
        assert_eq!(Self::solve2(get_example_input2().as_str()), 2);
        let start = Instant::now();
        let sol = Self::solve2(input.as_str());
        println!("Part 2 solved in: {:?}", start.elapsed());
        sol
    }

    fn solve1(input: &str) -> i64 {
        let data = parse_data(input);
        get_routes("you", "out", &data)
    }

    fn solve2(input: &str) -> i64 {
        let data = parse_data(input);
        let paths = get_routes2("svr", "out", &data);

        paths
    }

    fn get_example_input() -> String {
        "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out".to_string()
    }

}

fn parse_data(input: &str) -> HashMap<&str, Vec<&str>> {
    let data = input.lines().map(|line|
        {
            let line_data = line.split(' ').collect::<Vec<&str>>();
            let server = line_data[0].trim_end_matches(':');
            let connections = line_data.iter().skip(1).map(|s| *s).collect::<Vec<&str>>();

            (server, connections)
        }
    ).collect::<HashMap<&str, Vec<&str>>>();
    data
}

fn get_example_input2() -> String {
    "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out".to_string()
}

fn get_routes(start: &str, end: &str, data: &HashMap<&str, Vec<&str>>) -> i64 {
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut paths = 0;

    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if current == end {
            paths += 1;
        }

        if let Some(neighbors) = data.get(current) {
            for &neighbor in neighbors {
                queue.push_back(neighbor);
            }
        }
    }

    paths
}

fn get_routes2(start: &str, end: &str, data: &HashMap<&str, Vec<&str>>) -> i64 {
    let mut queue: VecDeque<(&str, bool, bool)> = VecDeque::new();
    let mut paths = 0;

    queue.push_back((start, false, false));

    while let Some(current) = queue.pop_front() {
        let mut dac_seen = current.1;
        let mut fft_seen = current.2;
        
        if current.0 == end  && dac_seen && fft_seen {
            paths += 1;
        }
        if current.0 == "dac" {
            dac_seen = true;
        }
        if current.0 == "fft" {
            fft_seen = true;
        }

        if let Some(neighbors) = data.get(current.0) {
            for &neighbor in neighbors {
                queue.push_back((neighbor, dac_seen, fft_seen));
            }
        }
    }

    paths
}