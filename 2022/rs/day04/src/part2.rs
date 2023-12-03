use crate::custom_error::AocError;

struct Pair {
    low: u32,
    high: u32,
}

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut fully_contains_count = 0;

    for line in input.lines() {
        let pair1 = new_pair(line.split(",").nth(0).unwrap());
        let pair2 = new_pair(line.split(",").nth(1).unwrap());

        let one_contains_two = pair1.high >= pair2.low && pair1.low <= pair2.high;
        let two_contains_one = pair2.high >= pair1.low && pair2.low <= pair1.high;

        if one_contains_two || two_contains_one {
            fully_contains_count += 1;
        }
    }

    Ok(fully_contains_count.to_string())
}

fn new_pair(s: &str) -> Pair {
    Pair {
        low: s.split("-").nth(0).unwrap().parse().unwrap(),
        high: s.split("-").nth(1).unwrap().parse().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let want = "4";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
