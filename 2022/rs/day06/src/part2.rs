use std::collections::{HashSet, VecDeque};

use crate::custom_error::AocError;

const CHARS: usize = 14;

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut count = 0;

    let mut buf = VecDeque::new();
    for ch in input.chars() {
        let current_chars_set: HashSet<char> = HashSet::from_iter(buf.iter().cloned());
        if buf.len() == CHARS && current_chars_set.len() == CHARS {
            // success
            break;
        }

        buf.push_back(ch);
        if buf.len() > CHARS {
            buf.pop_front();
        }

        count += 1;
    }

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let want = "19";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_process2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let want = "23";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_process3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let want = "23";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_process4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let want = "29";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_process5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let want = "26";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
