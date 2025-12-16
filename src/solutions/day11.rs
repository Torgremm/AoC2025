#![allow(dead_code)]
use crate::solutions::sol_trait::Solution;
use std::collections::HashMap;
use std::time::Instant;
pub struct Day11;

impl Solution for Day11 {
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
        let mut memo = HashMap::new();

        get_routes("you", "out", &data, &mut memo)
    }

    fn solve2(input: &str) -> i64 {
        let data = parse_data(input);
        let mut memo = HashMap::new();
        get_routes2("svr", "out", &data, &mut memo, false, false)
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
iii: out"
            .to_string()
    }
}

fn parse_data(input: &str) -> HashMap<&str, Vec<&str>> {
    let data = input
        .lines()
        .map(|line| {
            let line_data = line.split(' ').collect::<Vec<&str>>();
            let server = line_data[0].trim_end_matches(':');
            let connections = line_data.iter().skip(1).map(|s| *s).collect::<Vec<&str>>();

            (server, connections)
        })
        .collect::<HashMap<&str, Vec<&str>>>();
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
hhh: out"
        .to_string()
}

fn get_routes(
    start: &str,
    end: &str,
    data: &HashMap<&str, Vec<&str>>,
    memo: &mut HashMap<String, i64>,
) -> i64 {
    if let Some(&cached) = memo.get(start) {
        return cached;
    }

    if start == end {
        return 1;
    }

    let mut total = 0;

    if let Some(neighbors) = data.get(start) {
        for &neighbor in neighbors {
            total += get_routes(neighbor, end, data, memo);
        }
    }

    memo.insert(start.to_string(), total);
    total
}

fn get_routes2(
    start: &str,
    end: &str,
    data: &HashMap<&str, Vec<&str>>,
    memo: &mut HashMap<(String, bool, bool), i64>,
    dac_seen: bool,
    fft_seen: bool,
) -> i64 {
    let dac = dac_seen || start == "dac";
    let fft = fft_seen || start == "fft";

    let key = (start.to_string(), dac, fft);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    if start == end {
        return (dac && fft) as i64;
    }

    let mut total = 0;

    if let Some(neighbors) = data.get(start) {
        for &n in neighbors {
            total += get_routes2(n, end, data, memo, dac, fft);
        }
    }

    memo.insert(key, total);
    total
}
