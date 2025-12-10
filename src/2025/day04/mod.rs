use std::collections::HashSet;

use crate::common::{
    grid::Grid,
    point::{DOWN, LEFT, Point, RIGHT, UP},
};

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

fn parse_input(input: &str) -> Grid<bool> {
    Grid::from(
        input
            .lines()
            .map(|line| line.chars().map(|ch| ch == '@').collect())
            .collect(),
    )
}

fn accessible_rolls(grid: &Grid<bool>) -> HashSet<Point> {
    let directions = [
        UP,
        UP + RIGHT,
        RIGHT,
        DOWN + RIGHT,
        DOWN,
        DOWN + LEFT,
        LEFT,
        UP + LEFT,
    ];

    let mut points = HashSet::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);

            if !grid[point] {
                continue;
            }

            let adjacent_rolls = directions
                .iter()
                .map(|&dir| point + dir)
                .filter(|p| grid.contains_point(*p))
                .filter(|p| grid[*p])
                .count();

            if adjacent_rolls < 4 {
                points.insert(point);
            }
        }
    }

    points
}
