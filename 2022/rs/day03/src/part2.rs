use crate::custom_error::AocError;
use phf::phf_map;
use std::collections::HashSet;

#[derive(PartialEq)]
enum CurrentLine {
    One,
    Two,
    Three,
}

static PRIORITIES: phf::Map<&'static str, u32> = phf_map! {
    "a" => 1,
    "b" => 2,
    "c" => 3,
    "d" => 4,
    "e" => 5,
    "f" => 6,
    "g" => 7,
    "h" => 8,
    "i" => 9,
    "j" => 10,
    "k" => 11,
    "l" => 12,
    "m" => 13,
    "n" => 14,
    "o" => 15,
    "p" => 16,
    "q" => 17,
    "r" => 18,
    "s" => 19,
    "t" => 20,
    "u" => 21,
    "v" => 22,
    "w" => 23,
    "x" => 24,
    "y" => 25,
    "z" => 26,
    "A" => 27,
    "B" => 28,
    "C" => 29,
    "D" => 30,
    "E" => 31,
    "F" => 32,
    "G" => 33,
    "H" => 34,
    "I" => 35,
    "J" => 36,
    "K" => 37,
    "L" => 38,
    "M" => 39,
    "N" => 40,
    "O" => 41,
    "P" => 42,
    "Q" => 43,
    "R" => 44,
    "S" => 45,
    "T" => 46,
    "U" => 47,
    "V" => 48,
    "W" => 49,
    "X" => 50,
    "Y" => 51,
    "Z" => 52,
};

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut total = 0;

    let mut current_line = CurrentLine::One;
    let mut one_items = HashSet::new();
    let mut two_items = HashSet::new();
    let mut three_items = HashSet::new();

    for line in input.lines() {
        line.chars().map(|ch| ch.to_string()).for_each(|ch| {
            match current_line {
                CurrentLine::One => one_items.insert(ch),
                CurrentLine::Two => two_items.insert(ch),
                CurrentLine::Three => three_items.insert(ch),
            };
        });

        if current_line == CurrentLine::Three {
            let common_items = &(&one_items & &two_items) & &three_items;
            let common_item = common_items.iter().next().unwrap();
           
            total += PRIORITIES.get(common_item).unwrap();

            one_items.clear();
            two_items.clear();
            three_items.clear();
        }

        current_line = match current_line {
            CurrentLine::One => CurrentLine::Two,
            CurrentLine::Two => CurrentLine::Three,
            CurrentLine::Three => CurrentLine::One,
        };
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let want = "70";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
