use super::{Operation, parse_input};

#[must_use] 
pub fn result(input: &str) -> usize {
    let columns = parse_input(input);

    columns.iter().fold(0_usize, |acc, column| {
        let column_iter = column
            .numbers
            .iter()
            .map(|number| number.trim().parse::<usize>().unwrap());

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

        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 4648618073226);
    }
}
