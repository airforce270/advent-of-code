use std::collections::HashMap;

use crate::custom_error::AocError;

struct Options {
    left: String,
    right: String,
}

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut paths = HashMap::new();

    for line in input.lines().skip(2) {
        let name = line.split(" = (").next().unwrap();
        let options = line.split(" = (").skip(1).next().unwrap().replace(")", "");
        let left = options.split(", ").next().unwrap().to_string();
        let right = options.split(", ").skip(1).next().unwrap().to_string();
        paths.insert(name, Options { left, right });
    }

    let mut current = "AAA";
    let mut steps = 0;
    for instruction in input.lines().next().unwrap().chars().cycle() {
        if current == "ZZZ" {
            break;
        }

        current = match instruction {
            'L' => &paths[&current].left,
            'R' => &paths[&current].right,
            _ => {
                return Err(AocError {
                    details: format!("Unknown instruction: {instruction}"),
                })
            }
        };

        steps += 1;
    }

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let want = "2";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_process_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let want = "6";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
