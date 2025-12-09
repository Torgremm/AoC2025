mod solutions;
use crate::solutions::sol_trait::Solution;

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
    run_day!(9);
}
