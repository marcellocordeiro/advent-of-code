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

impl TryFrom<char> for Field {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Space,
            'S' => Self::Start,
            '^' => Self::Splitter,

            _ => return Err(format!("Invalid field: {value}")),
        })
    }
}

fn parse_input(input: &str) -> Grid<Field> {
    Grid::from(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| Field::try_from(ch).unwrap())
                    .collect()
            })
            .collect(),
    )
}
