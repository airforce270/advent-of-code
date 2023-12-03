use crate::custom_error::AocError;
use phf::{phf_set, Set};

static NUM_CHARS: Set<&'static str> = phf_set! {
   "0",
   "1",
   "2",
   "3",
   "4",
   "5",
   "6",
   "7",
   "8",
   "9",
};

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut part_numbers: Vec<u64> = vec![];

    let mut table: Vec<Vec<String>> = vec![];
    for line in input.lines() {
        table.push(line.chars().map(|ch| ch.to_string()).collect());
    }

    let mut i = 0;
    while i < table.len() {
        let row = table.get(i).unwrap();

        let mut current_num: Vec<&str> = vec![];
        let mut current_num_is_part = false;

        let mut j = 0;
        while j < row.len() {
            let ch = row.get(j).unwrap();

            if NUM_CHARS.contains(ch) {
                if has_surrounding_symbol(&table, i, j) {
                    current_num_is_part = true;
                }
                current_num.push(ch);

                if current_num_is_part
                    && !NUM_CHARS.contains(row.get(j + 1).unwrap_or(&".".to_string()))
                {
                    part_numbers.push(current_num.join("").parse().unwrap());
                    current_num.clear();
                    current_num_is_part = false;
                }
            }

            if ch == "." {
                current_num.clear();
                current_num_is_part = false;
            }

            j += 1;
        }
        i += 1;
    }

    Ok(part_numbers.iter().sum::<u64>().to_string())
}

fn has_surrounding_symbol(table: &Vec<Vec<String>>, i: usize, j: usize) -> bool {
    let empty: Vec<String> = vec![];
    let dot = ".".to_string();

    let has_prev_col = j > 0;
    let has_next_col = j < table.get(i).unwrap().len() - 1;
    let has_prev_row = i > 0;
    let has_next_row = i < table.len() - 1;

    let prev_in_row = has_prev_col && is_symbol(table.get(i).unwrap().get(j - 1).unwrap_or(&dot));
    let next_in_row = has_next_col && is_symbol(table.get(i).unwrap().get(j + 1).unwrap_or(&dot));
    let top_left = has_prev_row
        && has_prev_col
        && is_symbol(
            table
                .get(i - 1)
                .unwrap_or(&empty)
                .get(j - 1)
                .unwrap_or(&dot),
        );
    let top_middle =
        has_prev_row && is_symbol(table.get(i - 1).unwrap_or(&empty).get(j).unwrap_or(&dot));
    let top_right = has_prev_row
        && has_next_col
        && is_symbol(
            table
                .get(i - 1)
                .unwrap_or(&empty)
                .get(j + 1)
                .unwrap_or(&dot),
        );
    let bottom_left = has_next_row
        && has_prev_col
        && is_symbol(
            table
                .get(i + 1)
                .unwrap_or(&empty)
                .get(j - 1)
                .unwrap_or(&dot),
        );
    let bottom_middle =
        has_next_row && is_symbol(table.get(i + 1).unwrap_or(&empty).get(j).unwrap_or(&dot));
    let bottom_right = has_next_row
        && has_next_col
        && is_symbol(
            table
                .get(i + 1)
                .unwrap_or(&empty)
                .get(j + 1)
                .unwrap_or(&dot),
        );

    prev_in_row
        || next_in_row
        || top_left
        || top_middle
        || top_right
        || bottom_left
        || bottom_middle
        || bottom_right
}

fn is_symbol(s: &str) -> bool {
    !NUM_CHARS.contains(s) && s != "."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let want = "4361";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
