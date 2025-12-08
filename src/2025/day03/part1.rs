use super::{max_joltage, parse_input};

pub fn result(input: &str) -> usize {
    let banks = parse_input(input);

    banks.into_iter().map(|bank| max_joltage(bank, 2)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 357);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 17301);
    }
}
