use std::collections::HashSet;
use std::io::Read;

use eyre::Result;
use itertools::Itertools;

fn read_input() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Down,
    Right,
    Up,
    Left,
}

impl Direction {
    fn first() -> Self {
        Self::Down
    }

    fn next(self) -> Option<Self> {
        match self {
            Self::Down => Some(Self::Right),
            Self::Right => Some(Self::Up),
            Self::Up => Some(Self::Left),
            Self::Left => None,
        }
    }

    fn go(self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        Some(match self {
            Self::Down => (x, y + 1),
            Self::Right => (x + 1, y),
            Self::Up => (x, y.checked_sub(1)?),
            Self::Left => (x.checked_sub(1)?, y),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct InformedDirection {
    direction: Direction,
    width: usize,
    height: usize,
}

impl InformedDirection {
    fn start((width, height): (usize, usize)) -> Self {
        Self {
            direction: Direction::first(),
            width,
            height,
        }
    }

    fn next(self) -> Option<Self> {
        self.direction.next().map(|direction| Self {
            direction,
            width: self.width,
            height: self.height,
        })
    }

    fn reset(self) -> Self {
        Self::start((self.width, self.height))
    }

    fn go(self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        self.direction
            .go((x, y))
            .filter(|(x, y)| *x < self.width && *y < self.height)
    }
}

#[derive(Debug, Clone, Copy)]
struct PathNode {
    direction: InformedDirection,
    x: usize,
    y: usize,
}

impl PathNode {
    fn start(map: &Vec<Vec<u32>>, (x, y): (usize, usize)) -> Self {
        let width = map.first().map(|row| row.len()).unwrap_or(0);
        let height = map.len();
        let direction = InformedDirection::start((width, height));
        PathNode { direction, x, y }
    }

    fn next(self) -> Option<PathNode> {
        Some(Self {
            direction: self.direction.next()?,
            x: self.x,
            y: self.y,
        })
    }

    fn go(self) -> Option<Self> {
        self.direction.go((self.x, self.y)).map(|(x, y)| Self {
            direction: self.direction.reset(),
            x,
            y,
        })
    }
}

fn check_node(map: &Vec<Vec<u32>>, index: usize, node: PathNode) -> bool {
    usize::try_from(map[node.y][node.x]).unwrap() == index
}

fn count_paths(map: &Vec<Vec<u32>>, (x, y): (usize, usize)) -> usize {
    let mut peaks = HashSet::new();
    let mut path = vec![PathNode::start(map, (x, y))];

    while let Some(last_node) = path.last().copied() {
        // eprint!(
        //     "path: {:?}",
        //     path.iter()
        //         .map(|node| (node.x, node.y, map[node.y][node.x].to_string()))
        //         .collect::<Vec<_>>()
        // );
        // eprintln!(" {:?}", last_node.direction.direction);

        if let Some(next_node) = last_node.go() {
            if check_node(map, path.len(), next_node) {
                path.push(next_node);

                if path.len() < 10 {
                    continue;
                }
            }
        }

        if path.len() == 10 {
            // eprintln!(
            //     "good path: {:?}\n",
            //     path.iter()
            //         .map(|node| (node.x, node.y, map[node.y][node.x].to_string()))
            //         .collect::<Vec<_>>()
            // );
            let peak_node = path.pop().unwrap();
            peaks.insert((peak_node.x, peak_node.y));
        }

        while let Some(last_node) = path.pop() {
            if let Some(updated_node) = last_node.next() {
                path.push(updated_node);
                break;
            }
        }
    }

    peaks.len()
}

fn solve(input: String) -> usize {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or(u32::MAX))
                .collect()
        })
        .collect();

    let trail_heads: Vec<_> = map
        .iter()
        .inspect(|row| {
            assert!(
                Some(row.len()) == map.first().map(|first_row| first_row.len()),
                "not all rows have the same length"
            )
        })
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, h)| (*h == 0).then_some((x, y)))
        })
        .collect();

    eprintln!("trail_heads: {} {trail_heads:?}", trail_heads.len());

    trail_heads
        .into_iter()
        .fold(0, |s, t| s + count_paths(&map, t))
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = read_input()?;
    let answer = solve(input);
    println!("result: {answer}");
    Ok(())
}
