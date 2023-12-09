use crate::custom_error::AocError;

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let histories: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.split(" ").map(|num| num.parse().unwrap()).collect())
        .collect();

    let mut predictions: Vec<i32> = vec![];
    for history in histories.iter() {
        let mut diffs: Vec<Vec<i32>> = vec![];
        diffs.push(history.to_vec());

        loop {
            let mut diff: Vec<i32> = vec![];
            let mut prev: Option<i32> = None;
            for measurement in diffs.last().unwrap().iter() {
                if let Some(p) = prev {
                    diff.push(*measurement - p);
                }
                prev = Some(*measurement);
            }
            if diff.iter().all(|item| *item == 0) {
                break;
            } else {
                diffs.push(diff);
            }
        }

        predictions.push(
            diffs
                .iter()
                .rev()
                .fold(0, |acc, diff| diff.first().unwrap() - acc),
        );
    }

    Ok(predictions.iter().sum::<i32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let want = "2";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
