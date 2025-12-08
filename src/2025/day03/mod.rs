pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn max_joltage(bank: &[usize], size: usize) -> usize {
    let mut window_start = 0;
    let mut joltage = 0;

    for digit in 0..size {
        let digits_to_spare = size - (digit + 1);
        let window_end = bank.len() - digits_to_spare - 1;

        let window = &bank[window_start..=window_end];
        let max = *window.iter().max().unwrap();
        let index = window.iter().position(|x| *x == max).unwrap();

        window_start += index + 1;
        joltage += max * (10_usize.pow((size - digit - 1) as u32));
    }

    joltage
}
