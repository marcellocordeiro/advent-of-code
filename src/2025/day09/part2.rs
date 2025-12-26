use itertools::Itertools;

use super::{area, parse_input};

#[must_use] 
pub fn result(input: &str) -> usize {
    let tiles = parse_input(input);

    let edges = tiles
        .clone()
        .into_iter()
        .circular_tuple_windows::<(_, _)>()
        .collect_vec();

    tiles
        .clone()
        .into_iter()
        .cartesian_product(tiles)
        .filter(|(a, b)| {
            // TODO: rewrite this algorithm (AABB)
            edges.iter().all(|edge| {
                edge.0.0 <= a.0.min(b.0) && edge.1.0 <= a.0.min(b.0)
                    || edge.0.0 >= a.0.max(b.0) && edge.1.0 >= a.0.max(b.0)
                    || edge.0.1 <= a.1.min(b.1) && edge.1.1 <= a.1.min(b.1)
                    || edge.0.1 >= a.1.max(b.1) && edge.1.1 >= a.1.max(b.1)
            })
        })
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

        assert_eq!(result, 24);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 1569262188);
    }
}
