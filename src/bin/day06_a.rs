use std::collections::HashSet;
use std::io::Read;

use eyre::Result;

fn read_input() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(Debug, Default, Clone, Copy)]
struct Dimension {
    rows: usize,
    columns: usize,
}

impl Dimension {
    fn extend(&mut self, Position { row, column }: Position) {
        self.rows = self.rows.max(row + 1);
        self.columns = self.rows.max(column + 1);
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = char;

    fn try_from(c: char) -> Result<Self, char> {
        match c {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(c),
        }
    }
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        }
    }
}

#[derive(Debug, Clone)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn step(&mut self, map: &Map) -> Option<()> {
        let old_position = self.position.clone();

        match self.direction {
            Direction::Up => self.position.row = self.position.row.checked_sub(1)?,
            Direction::Down => {
                self.position.row =
                    Some(self.position.row + 1).filter(|row| *row < map.dimension.rows)?
            }
            Direction::Left => self.position.column = self.position.column.checked_sub(1)?,
            Direction::Right => {
                self.position.column = Some(self.position.column + 1)
                    .filter(|column| *column < map.dimension.columns)?
            }
        };

        if map.obstacles.contains(&self.position) {
            self.position = old_position;
            self.direction = self.direction.turn_right();
        }

        Some(())
    }
}

#[derive(Debug, Default)]
struct Map {
    dimension: Dimension,
    obstacles: HashSet<Position>,
}

fn parse_input(input: &str) -> (Map, Guard) {
    match input.lines().enumerate().fold(
        (Map::default(), None),
        |(mut map, mut maybe_guard), (row, line)| {
            line.chars().enumerate().for_each(|(column, c)| {
                let position = Position { row, column };
                map.dimension.extend(position);
                match Direction::try_from(c) {
                    Ok(direction) => {
                        let old_guard = maybe_guard.replace(Guard {
                            position,
                            direction,
                        });
                        assert!(
                            old_guard.is_none(),
                            "there already was a guard: {old_guard:?}"
                        );
                    }
                    Err('#') => {
                        map.obstacles.insert(position);
                    }
                    Err('.') => (),
                    Err(c) => {
                        panic!("unexpected character at {position:?}: {c:?}");
                    }
                }
            });
            (map, maybe_guard)
        },
    ) {
        (map, Some(guard)) => (map, guard),
        (_, None) => panic!("there was no guard!"),
    }
}

fn solve(input: String) -> i64 {
    let (map, mut guard) = parse_input(&input);
    let mut visited = HashSet::new();
    visited.insert(guard.position);

    std::iter::from_fn(|| {
        guard.step(&map)?;
        Some(guard.clone())
    })
    .for_each(|guard| {
        visited.insert(guard.position);
    });

    visited.len().try_into().expect("overflow")
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = read_input()?;
    let answer = solve(input);
    println!("{answer}");
    Ok(())
}
