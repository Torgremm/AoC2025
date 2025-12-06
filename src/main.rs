mod solutions;
use solutions::day4::Day4;
use solutions::sol_trait::Solution;

fn main() {

    let day1_1 = solutions::day1::get_answer1();
    let day1_2 = solutions::day1::get_answer2();

    let day2_1 = solutions::day2::get_answer1();
    let day2_2 = solutions::day2::get_answer2();

    let day3_1 = solutions::day3::get_answer1();
    let day3_2 = solutions::day3::get_answer2();

    let day4_1 = solutions::day4::Day4::get_answer1();
    let day4_2 = solutions::day4::Day4::get_answer2();

    println!("Day 1:");
    println!("\tPart 1: {day1_1}");
    println!("\tPart 2: {day1_2}");

    println!("Day 2:");
    println!("\tPart 1: {day2_1}");
    println!("\tPart 2: {day2_2}");

    println!("Day 3:");
    println!("\tPart 1: {day3_1}");
    println!("\tPart 2: {day3_2}");

    println!("Day 4:");
    println!("\tPart 1: {day4_1}");
    println!("\tPart 2: {day4_2}");
}
