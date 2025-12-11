use itertools::Itertools;

use super::parse_input;
use crate::year2025::day08::distance_squared;

pub fn result(input: &str) -> usize {
    let points = parse_input(input);

    let conn = points
        .into_iter()
        .permutations(2)
        .map(|perm| {
            let (a, b) = perm.into_iter().collect_tuple().unwrap();
            let distance_squared = distance_squared(&a, &b);

            ((a, b), distance_squared)
        })
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .take(1000)
        .map(|perm| perm.1)
        .collect_vec();

    0
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

        assert_eq!(result, 0);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 0);
    }
}
