use core::fmt;

use crate::custom_error::AocError;

// Partial solution. Works for test cases but website rejects the value it
// produces for the real input. w/e, gave up.

const ASH_CHAR: char = '.';
const ROCK_CHAR: char = '#';

#[derive(Clone, PartialEq)]
enum Tile {
    Ash,
    Rock,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Ash => write!(f, "{}", ASH_CHAR),
            Tile::Rock => write!(f, "{}", ROCK_CHAR),
        }
    }
}

struct Board {
    rows: Vec<Vec<Tile>>,
}

// 0 #...##..#
// 1 #....#..#
// 2 ..##..###
// 3v#####.##.
// 4^#####.##.
// 5 ..##..###
// 6 #....#..#

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{")?;
        for row in &self.rows {
            writeln!(f, "  {:?}", row)?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl Board {
    fn new() -> Board {
        Board { rows: vec![] }
    }

    // index after which reflection happens
    // answer is calculated using this+1
    fn vertical_reflection(&self) -> Option<usize> {
        println!("Evaluating board: {:?}", self);
        let row_count = self.rows.len();
        let row_len = self.rows[0].len();

        let mut evaluating: usize = 0;
        let mut i: usize = 0;
        let mut found = false;

        loop {
            println!("Starting eval at i={i}, evaluating index {evaluating}");
            let at_right = evaluating == row_len - 1;
            let at_bottom = i == row_count - 1;
            if at_right {
                println!("At right. Didn't find a mirror");
                // If any row doesn't have a mirror, there's no mirror
                return None;
            }
            if at_bottom {
                println!("At bottom. Found? {found}: {evaluating}");
                return if found { Some(evaluating) } else { None };
            }

            let size = evaluating + 1;

            let right = self.rows[i].iter().skip(size).take(size);
            let left_if_not_past_middle = self.rows[i].iter().take(size);
            let right_len = right.len();
            let left_if_past_middle = self.rows[i].iter().skip(size - right_len).take(right_len);

            let is_reflection_here = if size > row_len / 2 {
                println!(
                    "Comparing (A) {:?} and {:?}",
                    left_if_past_middle.clone().collect::<Vec<&Tile>>(),
                    right.clone().rev().collect::<Vec<&Tile>>()
                );
                left_if_past_middle.eq(right.rev())
            } else {
                println!(
                    "Comparing (B) {:?} and {:?}",
                    left_if_not_past_middle.clone().collect::<Vec<&Tile>>(),
                    right.clone().rev().collect::<Vec<&Tile>>()
                );
                left_if_not_past_middle.eq(right.rev())
            };

            if !is_reflection_here {
                println!("Not a reflection");
                evaluating += 1;
                if found {
                    println!("Resetting");
                    found = false;
                    i = 0;
                }
            } else {
                println!("Is a reflection");
                i += 1;
                found = true;
            }
        }
    }

    // index after which reflection happens
    // answer is calculated using this+1
    fn horizontal_reflection(&self) -> Option<usize> {
        println!("Doing horizontal reflection:");
        let transposed = Board {
            rows: transpose(&self.rows),
        };
        transposed.vertical_reflection()
    }
}

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let boards = read_boards(input);

    let total: usize = boards
        .iter()
        .map(|board| {
            board
                .vertical_reflection()
                .map(|v| v + 1)
                .or_else(|| board.horizontal_reflection().map(|v| (v + 1) * 100))
                .expect("no reflection")
        })
        .sum();

    Ok(total.to_string())
}

fn read_boards(input: &str) -> Vec<Board> {
    let mut boards: Vec<Board> = vec![];

    let mut current = Board::new();
    for line in input.lines() {
        if line.is_empty() {
            boards.push(current);
            current = Board::new();
            continue;
        }

        let row: Vec<Tile> = line
            .chars()
            .map(|ch| match ch {
                ASH_CHAR => Tile::Ash,
                ROCK_CHAR => Tile::Rock,
                _ => panic!("unknown char: {ch}"),
            })
            .collect();

        current.rows.push(row);
    }
    if current.rows.len() > 0 {
        boards.push(current);
    }

    boards
}

// https://stackoverflow.com/a/64499219
fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let want = "405";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
    #[test]
    fn test_process_v() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let want = "5";

        let board = &read_boards(input)[0];
        let result = board.vertical_reflection().map(|v| (v + 1).to_string());
        match result {
            Some(r) => assert_eq!(want, r),
            None => panic!("None returned"),
        }
    }
    #[test]
    fn test_process_h() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let want = "4";

        let board = &read_boards(input)[0];
        let result = board.horizontal_reflection().map(|v| (v + 1).to_string());
        match result {
            Some(r) => assert_eq!(want, r),
            None => panic!("None returned"),
        }
    }
}
