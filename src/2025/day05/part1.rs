use super::parse_input;

pub fn result(input: &str) -> usize {
    let database = parse_input(input);

    database
        .available_ids
        .iter()
        .filter(|id| database.ranges.iter().any(|(a, b)| (a..=b).contains(id)))
        .count()
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

        assert_eq!(result, 3);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 862);
    }
}
