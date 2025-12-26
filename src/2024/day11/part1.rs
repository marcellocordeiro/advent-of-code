use super::{iterate, parse_input};

#[must_use] 
pub fn result(input: &str) -> usize {
    let mut stones = parse_input(input);

    //dbg!(&stones);
    for _ in 0..25 {
        iterate(&mut stones);
        //dbg!(&stones);
    }

    stones.len()
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

        assert_eq!(result, 55312);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 189547);
    }
}
