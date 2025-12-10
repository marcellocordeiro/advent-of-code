pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

struct Database {
    ranges: Vec<(usize, usize)>,
    available_ids: Vec<usize>,
}

fn parse_input(input: &str) -> Database {
    let (ranges, available_ids) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            let (a, b) = (a.parse().unwrap(), b.parse().unwrap());

            (a, b)
        })
        .collect();

    let available_ids = available_ids
        .lines()
        .map(|id| id.parse().unwrap())
        .collect();

    Database {
        ranges,
        available_ids,
    }
}
