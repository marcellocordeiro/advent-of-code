use crate::parse_input;

pub fn result(input: &str) -> usize {
    _ = parse_input(input);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

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
