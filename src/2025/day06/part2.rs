use super::{Operation, parse_input};

#[must_use] 
pub fn result(input: &str) -> usize {
    let columns = parse_input(input);

    columns.iter().fold(0_usize, |acc, column| {
        let mut numbers = vec![];

        for i in 0..column.length {
            let string = column
                .numbers
                .iter()
                .map(|row| row.as_bytes()[i] as char)
                .collect::<String>()
                .trim()
                .to_string();

            if !string.is_empty() {
                numbers.push(string.parse::<usize>().unwrap());
            }
        }

        let column_iter = numbers.iter();

        acc + match column.operation {
            Operation::Add => column_iter.sum::<usize>(),
            Operation::Mul => column_iter.product(),
        }
    })
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

        assert_eq!(result, 3263827);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 7329921182115);
    }
}
