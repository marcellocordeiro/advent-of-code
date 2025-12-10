use crate::common::grid::Grid;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Space,
    Start,
    Splitter,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            '.' => Field::Space,
            'S' => Field::Start,
            '^' => Field::Splitter,

            _ => panic!("Invalid field: {value}"),
        }
    }
}

fn parse_input(input: &str) -> Grid<Field> {
    Grid::from(
        input
            .lines()
            .map(|line| line.chars().map(Field::from).collect())
            .collect(),
    )
}
