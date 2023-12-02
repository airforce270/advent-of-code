use crate::custom_error::AocError;
use std::result::Result;

enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Draw,
    Win,
    Loss,
}

const ROCK_CHOSEN: u8 = 1;
const PAPER_CHOSEN: u8 = 2;
const SCISSORS_CHOSEN: u8 = 3;

const LOSS_SCORE: u8 = 0;
const DRAW_SCORE: u8 = 3;
const WIN_SCORE: u8 = 6;

pub fn process(input: &str) -> Result<String, AocError> {
    let mut game_scores: Vec<u32> = vec![];

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let opponent_chose = parse_start(*parts.first().unwrap())?;
        let need_to_end_with = parse_respond(*parts.last().unwrap())?;

        game_scores.push(
            (match need_to_end_with {
                Outcome::Win => WIN_SCORE,
                Outcome::Draw => DRAW_SCORE,
                Outcome::Loss => LOSS_SCORE,
            } + match opponent_chose {
                Choice::Rock => match need_to_end_with {
                    Outcome::Win => PAPER_CHOSEN,
                    Outcome::Loss => SCISSORS_CHOSEN,
                    Outcome::Draw => ROCK_CHOSEN,
                },
                Choice::Paper => match need_to_end_with {
                    Outcome::Win => SCISSORS_CHOSEN,
                    Outcome::Loss => ROCK_CHOSEN,
                    Outcome::Draw => PAPER_CHOSEN,
                },
                Choice::Scissors => match need_to_end_with {
                    Outcome::Win => ROCK_CHOSEN,
                    Outcome::Loss => PAPER_CHOSEN,
                    Outcome::Draw => SCISSORS_CHOSEN,
                },
            })
            .into(),
        );
    }

    Ok(game_scores.iter().sum::<u32>().to_string())
}

fn parse_start(start: &str) -> Result<Choice, AocError> {
    match start {
        "A" => Ok(Choice::Rock),
        "B" => Ok(Choice::Paper),
        "C" => Ok(Choice::Scissors),
        _ => Err(AocError {
            details: format!("unknown start: {}", start),
        }),
    }
}

fn parse_respond(respond: &str) -> Result<Outcome, AocError> {
    match respond {
        "X" => Ok(Outcome::Loss),
        "Y" => Ok(Outcome::Draw),
        "Z" => Ok(Outcome::Win),
        _ => Err(AocError {
            details: format!("unknown respond: {}", respond),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "A Y
B X
C Z";
        let want = "12";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
