pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .split(',')
        .map(|range| {
            let (a, b) = range.split_once('-').unwrap();

            (a.trim().parse().unwrap(), b.trim().parse().unwrap())
        })
        .collect()
}
