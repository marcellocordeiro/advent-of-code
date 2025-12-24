use super::{accessible_rolls, parse_input};

pub fn result(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut count = 0;

    loop {
        let removed = accessible_rolls(&grid);
        count += removed.len();

        if removed.is_empty() {
            break;
        }

        for point in removed {
            grid[point] = false;
        }
    }

    count
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

        assert_eq!(result, 43);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 9000);
    }
}
