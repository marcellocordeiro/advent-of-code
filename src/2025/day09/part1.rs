use itertools::Itertools;

use super::{area, parse_input};

pub fn result(input: &str) -> usize {
    let tiles = parse_input(input);

    tiles
        .clone()
        .into_iter()
        .cartesian_product(tiles)
        .map(|(a, b)| area(a, b))
        .max()
        .unwrap()
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

        assert_eq!(result, 50);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 4735222687);
    }
}
