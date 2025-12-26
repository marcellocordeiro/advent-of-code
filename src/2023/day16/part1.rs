use super::{Beam, Direction, get_visited_count, parse_input};

#[must_use] 
pub fn result(input: &str) -> usize {
    let grid = parse_input(input);

    let first_beam = Beam {
        position: (0, 0),
        direction: Direction::East,
    };

    get_visited_count(&grid, first_beam)
}

#[cfg(test)]
mod tests {
    use super::{
        super::{INPUT, SAMPLE},
        *,
    };

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 46);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 7472);
    }
}
