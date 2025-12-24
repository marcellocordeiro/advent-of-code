use super::{get_proper_min_location, parse_input};

pub fn result(input: &str) -> usize {
    let almanac = parse_input(input);

    let seed_ranges = almanac
        .seeds
        .chunks_exact(2)
        .map(|chunk| {
            let [src_start, length] = chunk.try_into().unwrap();

            src_start..(src_start + length)
        })
        .collect::<Vec<_>>();

    seed_ranges
        .iter()
        .flat_map(|range| {
            range
                .clone()
                .map(|seed| get_proper_min_location(seed, &almanac.maps))
        })
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
        let result = result(SAMPLE);

        assert_eq!(result, 46);
    }

    #[test]
    #[ignore = "takes very long to compute"]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 69323688);
    }
}
