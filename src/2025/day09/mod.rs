pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

type Tile = (usize, usize);

fn parse_input(input: &str) -> Vec<Tile> {
    input
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect()
}

fn area((x1, y1): Tile, (x2, y2): Tile) -> usize {
    (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
}
