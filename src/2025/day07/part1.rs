use std::collections::HashSet;

use super::parse_input;
use crate::{
    common::point::{DOWN, LEFT, Point, RIGHT},
    year2025::day07::Field,
};

#[must_use] 
pub fn result(input: &str) -> usize {
    let grid = parse_input(input);
    let mut beams = HashSet::<Point>::new();

    let mut split_count = 0;

    for y in 0..(grid.height - 1) {
        // no need to check the last row
        for x in 0..grid.width {
            let point = Point::new(x, y);
            let has_beam = (grid[point] == Field::Start) || beams.contains(&point);

            if !has_beam {
                continue;
            }

            let will_split = grid[point + DOWN] == Field::Splitter;

            if will_split {
                split_count += 1;

                let iter = [LEFT + DOWN, DOWN + RIGHT]
                    .into_iter()
                    .filter_map(|direction| {
                        let next = point + direction;

                        grid.contains_point(next).then_some(next)
                    });

                for next in iter {
                    beams.insert(next);
                }
            } else {
                beams.insert(point + DOWN);
            }
        }
    }

    split_count
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

        assert_eq!(result, 21);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 1656);
    }
}
