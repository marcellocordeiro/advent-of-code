use crate::parse_input;
use std::thread::current;

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
            |state, rotation| {
                // Only adjust the cycles
                let rotation_cycles = rotation.abs() / 100;
                let rotation = rotation % 100;

                let state = State {
                    current: state.current,
                    zero_count: state.zero_count + rotation_cycles as usize,
                };

                if rotation == 0 {
                    return state;
                }

                if state.current == 0 {
                    let next = (state.current + rotation).rem_euclid(100);

                    return State {
                        current: next,
                        zero_count: state.zero_count,
                    };
                }

                let current_with_cycles = state.current + rotation;
                let has_clicked = current_with_cycles < 1 || current_with_cycles > 99;

                let next = (state.current + rotation).rem_euclid(100);

                State {
                    current: next,
                    zero_count: state.zero_count + (has_clicked as usize),
                }
            },
        )
        .zero_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 6);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 6738);
    }
}
