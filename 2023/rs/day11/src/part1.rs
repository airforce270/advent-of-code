use crate::custom_error::AocError;

#[derive(Debug)]
struct Coord {
    i: i32,
    j: i32,
}

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch == '#').collect())
        .collect();

    // double empty rows
    let mut empty_rows: Vec<usize> = vec![];
    for i in 0..grid.len() - 1 {
        let row_empty = grid[i].iter().all(|val| !val);
        if row_empty {
            empty_rows.push(i);
        }
    }
    let width = grid.iter().next().unwrap().len();
    for i in empty_rows.iter().rev().copied() {
        grid.insert(i, (0..width).map(|_| false).collect());
    }
    // double empty columns
    let mut empty_columns: Vec<usize> = vec![];
    'col: for j in 0..width - 1 {
        for row in grid.iter() {
            if row[j] {
                continue 'col;
            }
        }
        empty_columns.push(j);
    }
    for row in grid.iter_mut() {
        for j in empty_columns.iter().rev().copied() {
            row.insert(j, false);
        }
    }

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

    let mut distances: Vec<i32> = vec![];
    for i in 0..all_galaxies.len() - 1 {
        for i2 in i + 1..all_galaxies.len() {
            let g1 = &all_galaxies[i];
            let g2 = &all_galaxies[i2];
            let distance = (g1.i - g2.i).abs() + (g1.j - g2.j).abs();
            distances.push(distance);
        }
    }

    Ok(distances.iter().sum::<i32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let want = "374";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
