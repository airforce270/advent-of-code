use crate::custom_error::AocError;

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

// https://stackoverflow.com/questions/31497422/filter-all-non-integers-from-string-and-yield-vector/31497485#31497485
fn numbers(line: &str) -> Vec<u32> {
    line.chars().filter_map(|ch| ch.to_digit(10)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let want = "142";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
