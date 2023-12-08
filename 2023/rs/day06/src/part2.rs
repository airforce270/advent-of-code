use crate::custom_error::AocError;

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn count_win_scenarios(&self) -> u64 {
        let mut wins = 0;

        for charge_ms in 0..self.time + 1 {
            let distance = charge_ms * (self.time - charge_ms);
            if distance > self.distance {
                wins += 1;
            }
        }

        wins
    }
}

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let race = parse_race(input);
    let win_scenarios = race.count_win_scenarios();
    Ok(win_scenarios.to_string())
}

fn parse_race(input: &str) -> Race {
    let mut lines = input.lines();
    let time = parse_line(lines.next().unwrap());
    let distance = parse_line(lines.next().unwrap());

 Race { time, distance }
}

fn parse_line(line: &str) -> u64 {
    line.split(":")
        .skip(1)
        .next()
        .unwrap()
       .replace(" ", "")
        .parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let want = "71503";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
