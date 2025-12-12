use std::collections::HashMap;

use super::parse_input;
use crate::{
    common::point::{DOWN, LEFT, Point, RIGHT},
    year2025::day07::Field,
};

pub fn result(input: &str) -> usize {
    let grid = parse_input(input);
    let mut beams = HashMap::<Point, usize>::new();

    for y in 0..(grid.height - 1) {
        // no need to check the last row
        for x in 0..grid.width {
            let point = Point::new(x, y);
            let has_beam = (grid[point] == Field::Start) || beams.contains_key(&point);

            if !has_beam {
                continue;
            }

            let paths_count = if grid[point] == Field::Start {
                1
            } else {
                *beams.get(&point).unwrap()
            };

            let will_split = grid[point + DOWN] == Field::Splitter;

            if will_split {
                let iter = [LEFT + DOWN, DOWN + RIGHT]
                    .into_iter()
                    .filter_map(|direction| {
                        let next = point + direction;

                        grid.contains_point(next).then_some(next)
                    });

                for next in iter {
                    *beams.entry(next).or_default() += paths_count;
                }
            } else {
                *beams.entry(point + DOWN).or_default() += paths_count;
            }
        }
    }

    beams
        .iter()
        .filter(|(key, _)| key.y == (grid.height - 1))
        .map(|(_, value)| *value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{
        super::{INPUT, SAMPLE},
        *,
    };

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 40);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 76624086587804);
    }
}
