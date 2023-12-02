use crate::custom_error::AocError;

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut possible_games_sum = 0;

    'game: for line in input.lines() {
        let line_parts: Vec<&str> = line.split(": ").collect();
        let game_num: i32 = line_parts
            .first()
            .unwrap()
            .replace("Game ", "")
            .parse()
            .unwrap();

        for game in line_parts.last().unwrap().split("; ").into_iter() {
            for game_item in game.split(", ").map(|item| item.trim()).into_iter() {
                let game_item_parts: Vec<&str> = game_item.split(" ").collect();
                let count: i32 = game_item_parts.first().unwrap().parse().unwrap();
                match *game_item_parts.last().unwrap() {
                    "blue" => {
                        if count > BLUE_LIMIT {
                            continue 'game;
                        }
                    }
                    "green" => {
                        if count > GREEN_LIMIT {
                            continue 'game;
                        }
                    }
                    "red" => {
                        if count > RED_LIMIT {
                            continue 'game;
                        }
                    }
                    n => {
                        return Err(AocError {
                            details: format!("unknown game type: {}", n),
                        })
                    }
                }
            }
        }
        possible_games_sum += game_num;
    }

    Ok(possible_games_sum.to_string())
}

const RED_LIMIT: i32 = 12;
const GREEN_LIMIT: i32 = 13;
const BLUE_LIMIT: i32 = 14;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let want = "8";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
