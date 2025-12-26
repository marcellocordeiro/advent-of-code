use super::parse_input;

#[must_use] 
pub fn result(input: &str) -> usize {
    let ranges = parse_input(input);

    ranges
        .into_iter()
        .map(|(a, b)| {
            (a..=b)
                .filter(|num| {
                    let string = format!("{num}");
                    let (left, right) = string.split_at(string.len() / 2);

                    left == right
                })
                .sum::<usize>()
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
        let result = result(SAMPLE);

        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 12586854255);
    }
}
