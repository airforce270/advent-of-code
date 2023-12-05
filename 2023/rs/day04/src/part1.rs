use std::collections::HashSet;

use crate::custom_error::AocError;

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut results: Vec<u32> = vec![];

    for line in input.lines() {
        let mut parts = line.split(":").into_iter();

        let _card_no: u32 = parts
            .next()
            .unwrap()
            .replace("Card", "")
            .trim()
            .parse()
            .unwrap();

        let mut all_nums = parts
            .next()
            .unwrap()
            .split("|")
            .into_iter()
            .map(|group| group.trim());
        let winning_nums: HashSet<u32> = parse_space_separated_nums(all_nums.next().unwrap());
        let my_nums: HashSet<u32> = parse_space_separated_nums(all_nums.next().unwrap());

        let mut score = 0;
        for num in winning_nums.iter() {
            if !my_nums.contains(num) {
                continue;
            }

            score = match score {
                0 => 1,
                sc => sc * 2,
            };
        }

        results.push(score);
    }

    Ok(results.iter().sum::<u32>().to_string())
}

fn parse_space_separated_nums(nums: &str) -> HashSet<u32> {
    nums.split(" ")
        .map(|num| num.trim())
        .filter(|num| !num.is_empty())
        .map(|num| num.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let want = "13";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
