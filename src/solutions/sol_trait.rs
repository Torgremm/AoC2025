pub trait Solution {
    fn get_answer1() -> i64;
    fn get_answer2() -> i64;

    fn solve1(input: &str) -> i64;
    fn solve2(input: &str) -> i64;

    fn get_input() -> String;
    fn get_example_input() -> String;
}