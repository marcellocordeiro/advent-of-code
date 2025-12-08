use super::parse_input;

struct State {
    current: i32,
    zero_count: usize,
}

pub fn result(input: &str) -> usize {
    let rotations = parse_input(input);

    rotations
        .iter()
        .fold(
            State {
                current: 50,
                zero_count: 0,
            },
            |state, &rotation| {
                if rotation == 0 {
                    return state;
                }

                let next = (state.current + rotation).rem_euclid(100);

                State {
                    current: next,
                    zero_count: state.zero_count + (next == 0) as usize,
                }
            },
        )
        .zero_count
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

        assert_eq!(result, 3);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 1150);
    }
}
