use super::{parse_input, permutations};

#[must_use] 
pub fn result(input: &str) -> usize {
    let groups = parse_input(input);

    groups
        .iter()
        .map(|group| {
            let adjusted_conditions = group
                .conditions
                .chars()
                .chain(['?'])
                .cycle()
                .take((5 * (group.conditions.len() + 1)) - 1)
                .collect::<String>();
            let adjusted_ranges = group.ranges.repeat(5);

            permutations(&adjusted_conditions, &adjusted_ranges)
        })
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
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 525152);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 23903579139437);
    }
}
