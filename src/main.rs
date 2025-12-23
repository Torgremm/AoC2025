mod solutions;
use crate::solutions::{day1,day3, sol_trait::Solution};

macro_rules! run_day {
    ($daynum:literal) => {{
        paste::paste! {
            let day1 = solutions::[<day $daynum>]::[<Day $daynum>]::get_answer1();
            let day2 = solutions::[<day $daynum>]::[<Day $daynum>]::get_answer2();

            println!("Day {}:", $daynum);
            println!("\tPart 1: {}", day1);
            println!("\tPart 2: {}", day2);
        }
    }};
}

fn main() {
    println!("Day {}:", 1);
    println!("\tPart 1: {}", day1::get_answer1());
    println!("\tPart 2: {}", day1::get_answer2());
    run_day!(2);
    println!("Day {}:", 3);
    println!("\tPart 1: {}", day3::get_answer1());
    println!("\tPart 2: {}", day3::get_answer2());
    run_day!(4);
    run_day!(5);
    run_day!(6);
    run_day!(7);
    run_day!(8);
    run_day!(9);
    run_day!(10);
    run_day!(11);
    run_day!(12);
}
