use itertools::Itertools;

use super::{area, parse_input};

pub fn result(input: &str) -> usize {
    let tiles = parse_input(input);

    tiles
        .into_iter()
        .combinations(2)
        .map(|comb| area(comb[0], comb[1]))
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
