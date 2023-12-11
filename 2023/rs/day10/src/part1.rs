use phf::phf_map;

use crate::custom_error::AocError;

struct Pipe {
    kind: PipeKind,
    distance: u32,
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

    let (mut i, mut j) = (starting.i, starting.j);

    let mut source;
    let mut distance = 1;

    if starting.i > 0 && board[i - 1][j].kind.is_navigable_from_north() {
        let next = &mut board[i - 1][j];
        source = Some(Direction::North);
        next.source = source;
        next.distance = distance;
        (i, j) = (next.i, next.j);
    } else if starting.j < width && board[i][j + 1].kind.is_navigable_from_west() {
        let next = &mut board[i][j + 1];
        source = Some(Direction::West);
        next.source = source;
        next.distance = distance;
        (i, j) = (next.i, next.j);
    } else if starting.i < height && board[i + 1][j].kind.is_navigable_from_south() {
        let next = &mut board[i + 1][j];
        source = Some(Direction::South);
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

        let src;
        (i, j, src) = curr.select_next_coords();
        source = Some(src);

        let next = &mut board[i][j];
        next.source = source;
        next.distance = distance;

        distance += 1;
    }

    Ok((distance / 2).to_string())
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
            && (self == PipeKind::SouthWest
                || self == PipeKind::SouthEast
                || self == PipeKind::NorthSouth)
    }

    fn is_navigable_from_south(self) -> bool {
        self.is_navigable()
            && (self == PipeKind::NorthWest
                || self == PipeKind::NorthEast
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
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let want = "8";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
