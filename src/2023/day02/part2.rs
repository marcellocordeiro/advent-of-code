use super::{Game, parse_input};

pub fn result(input: &str) -> i32 {
    let games = parse_input(input);

    games.iter().map(each_result).sum()
}

fn each_result(game: &Game) -> i32 {
    game.plays
        .iter()
        .fold([0, 0, 0], |acc, play| {
            [
                acc[0].max(play.red),
                acc[1].max(play.green),
                acc[2].max(play.blue),
            ]
        })
        .iter()
        .product()
}

#[cfg(test)]
mod tests {
    use super::{
        super::{INPUT, SAMPLE, parse_game},
        *,
    };

    #[test]
    fn test_each_sample_line() {
        let lines = SAMPLE.lines();
        let results = [48, 12, 1560, 630, 36];

        assert_eq!(lines.clone().count(), results.len());

        for (line, expected_result) in lines.zip(results) {
            let game = parse_game(line);
            let actual_result = each_result(&game);

            assert_eq!(actual_result, expected_result, "for {line}");
        }
    }

    #[test]
    fn test_sample() {
        let actual_result = result(SAMPLE);

        assert_eq!(actual_result, 2286);
    }

    #[test]
    fn test_input() {
        let actual_result = result(INPUT);

        assert_eq!(actual_result, 72706);
    }
}
