use super::{get_proper_min_location, parse_input};

#[must_use] 
pub fn result(input: &str) -> usize {
    let almanac = parse_input(input);

    almanac
        .seeds
        .iter()
        .map(|seed| get_proper_min_location(*seed, &almanac.maps))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{
        super::{INPUT, SAMPLE},
        *,
    };

    #[test]
    fn test_sample() {
        let min = result(SAMPLE);

        assert_eq!(min, 35);
    }

    #[test]
    fn test_input() {
        let min = result(INPUT);

        assert_eq!(min, 111_627_841);
    }
}
