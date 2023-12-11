use crate::custom_error::AocError;

#[derive(Debug)]
struct Coord {
    i: i64,
    j: i64,
}

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    process_impl(input, 1_000_000)
}

fn process_impl(input: &str, factor: i64) -> std::result::Result<String, AocError> {
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch == '#').collect())
        .collect();

    let mut all_galaxies: Vec<Coord> = vec![];
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell {
                all_galaxies.push(Coord {
                    i: i.try_into().unwrap(),
                    j: j.try_into().unwrap(),
                });
            }
        }
    }

    // multiply empty rows
    let mut empty_rows: Vec<i64> = vec![];
    for i in 0..grid.len() - 1 {
        let row_empty = grid[i].iter().all(|val| !val);
        if row_empty {
            empty_rows.push(i.try_into().unwrap());
        }
    }
    let width = grid.iter().next().unwrap().len();
    for i in empty_rows.iter().rev().copied() {
        for galaxy in all_galaxies.iter_mut() {
            if i > galaxy.i {
                continue;
            }
            galaxy.i += factor - 1;
        }
    }
    // multiply empty columns
    let mut empty_columns: Vec<i64> = vec![];
    'col: for j in 0..width - 1 {
        for row in grid.iter() {
            if row.len() == 0 {
                continue;
            }
            if row[j] {
                continue 'col;
            }
        }
        empty_columns.push(j.try_into().unwrap());
    }
    for j in empty_columns.iter().rev().copied() {
        for galaxy in all_galaxies.iter_mut() {
            if j > galaxy.j {
                continue;
            }
            galaxy.j += factor - 1;
        }
    }

    let mut distances: Vec<i64> = vec![];
    for i in 0..all_galaxies.len() - 1 {
        for i2 in i + 1..all_galaxies.len() {
            let g1 = &all_galaxies[i];
            let g2 = &all_galaxies[i2];
            let distance = (g1.i - g2.i).abs() + (g1.j - g2.j).abs();
            distances.push(distance);
        }
    }

    Ok(distances.iter().sum::<i64>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../input.txt");
        let want = "9799681";
        match process_impl(input, 2) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
