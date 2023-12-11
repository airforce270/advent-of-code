use phf::phf_map;

use crate::custom_error::AocError;

// big thanks to /u/3saster for pointers to which formulas to use:
// https://www.reddit.com/r/adventofcode/comments/18f1sgh/comment/kcugm6t/

struct Pipe {
    kind: PipeKind,
    distance: usize,
    source: Option<Direction>,
    i: usize,
    j: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PipeKind {
    NorthSouth,
    WestEast,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Starting,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, Debug)]
struct Coord {
    i: usize,
    j: usize,
}

static PIPE_CHARS: phf::Map<char, PipeKind> = phf_map! {
    '|' => PipeKind::NorthSouth,
    '-' => PipeKind::WestEast,
    'L' => PipeKind::NorthEast,
    'J' => PipeKind::NorthWest,
    '7' => PipeKind::SouthWest,
    'F' => PipeKind::SouthEast,
    '.' => PipeKind::Ground,
    'S' => PipeKind::Starting,
};

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut board: Vec<Vec<Pipe>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, ch)| Pipe {
                    kind: PIPE_CHARS[&ch],
                    distance: 0,
                    source: None,
                    i,
                    j,
                })
                .collect()
        })
        .collect();
    let width = board.iter().next().unwrap().len();
    let height = board.len();

    let starting = board
        .iter()
        .flatten()
        .filter(|p| p.kind == PipeKind::Starting)
        .next()
        .unwrap();

    let mut all_coords: Vec<Coord> = vec![];

    let (mut i, mut j) = (starting.i, starting.j);

    all_coords.push(Coord { i, j });

    let mut source;
    let mut distance: usize = 1;

    if starting.i > 0 && board[i - 1][j].kind.is_navigable_from_south() {
        let next = &mut board[i - 1][j];
        source = Some(Direction::South);
        next.source = source;
        next.distance = distance;
        (i, j) = (next.i, next.j);
    } else if starting.j < width && board[i][j + 1].kind.is_navigable_from_west() {
        let next = &mut board[i][j + 1];
        source = Some(Direction::West);
        next.source = source;
        next.distance = distance;
        (i, j) = (next.i, next.j);
    } else if starting.i < height && board[i + 1][j].kind.is_navigable_from_north() {
        let next = &mut board[i + 1][j];
        source = Some(Direction::North);
        next.source = source;
        next.distance = distance;
        (i, j) = (next.i, next.j);
    } else if starting.j > 0 && board[i][j - 1].kind.is_navigable_from_east() {
        let next = &mut board[i][j - 1];
        source = Some(Direction::East);
        next.source = source;
        next.distance = distance;
        (i, j) = (next.i, next.j);
    } else {
        return Err(AocError {
            details: format!("nowhere to go from starting"),
        });
    }

    loop {
        let curr = &board[i][j];
        if curr.kind == PipeKind::Starting {
            break;
        }

        all_coords.push(Coord { i, j });

        let src;
        (i, j, src) = curr.select_next_coords();
        source = Some(src);

        let next = &mut board[i][j];
        next.source = source;
        next.distance = distance;

        distance += 1;
    }

    all_coords.push(all_coords.first().unwrap().clone());

    // using the shoelace formula to calculate area
    let mut area: i32 = 0;
    let mut prev: Option<Coord> = None;
    for coord in all_coords.iter() {
        match prev {
            Some(prev_coord) => {
                let first: i32 = (prev_coord.j * coord.i).try_into().unwrap();
                let second: i32 = (prev_coord.i * coord.j).try_into().unwrap();
                area += first - second;
            }
            None => {}
        }
        prev = Some(coord.to_owned());
    }
    area = (area / 2).abs();

    // using Pick's formula
    // i = A - b/2 - h + 1
    // A = area
    // b = distance
    // h = 0, assume no holes
    let enclosed = area - TryInto::<i32>::try_into(distance / 2).unwrap() - 0 + 1;

    Ok(enclosed.to_string())
}

impl Pipe {
    fn select_next_coords(&self) -> (usize, usize, Direction) {
        match &self.source {
            Some(source) => match source {
                Direction::North => match self.kind {
                    PipeKind::NorthEast => (self.i, self.j + 1, Direction::West),
                    PipeKind::NorthWest => (self.i, self.j - 1, Direction::East),
                    PipeKind::NorthSouth => (self.i + 1, self.j, Direction::North),
                    _ => panic!(
                        "coming from north but wrong type: {} {} {:?}",
                        self.i, self.j, self.kind
                    ),
                },
                Direction::South => match self.kind {
                    PipeKind::SouthWest => (self.i, self.j - 1, Direction::East),
                    PipeKind::SouthEast => (self.i, self.j + 1, Direction::West),
                    PipeKind::NorthSouth => (self.i - 1, self.j, Direction::South),
                    _ => panic!(
                        "coming from south but wrong type: {} {} {:?}",
                        self.i, self.j, self.kind
                    ),
                },
                Direction::East => match self.kind {
                    PipeKind::NorthEast => (self.i - 1, self.j, Direction::South),
                    PipeKind::SouthEast => (self.i + 1, self.j, Direction::North),
                    PipeKind::WestEast => (self.i, self.j - 1, Direction::East),
                    _ => panic!(
                        "coming from east but wrong type: {} {} {:?}",
                        self.i, self.j, self.kind
                    ),
                },
                Direction::West => match self.kind {
                    PipeKind::NorthWest => (self.i - 1, self.j, Direction::South),
                    PipeKind::SouthWest => (self.i + 1, self.j, Direction::North),
                    PipeKind::WestEast => (self.i, self.j + 1, Direction::West),
                    _ => panic!(
                        "coming from west but wrong type: {} {} {:?}",
                        self.i, self.j, self.kind
                    ),
                },
            },
            None => panic!(
                "tried to select next coords without source destination [{}, {}]",
                self.i, self.j
            ),
        }
    }
}

impl PipeKind {
    fn is_navigable_from_west(self) -> bool {
        self.is_navigable()
            && (self == PipeKind::NorthWest
                || self == PipeKind::SouthWest
                || self == PipeKind::WestEast)
    }

    fn is_navigable_from_east(self) -> bool {
        self.is_navigable()
            && (self == PipeKind::NorthEast
                || self == PipeKind::SouthEast
                || self == PipeKind::WestEast)
    }

    fn is_navigable_from_north(self) -> bool {
        self.is_navigable()
            && (self == PipeKind::NorthWest
                || self == PipeKind::NorthEast
                || self == PipeKind::NorthSouth)
    }

    fn is_navigable_from_south(self) -> bool {
        self.is_navigable()
            && (self == PipeKind::SouthWest
                || self == PipeKind::SouthEast
                || self == PipeKind::NorthSouth)
    }

    fn is_navigable(self) -> bool {
        self != PipeKind::Ground && self != PipeKind::Starting
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let want = "10";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_process_2() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let want = "4";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
