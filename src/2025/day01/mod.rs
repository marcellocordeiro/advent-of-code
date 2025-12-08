pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let (direction, times) = line.split_at(1);
            let times = times.parse::<i32>().unwrap();

            match direction {
                "L" => -times,
                "R" => times,
                _ => panic!("Invalid direction: {direction}"),
            }
        })
        .collect()
}
