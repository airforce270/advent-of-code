use crate::custom_error::AocError;
use phf::phf_map;

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut total = 0;

    for line in input.lines() {
        let nums = numbers(line);
        total += match nums.len() {
            0 => {
                return Err(AocError {
                    details: format!("no numbers in line: {}", line),
                })
            }
            1 => (nums.first().unwrap() * 10) + nums.first().unwrap(),
            _ => (nums.first().unwrap() * 10) + nums.last().unwrap(),
        }
    }

    Ok(total.to_string())
}

static NUM_NAMES: phf::Map<&'static str, u32> = phf_map! {
    "0" => 0,
    "1" => 1,
    "2" => 2,
    "3" => 3,
    "4" => 4,
    "5" => 5,
    "6" => 6,
    "7" => 7,
    "8" => 8,
    "9" => 9,
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

fn numbers(line: &str) -> Vec<u32> {
    let mut nums: Vec<u32> = Vec::new();

    for i in 0..line.len() {
        for (name, num) in NUM_NAMES.entries() {
            let possible_match: String = line.chars().skip(i).take(name.len()).collect();
            if possible_match == *name {
                nums.push(*num);
            }
        }
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let want = "281";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
