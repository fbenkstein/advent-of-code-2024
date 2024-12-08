use std::borrow::Borrow;
use std::collections::{hash_map::Entry as HashMapEntry, HashMap, HashSet};
use std::io::Read;

use eyre::Result;
use itertools::Itertools;

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

impl Position {
    fn is_inbounds(&self, d: Dimension) -> bool {
        self.row < d.rows && self.column < d.columns
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct PositionVec {
    d_row: isize,
    d_column: isize,
}

impl TryFrom<Position> for PositionVec {
    type Error = std::num::TryFromIntError;

    fn try_from(p: Position) -> Result<PositionVec, Self::Error> {
        Ok(PositionVec {
            d_row: p.row.try_into()?,
            d_column: p.column.try_into()?,
        })
    }
}

impl std::ops::Neg for PositionVec {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            d_row: -self.d_row,
            d_column: -self.d_column,
        }
    }
}

impl std::ops::Sub for &Position {
    type Output = PositionVec;

    fn sub(self, rhs: &Position) -> PositionVec {
        let d_row = isize::try_from(self.row).unwrap() - isize::try_from(rhs.row).unwrap();
        let d_column = isize::try_from(self.column).unwrap() - isize::try_from(rhs.column).unwrap();
        PositionVec { d_row, d_column }
    }
}

impl std::ops::Sub<PositionVec> for &Position {
    type Output = Option<Position>;

    fn sub(self, rhs: PositionVec) -> Option<Position> {
        self + -rhs
    }
}

impl std::ops::Add<PositionVec> for &Position {
    type Output = Option<Position>;

    fn add(self, rhs: PositionVec) -> Option<Position> {
        Some(Position {
            row: (isize::try_from(self.row).ok()? + rhs.d_row)
                .try_into()
                .ok()?,
            column: (isize::try_from(self.column).ok()? + rhs.d_column)
                .try_into()
                .ok()?,
        })
    }
}

trait Antinodes {
    fn antinodes(&self) -> Vec<Position>;
}

impl<P> Antinodes for (P, P)
where
    P: Borrow<Position>,
{
    fn antinodes(&self) -> Vec<Position> {
        let (p1, p2) = self;
        let (p1, p2) = (p1.borrow(), p2.borrow());
        let mut antinodes = Vec::new();

        let v = p2 - p1;

        if let Some(a1) = p1 - v {
            antinodes.push(a1);
        }

        if let Some(a2) = p2 + v {
            antinodes.push(a2);
        }

        antinodes
    }
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Frequency(char);

impl TryFrom<char> for Frequency {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        if c.is_ascii_alphanumeric() {
            Ok(Self(c))
        } else {
            Err(c)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Location {
    Antenna(Frequency),
    Antinode,
}

#[derive(Debug, Default)]
struct Map {
    dimension: Dimension,
    locations: HashMap<Position, Location>,
    antennas: HashMap<Frequency, HashSet<Position>>,
    antinodes: HashSet<Position>,
}

impl Map {
    fn add_antenna(&mut self, position: Position, f: Frequency) {
        match self.locations.insert(position, Location::Antenna(f)) {
            None => (),
            Some(l) => unreachable!("already an object at position {position:?}: {l:?}"),
        }

        match self.antennas.entry(f).or_default().insert(position) {
            true => (),
            false => unreachable!("duplicate antenna at {position:?}: {f:?}"),
        }
    }

    fn place_antinodes(&mut self) {
        for (_f, positions) in self.antennas.iter() {
            for v in positions.iter().combinations(2) {
                let &[p1, p2] = &v[..] else {
                    unreachable!("{v:?}")
                };

                let antinodes = (p1, p2).antinodes();

                eprintln!("antinodes for {p1:?}, {p2:?}:\n  {antinodes:?}");

                for position in antinodes
                    .into_iter()
                    .filter(|p| p.is_inbounds(self.dimension))
                {
                    self.antinodes.insert(position);

                    match self.locations.entry(position) {
                        HashMapEntry::Vacant(e) => {
                            e.insert(Location::Antinode);
                        }
                        HashMapEntry::Occupied(e) => {
                            // eprintln!("cannot place antinode at {:?}: {:?}", e.key(), e.get());
                            let _ = e;
                        }
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.dimension.rows {
            for column in 0..self.dimension.columns {
                match self.locations.get(&Position { row, column }) {
                    Some(Location::Antenna(Frequency(c))) => {
                        write!(f, "{}", c)?;
                    }
                    Some(Location::Antinode) => {
                        write!(f, "#")?;
                    }
                    None => {
                        write!(f, ".")?;
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn solve(input: String) -> i64 {
    let mut map = Map::default();
    input.lines().enumerate().for_each(|(row, s)| {
        s.chars().enumerate().for_each(|(column, c)| {
            let position = Position { row, column };
            map.dimension.extend(position);
            match Frequency::try_from(c) {
                Ok(f) => map.add_antenna(position, f),
                Err('.') => (),
                Err(c) => unreachable!("unexpected character {c}"),
            }
        })
    });

    dbg!(&map.dimension);

    eprintln!("{map}");

    map.place_antinodes();

    eprintln!();
    eprintln!("{map}");

    map.antinodes.len().try_into().expect("overflow")
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = read_input()?;
    let answer = solve(input);
    println!("{answer}");
    Ok(())
}
