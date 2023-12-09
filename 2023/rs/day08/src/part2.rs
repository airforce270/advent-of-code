use crate::custom_error::AocError;
use num::Integer;
use std::collections::HashMap;

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

    let mut all_steps: Vec<u64> = vec![];
    for starting_position in paths.keys().filter(|k| k.ends_with("A")) {
        let mut steps = 0;
        let mut current: &str = &starting_position;
        for instruction in input.lines().next().unwrap().chars().cycle() {
            if current.ends_with("Z") {
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
        all_steps.push(steps);
    }

    let mut nums_lcm = 0;
    for step in all_steps.iter() {
        if nums_lcm == 0 {
            nums_lcm = *step;
        }
        nums_lcm = Integer::lcm(&nums_lcm, step);
    }

    Ok(nums_lcm.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let want = "6";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
