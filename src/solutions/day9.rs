#![allow(dead_code)]
use crate::solutions::sol_trait::Solution;
use std::collections::HashSet;
use bevy::prelude::*;
pub struct Day9;

pub fn get_area(a: &(i64, i64), b: &(i64, i64)) -> i64 {
    let dx = (a.0 - b.0 + 1).abs();
    let dy = (a.1 - b.1 + 1).abs();

    dx * dy
}

pub fn is_green(point: &(i64, i64), polygon: &Vec<(i64, i64)>) -> bool {
    let (px, py) = *point;

    for window in polygon.windows(2) {
        let (a, b) = (&window[0], &window[1]);
        if (a.0 == b.0 && px == a.0 && py >= a.1.min(b.1) && py <= a.1.max(b.1)) ||
           (a.1 == b.1 && py == a.1 && px >= a.0.min(b.0) && px <= a.0.max(b.0)) {
            return true;
        }
    }
    let a = &polygon[0];
    let b = polygon.last().unwrap();
    if (a.0 == b.0 && px == a.0 && py >= a.1.min(b.1) && py <= a.1.max(b.1)) ||
       (a.1 == b.1 && py == a.1 && px >= a.0.min(b.0) && px <= a.0.max(b.0)) {
        return true;
    }

    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        if (yi > py) != (yj > py) {
            let intersect_x = xj + (py - yj) * (xi - xj) / (yi - yj);
            if px < intersect_x {
                inside = !inside;
            }
        }

        j = i;
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

fn does_leave_slow(
    start: (i64, i64),
    end: (i64, i64),
    polygon: &Vec<(i64, i64)>
) -> bool {
    if start.0 == end.0 { // vertical edge
        let x = start.0;
        for y in start.1.min(end.1)..=start.1.max(end.1) {
            if !is_green(&(x, y), polygon) {
                return true;
            }
        }
    } else { // horizontal edge
        let y = start.1;
        for x in start.0.min(end.0)..=start.0.max(end.0) {
            if !is_green(&(x, y), polygon) {
                return true;
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

pub fn visualize_solution(
    polygon: &Vec<(i64, i64)>,
    rect_bottom_left: (i64, i64),
    rect_top_right: (i64, i64),
) {
    let polygon = polygon.clone();
    // Compute bounds for scaling
    let x_min = polygon.iter().map(|(x,_)| *x).min().unwrap() as f32;
    let x_max = polygon.iter().map(|(x,_)| *x).max().unwrap() as f32;
    let y_min = polygon.iter().map(|(_,y)| *y).min().unwrap() as f32;
    let y_max = polygon.iter().map(|(_,y)| *y).max().unwrap() as f32;

    let window_width = 800.0;
    let window_height = 800.0;

    let scale_x = window_width / (x_max - x_min);
    let scale_y = window_height / (y_max - y_min);
    let scale = scale_x.min(scale_y);

    let to_screen = move |x: f32, y: f32| {
        Vec3::new(
            (x - x_min) * scale - window_width / 2.0,
            (y - y_min) * scale - window_height / 2.0,
            0.0,
        )
    };

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Polygon + Solution Rectangle".to_string(),
                resolution: (window_width, window_height).into(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(move |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());

            // Draw polygon in red
            for window in polygon.windows(2) {
                let (x1, y1) = window[0];
                let (x2, y2) = window[1];
                let p1 = to_screen(x1 as f32, y1 as f32);
                let p2 = to_screen(x2 as f32, y2 as f32);

                let dx = p2.x - p1.x;
                let dy = p2.y - p1.y;
                let length = (dx*dx + dy*dy).sqrt();
                let angle = dy.atan2(dx);

                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::new(length, 4.0)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new((p1.x + p2.x)/2.0, (p1.y + p2.y)/2.0, 0.0),
                        rotation: Quat::from_rotation_z(angle),
                        ..default()
                    },
                    ..default()
                });
            }

            // Close polygon
            let (x1, y1) = polygon[0];
            let (x2, y2) = *polygon.last().unwrap();
            let p1 = to_screen(x1 as f32, y1 as f32);
            let p2 = to_screen(x2 as f32, y2 as f32);
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            let length = (dx*dx + dy*dy).sqrt();
            let angle = dy.atan2(dx);
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(length, 4.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new((p1.x + p2.x)/2.0, (p1.y + p2.y)/2.0, 0.0),
                    rotation: Quat::from_rotation_z(angle),
                    ..default()
                },
                ..default()
            });

            // Draw solution rectangle in green
            let (rx_min, ry_min) = rect_bottom_left;
            let (rx_max, ry_max) = rect_top_right;

            let rect_points = vec![
                (rx_min, ry_min),
                (rx_max, ry_min),
                (rx_max, ry_max),
                (rx_min, ry_max),
                (rx_min, ry_min), // close rectangle
            ];

            for window in rect_points.windows(2) {
                let (x1, y1) = window[0];
                let (x2, y2) = window[1];
                let p1 = to_screen(x1 as f32, y1 as f32);
                let p2 = to_screen(x2 as f32, y2 as f32);

                let dx = p2.x - p1.x;
                let dy = p2.y - p1.y;
                let length = (dx*dx + dy*dy).sqrt();
                let angle = dy.atan2(dx);

                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::new(length, 4.0)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new((p1.x + p2.x)/2.0, (p1.y + p2.y)/2.0, 0.0),
                        rotation: Quat::from_rotation_z(angle),
                        ..default()
                    },
                    ..default()
                });
            }
        })
        .run();
}