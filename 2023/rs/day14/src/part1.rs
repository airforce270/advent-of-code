use std::fmt;

use crate::custom_error::AocError;

const EMPTY_CHAR: char = '.';
const IMMOVABLE_ROCK_CHAR: char = '#';
const ROLLING_ROCK_CHAR: char = 'O';

#[derive(Clone, PartialEq)]
enum Tile {
    Empty,
    Immovable,
    Rolling,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, "{}", EMPTY_CHAR),
            Tile::Immovable => write!(f, "{}", IMMOVABLE_ROCK_CHAR),
            Tile::Rolling => write!(f, "{}", ROLLING_ROCK_CHAR),
        }
    }
}

struct Board {
    cols: Vec<Vec<Tile>>,
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in transpose(&self.cols).iter() {
            write!(f, "{:?}\n", row).expect("write succeeds");
        }
        Ok(())
    }
}

impl Board {
    fn new() -> Board {
        Board { cols: vec![] }
    }

    fn tilt(&mut self) {
        let mut tilted_once = true;
        while tilted_once {
            tilted_once = false;

            for col in self.cols.iter_mut() {
                for i in 0..col.len() {
                    let elem = col.get(i).expect("in bounds").clone();
                    let next = match col.get(i + 1) {
                        Option::Some(val) => val.clone(),
                        Option::None => {
                            continue;
                        }
                    };

                    match elem {
                        Tile::Rolling => {
                            continue;
                        }
                        Tile::Immovable => {
                            continue;
                        }
                        Tile::Empty => match next {
                            Tile::Rolling => {
                                tilted_once = true;
                                col.swap(i, i + 1);
                            }
                            Tile::Immovable => {
                                continue;
                            }
                            Tile::Empty => {
                                continue;
                            }
                        },
                    }
                }
            }
        }
    }
}

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut boards = read_boards(input);

    for board in boards.iter_mut() {
        println!("board before tilting:\n{:?}", board);
        board.tilt();
        println!("board after tilting:\n{:?}", board);
    }

    let total: usize = boards
        .iter()
        .map(|board| {
            board
                .cols
                .iter()
                .map(|col| {
                    let mut col_total: usize = 0;
                    for (i, tile) in col.iter().enumerate() {
                        match tile {
                            Tile::Rolling => col_total += col.len() - i,
                            Tile::Immovable => {
                                continue;
                            }
                            Tile::Empty => {
                                continue;
                            }
                        }
                    }
                    col_total
                })
                .sum::<usize>()
        })
        .sum();

    Ok(total.to_string())
}

fn read_boards(input: &str) -> Vec<Board> {
    let mut boards: Vec<Board> = vec![];

    let mut current_rows: Vec<Vec<Tile>> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            let mut board = Board::new();
            board.cols = transpose(&current_rows);
            boards.push(board);
            current_rows = vec![];
            continue;
        }

        let row: Vec<Tile> = line
            .chars()
            .map(|ch| match ch {
                EMPTY_CHAR => Tile::Empty,
                IMMOVABLE_ROCK_CHAR => Tile::Immovable,
                ROLLING_ROCK_CHAR => Tile::Rolling,
                _ => panic!("unknown char: {ch}"),
            })
            .collect();

        current_rows.push(row);
    }
    if current_rows.len() > 0 {
        let mut board = Board::new();
        board.cols = transpose(&current_rows);
        boards.push(board);
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
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let want = "136";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
