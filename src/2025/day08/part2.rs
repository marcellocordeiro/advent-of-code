use std::collections::HashSet;

use itertools::Itertools;

use super::{distance_squared, parse_input};

pub fn result(input: &str) -> usize {
    result_with_max_connections(input)
}

fn result_with_max_connections(input: &str) -> usize {
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

    let mut last_pair = None;

    for (a, b) in pairs {
        let a_set_index = circuits.iter().position(|s| s.contains(&a)).unwrap();
        let b_set_index = circuits.iter().position(|s| s.contains(&b)).unwrap();

        if a_set_index == b_set_index {
            continue;
        }

        last_pair = Some((a.clone(), b.clone()));

        let b_set = std::mem::take(&mut circuits[b_set_index]);
        circuits[a_set_index].extend(b_set);
    }

    last_pair.map(|(a, b)| (a.x * b.x) as usize).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{
        super::{INPUT, SAMPLE},
        *,
    };

    #[test]
    fn test_sample() {
        let result = result_with_max_connections(SAMPLE);

        assert_eq!(result, 25272);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 8663467782);
    }
}
