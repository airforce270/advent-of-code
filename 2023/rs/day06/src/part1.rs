use crate::custom_error::AocError;

struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn count_win_scenarios(&self) -> u32 {
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
    let races = parse_races(input);
    let win_scenarios = races.iter().map(|race| race.count_win_scenarios());
    Ok(win_scenarios.product::<u32>().to_string())
}

fn parse_races(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = parse_line(lines.next().unwrap());
    let distances = parse_line(lines.next().unwrap());

    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn parse_line(line: &str) -> impl Iterator<Item = u32> + '_ {
    line.split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .map(|num| num.trim())
        .filter(|num| !num.is_empty())
        .map(|num| num.parse::<u32>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let want = "288";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
