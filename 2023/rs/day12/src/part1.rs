use crate::custom_error::AocError;

#[derive(Clone, Copy, PartialEq)]
enum Spring {
    Working,
    Broke,
    Unknown,
}

impl Spring {
    fn from_char(ch: char) -> Spring {
        match ch {
            '.' => Spring::Working,
            '#' => Spring::Broke,
            '?' => Spring::Unknown,
            _ => panic!("unhandled char: {ch}"),
        }
    }
}

struct Row {
    springs: Vec<Spring>,
    arrangements: Vec<u8>,
}

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let rows: Vec<Row> = input
        .lines()
        .map(|line| Row {
            springs: line
                .split(" ")
                .next()
                .unwrap()
                .chars()
                .map(|ch| Spring::from_char(ch))
                .collect(),
            arrangements: line
                .split(" ")
                .skip(1)
                .next()
                .unwrap()
                .split(",")
                .map(|num| num.parse::<u8>().unwrap())
                .collect(),
        })
        .collect();

    Ok(rows
        .iter()
        .map(|row| count_row(row))
        .sum::<u32>()
        .to_string())
}

fn count_row(row: &Row) -> u32 {
    count(row.springs.iter(), row.arrangements.iter())
}

fn count<'a, 'b, I, I2>(springs: I, arrangements: I2) -> u32
where
    I: Iterator<Item = &'a Spring> + Clone,
    I2: Iterator<Item = &'b u8> + Clone,
{
    // Disclaimer: I didn't understand the problem well, and my solution is
    // heavily based on https://www.youtube.com/watch?v=g3Ms5e7Jdqo
    //
    // This is also terrible Rust code.

    if springs.clone().count() == 0 {
        return match arrangements.count() == 0 {
            true => 1,
            false => 0,
        };
    }
    if arrangements.clone().count() == 0 {
        return match springs.filter(|s| **s == Spring::Broke).count() > 0 {
            true => 0,
            false => 1,
        };
    };

    let first_spring = springs.clone().next().unwrap();
    let spring_count = springs.clone().count();
    let first_arr = *arrangements.clone().next().unwrap();
    let first_arr_usize = <u8 as Into<usize>>::into(first_arr);

    (match first_spring {
        Spring::Working | Spring::Unknown => count(
            springs
                .clone()
                .skip(1)
                .cloned()
                .collect::<Vec<Spring>>()
                .iter(),
            arrangements.clone().cloned().collect::<Vec<u8>>().iter(),
        ),
        Spring::Broke => 0,
    }) + (match first_spring {
        Spring::Broke | Spring::Unknown => {
            if <u8 as Into<usize>>::into(first_arr) <= spring_count
                && springs
                    .clone()
                    .take(first_arr.into())
                    .filter(|sp| **sp == Spring::Working)
                    .peekable()
                    .peek()
                    .is_none()
                && (<u8 as Into<usize>>::into(first_arr) == spring_count
                    || *springs
                        .clone()
                        .skip(first_arr.into())
                        .next()
                        .unwrap_or(&Spring::Working)
                        != Spring::Broke)
            {
                count(
                    springs
                        .clone()
                        .skip(first_arr_usize + 1)
                        .cloned()
                        .collect::<Vec<Spring>>()
                        .iter(),
                    arrangements
                        .clone()
                        .skip(1)
                        .cloned()
                        .collect::<Vec<u8>>()
                        .iter(),
                )
            } else {
                0
            }
        }
        Spring::Working => 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let want = "21";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
