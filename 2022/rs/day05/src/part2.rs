use std::collections::VecDeque;

use crate::custom_error::AocError;

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut stacks: Vec<VecDeque<String>> = Vec::new();

    let stack_count = (input.lines().next().unwrap().len() + 1) / 4;
    for _ in 0..stack_count {
        stacks.push(VecDeque::new());
    }

    let mut moving = false;
    'line: for line in input.lines() {
        if line.is_empty() {
            moving = true;
            continue;
        }

        if moving {
            let count: u32 = line
                .split(" from ")
                .nth(0)
                .unwrap()
                .replace("move ", "")
                .parse()
                .unwrap();
            let src_dest_term = line.split(" from ").nth(1).unwrap();
            let src: usize = src_dest_term.split(" to ").nth(0).unwrap().parse().unwrap();
            let dest: usize = src_dest_term.split(" to ").nth(1).unwrap().parse().unwrap();

            let mut items: VecDeque<String> = VecDeque::new();
            for _ in 0..count {
                items.push_front(stacks[src - 1].pop_back().unwrap());
            }
            for item in items.iter() {
                stacks[dest - 1].push_back(item.to_string());
            }
        } else {
            let mut iter = line.chars().skip(1);
            let mut current_stack = 0;
            loop {
                match iter.next() {
                    Some(ch) => {
                        if ch.is_digit(10) {
                            // the 1 2 3 row
                            continue 'line;
                        }

                        if ch != ' ' {
                            stacks[current_stack].push_front(ch.to_string());
                        }
                    }
                    None => break,
                }
                iter.next();
                iter.next();
                iter.next();
                current_stack += 1;
            }
        }
    }

    Ok(stacks
        .iter()
        .map(|stack| (*stack.iter().last().unwrap()).to_owned())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let want = "MCD";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
