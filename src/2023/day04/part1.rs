use super::{Card, number_matches_count, parse_input};

pub fn result(input: &str) -> i32 {
    let cards = parse_input(input);
    cards.iter().map(score).sum()
}

fn score(card: &Card) -> i32 {
    let count = number_matches_count(card);

    if count > 0 {
        2_i32.pow(count as u32 - 1)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{INPUT, SAMPLE, parse_input},
        *,
    };

    #[test]
    fn test_each_sample_line() {
        let cards = parse_input(SAMPLE);
        let results = [8, 2, 2, 1, 0, 0];

        assert_eq!(cards.len(), results.len());

        for (card, expected_result) in cards.into_iter().zip(results) {
            let id = card.id;
            let actual_result = score(&card);

            assert_eq!(actual_result, expected_result, "for {id}");
        }
    }

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 13);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 15268);
    }
}
