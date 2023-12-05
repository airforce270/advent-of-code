use std::collections::{HashSet, VecDeque};

use crate::custom_error::AocError;

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut count = 0;

    let mut buf = VecDeque::new();
    for ch in input.chars() {
        let current_chars_set: HashSet<char> = HashSet::from_iter(buf.iter().cloned());
        if buf.len() == 4 && current_chars_set.len() == 4 {
            // success
            break;
        }

        buf.push_back(ch);
        if buf.len() > 4 {
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
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let want = "5";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_process2() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let want = "6";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_process3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let want = "10";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_process4() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let want = "11";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
