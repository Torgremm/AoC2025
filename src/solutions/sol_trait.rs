pub trait Solution {
    fn get_answer1() -> i64;
    fn get_answer2() -> i64;

    fn solve1(input: &str) -> i64;
    fn solve2(input: &str) -> i64;

    fn get_input() -> String {
        let type_name = std::any::type_name::<Self>();
        let day_name = type_name.split("::").last().unwrap();

        let filename = format!("inputs/{}.txt", day_name);

        std::fs::read_to_string(&filename).unwrap()
    }

    fn get_example_input() -> String;
}
#[macro_export]
macro_rules! solve_with_time {
    ($part:literal, $expected:expr) => {{
        let start = std::time::Instant::now();

        let solve = match $part {
            1 => Self::solve1,
            2 => Self::solve2,
            _ => panic!("stupid worm"),
        };

        assert_eq!(solve(Self::get_example_input().as_str()), $expected);

        let result = solve(Self::get_input().as_str());
        println!("Part {} solved in {:?}", $part, start.elapsed());
        result
    }};
}
