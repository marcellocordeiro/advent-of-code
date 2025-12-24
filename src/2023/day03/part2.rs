use super::{EngineNumber, EngineSymbol, get_surrounding_coordinates, parse_input};

pub fn result(input: &str) -> i32 {
    let (numbers, symbols) = parse_input(input);

    symbols
        .iter()
        .filter(|s| is_gear(s))
        .filter_map(|s| {
            let surrounding_numbers = get_surrounding_numbers(s, &numbers);

            (surrounding_numbers.len() == 2)
                .then_some(surrounding_numbers.into_iter().product::<i32>())
        })
        .sum()
}

fn is_gear(symbol: &EngineSymbol) -> bool {
    symbol.symbol == '*'
}

fn get_surrounding_numbers(symbol: &EngineSymbol, numbers: &[EngineNumber]) -> Vec<i32> {
    let surrounding_coordinates = get_surrounding_coordinates((symbol.row, symbol.column));

    numbers
        .iter()
        .filter_map(|n| {
            surrounding_coordinates
                .iter()
                .any(|(row, column)| *row == n.row && n.column_range.contains(column))
                .then_some(n.number)
        })
        .collect()
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

        assert_eq!(result, 467835);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 80253814);
    }
}
