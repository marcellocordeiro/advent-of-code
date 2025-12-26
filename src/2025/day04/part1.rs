use super::{accessible_rolls, parse_input};

#[must_use] 
pub fn result(input: &str) -> usize {
    let grid = parse_input(input);

    accessible_rolls(&grid).len()
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

        assert_eq!(result, 13);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 1389);
    }
}
