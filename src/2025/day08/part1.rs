use std::collections::HashSet;

use itertools::Itertools;

use super::{distance_squared, parse_input};

pub fn result(input: &str) -> usize {
    result_with_max_connections(input, 1000)
}

fn result_with_max_connections(input: &str, max_connections: usize) -> usize {
    let points = parse_input(input)
        .into_iter()
        .sorted()
        .unique()
        .collect_vec();

    let mut circuits = points
        .clone()
        .into_iter()
        .map(|point| HashSet::from([point]))
        .collect_vec();

    let pairs = points
        .into_iter()
        .combinations(2)
        .sorted()
        .unique()
        .map(|perm| {
            let (a, b) = perm.into_iter().collect_tuple().unwrap();
            let distance_squared = distance_squared(&a, &b);

            ((a, b), distance_squared)
        })
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .map(|perm| perm.0);

    for (a, b) in pairs.take(max_connections) {
        let a_set_index = circuits.iter().position(|s| s.contains(&a)).unwrap();
        let b_set_index = circuits.iter().position(|s| s.contains(&b)).unwrap();

        if a_set_index == b_set_index {
            continue;
        }

        let b_set = std::mem::take(&mut circuits[b_set_index]);
        circuits[a_set_index].extend(b_set);
    }

    circuits
        .into_iter()
        .filter(|c| !c.is_empty())
        .sorted_by(|a, b| Ord::cmp(&b.len(), &a.len()))
        .map(|c| c.len())
        .take(3)
        .product()
}

#[cfg(test)]
mod tests {
    use super::{
        super::{INPUT, SAMPLE},
        *,
    };

    #[test]
    fn test_sample() {
        let result = result_with_max_connections(SAMPLE, 10);

        assert_eq!(result, 40);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 84968);
    }
}
