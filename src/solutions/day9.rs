#![allow(dead_code)]
use crate::solutions::sol_trait::Solution;
use std::collections::HashSet;
pub struct Day9;

pub fn get_area(a: &(i64, i64), b: &(i64, i64)) -> i64 {
    let dx = (a.0 - b.0).abs() + 1;
    let dy = (a.1 - b.1).abs() + 1;

    dx * dy
}

pub fn is_green(point: &(i64, i64), polygon: &[(i64, i64)]) -> bool {
    let (px, py) = point;
    let mut inside = false;

    let mut prev = polygon.last().unwrap();

    for curr in polygon {
        let (x1, y1) = *prev;
        let (x2, y2) = *curr;

        if (x1 == x2 && px == &x1 && py >= &y1.min(y2) && py <= &y1.max(y2)) ||
           (y1 == y2 && py == &y1 && px >= &x1.min(x2) && px <= &x1.max(x2)) {
            return true;
        }

        if (&y1 > py) != (&y2 > py) {
            let intersect_x = x1 + (py - y1) * (x2 - x1) / (y2 - y1);
            if px < &intersect_x {
                inside = !inside;
            }
        }

        prev = curr;
    }

    inside
}

fn does_leave(
    start: (i64, i64),
    end: (i64, i64),
    polygon: &Vec<(i64, i64)>,
    hor_lines: &HashSet<i64>,
    ver_lines: &HashSet<i64>
) -> bool {
    if start.0 == end.0 { // vertical edge
        let x = start.0;
        let y_min = start.1.min(end.1);
        let y_max = start.1.max(end.1);

        for &y in hor_lines {
            if y > y_min && y < y_max {
                let above = (x, y + 1);
                let below = (x, y - 1);

                if !is_green(&above, polygon) || !is_green(&below, polygon) {
                    return true;
                }
            }
        }
    } else { // horizontal edge
        let y = start.1;
        let x_min = start.0.min(end.0);
        let x_max = start.0.max(end.0);

        for &x in ver_lines {
            if x > x_min && x < x_max {
                let right = (x + 1, y);
                let left  = (x - 1, y);

                if !is_green(&right, polygon) || !is_green(&left, polygon){
                    return true;
                }
            }
        }
    }

    false
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
        let mut max_area: i64 = 0;
        let length = data.len();

        for i in 0..length{
            for j in i+1..length{
                let area = get_area(&data[i],&data[j]);
                
                if area > max_area {
                    max_area = area;
                }
            }
        }
        max_area
    }

    fn solve2(input: &str) -> i64 {
        let data: Vec<(i64,i64)> = input.lines().map(
            |line|{
                let dd: Vec<i64> = line.split(',').map(|c| c.parse().unwrap()).collect();
                (dd[0], dd[1])
            }
        ).collect();

        
        let mut max_area: i64 = 0;
        let length = data.len();        

        let hor_lines = HashSet::<i64>::from_iter(
            data.iter().map(|p| p.1)
        );

        let ver_lines = HashSet::<i64>::from_iter(
            data.iter().map(|p| p.0)
        );

        for i in 0..length{
            for j in i+1..length{
                let area = get_area(&data[i],&data[j]);
                if area <= max_area {
                    continue;
                }

                let point3 = (data[i].0, data[j].1);
                let point4 = (data[j].0, data[i].1);

                if !is_green(&point3, &data) || !is_green(&point4, &data) {
                    continue;
                }

                if does_leave(data[i], point3, &data, &hor_lines, &ver_lines) {
                    continue;
                }

                if does_leave(point3, data[j], &data, &hor_lines, &ver_lines) {
                    continue;
                }

                if does_leave(data[i], point4, &data, &hor_lines, &ver_lines) {
                    continue;
                }
                
                if does_leave(point4, data[j], &data, &hor_lines, &ver_lines) {
                    continue;
                }

                max_area = area;
            }
        }
        
        max_area
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