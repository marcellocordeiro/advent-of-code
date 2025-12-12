use itertools::Itertools;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
}

impl From<char> for Operation {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Add,
            '*' => Self::Mul,

            _ => panic!("Invalid operation: {value}"),
        }
    }
}

#[derive(Debug)]
struct Column {
    length: usize,
    numbers: Vec<String>,
    operation: Operation,
}

fn parse_input(input: &str) -> Vec<Column> {
    let lines = input.lines().collect_vec();

    let number_lines = &lines[0..(lines.len() - 1)];
    let operations = lines[lines.len() - 1]
        .lines()
        .next()
        .unwrap()
        .chars()
        .collect_vec();

    let mut columns = vec![];
    let mut window_start = 0;

    for i in 0..operations.len() {
        if (i + 1) == operations.len() || operations[i + 1] != ' ' {
            let window_end = i;
            let length = (window_end - window_start) + 1;

            let numbers = number_lines
                .iter()
                .map(|line| line[window_start..=window_end].to_string())
                .inspect(|number| assert!(number.len() == length))
                .collect();
            let operation = Operation::from(operations[window_start]);

            let column = Column {
                length,
                numbers,
                operation,
            };

            columns.push(column);

            window_start = i + 1;
        }
    }

    columns
}
