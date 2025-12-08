use super::{max_joltage, parse_input};

pub fn result(input: &str) -> usize {
    let banks = parse_input(input);

    banks.into_iter().map(|bank| max_joltage(&bank, 12)).sum()
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

        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 172162399742349);
    }
}
