use super::parse_input;

pub fn result(input: &str) -> usize {
    let ranges = parse_input(input);

    ranges
        .into_iter()
        .map(|(a, b)| {
            (a..=b)
                .filter(|num| {
                    let string = format!("{num}");
                    let length = string.len();

                    for window_end in 0..(length / 2) {
                        let window_size = window_end + 1;

                        // Can't evenly fit this pattern
                        if (length % window_size) != 0 {
                            continue;
                        }

                        let window = &string.as_bytes()[0..window_size];

                        let pattern_repeats = string
                            .as_bytes()
                            .chunks(window_size)
                            .all(|current_window| current_window == window);

                        if pattern_repeats {
                            return true;
                        }
                    }

                    false
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 4174379265);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 17298174201);
    }
}
