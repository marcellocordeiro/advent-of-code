use itertools::Itertools;

use super::parse_input;

pub fn result(input: &str) -> usize {
    let database = parse_input(input);

    let mut ranges = database
        .ranges
        .into_iter()
        .sorted() // TODO: there's something funky here
        .map(|range| Some(range))
        .collect::<Vec<_>>();

    // TODO: try a better algorithm
    for range1_index in 0..(ranges.len() - 1) {
        for range2_index in (range1_index + 1)..ranges.len() {
            let (Some(range1), Some(range2)) = (ranges[range1_index], ranges[range2_index]) else {
                continue;
            };

            let Some(new_range) = merge_ranges(range1, range2) else {
                continue;
            };

            ranges[range1_index] = Some(new_range);
            ranges[range2_index] = None;
        }
    }

    ranges
        .into_iter()
        .flatten()
        .map(|(a, b)| (a..=b).count())
        .sum()
}

fn merge_ranges(range1: (usize, usize), range2: (usize, usize)) -> Option<(usize, usize)> {
    let ((a1, b1), (a2, b2)) = {
        // TODO: there's something funky here
        if range1.0 < range2.0 {
            (range1, range2)
        } else {
            (range2, range1)
        }
    };

    // No overlap
    if b1 < a2 {
        return None;
    }

    let start = a1;
    let end = b1.max(b2);

    Some((start, end))
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

        assert_eq!(result, 14);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 357907198933892);
    }
}
