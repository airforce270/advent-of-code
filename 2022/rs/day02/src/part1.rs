use crate::custom_error::AocError;
use std::result::Result;

enum Choice {
    Rock,
    Paper,
    Scissors,
}

const ROCK_WIN: u8 = 1;
const PAPER_WIN: u8 = 2;
const SCISSORS_WIN: u8 = 3;

const LOST_SCORE: u8 = 0;
const DRAW_SCORE: u8 = 3;
const WIN_SCORE: u8 = 6;

pub fn process(input: &str) -> Result<String, AocError> {
    let mut game_scores: Vec<u32> = vec![];

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let opponent_chose = parse_start(*parts.first().unwrap())?;
        let i_respond_with = parse_respond(*parts.last().unwrap())?;

        game_scores.push(
            match i_respond_with {
                Choice::Rock => {
                    ROCK_WIN
                        + match opponent_chose {
                            Choice::Rock => DRAW_SCORE,
                            Choice::Paper => LOST_SCORE,
                            Choice::Scissors => WIN_SCORE,
                        }
                }
                Choice::Paper => {
                    PAPER_WIN
                        + match opponent_chose {
                            Choice::Rock => WIN_SCORE,
                            Choice::Paper => DRAW_SCORE,
                            Choice::Scissors => LOST_SCORE,
                        }
                }
                Choice::Scissors => {
                    SCISSORS_WIN
                        + match opponent_chose {
                            Choice::Rock => LOST_SCORE,
                            Choice::Paper => WIN_SCORE,
                            Choice::Scissors => DRAW_SCORE,
                        }
                }
            }
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

fn parse_respond(respond: &str) -> Result<Choice, AocError> {
    match respond {
        "X" => Ok(Choice::Rock),
        "Y" => Ok(Choice::Paper),
        "Z" => Ok(Choice::Scissors),
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
        let want = "15";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
