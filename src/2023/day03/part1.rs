use super::{EngineNumber, EngineSymbol, get_surrounding_coordinates, parse_input};

#[must_use] 
pub fn result(input: &str) -> i32 {
    let (numbers, symbols) = parse_input(input);

    numbers
        .iter()
        .filter_map(|n| has_surrounding_symbol(n, &symbols).then_some(n.number))
        .sum()
}

fn has_surrounding_symbol(number: &EngineNumber, symbols: &[EngineSymbol]) -> bool {
    for column in number.column_range.clone() {
        for (row, column) in get_surrounding_coordinates((number.row, column)) {
            let found_symbol = symbols.iter().any(|s| s.row == row && s.column == column);

            if found_symbol {
                return true;
            }
        }
    }

    false
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

        assert_eq!(result, 4361);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 530495);
    }
}
