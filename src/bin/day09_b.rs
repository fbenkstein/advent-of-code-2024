use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;

use eyre::Result;
use itertools::Itertools;

fn read_input() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

#[derive(Debug, Clone)]
struct Block {
    id: usize,
    size: u32,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Gap {
    size: u32,
}

const CHARS: [char; 76] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z', '!', '#', '$', '%', '&', '*', '+', '-', '/', ':', ';', '?', '@', '^',
];

fn print_blocks(blocks: &BTreeMap<u32, Block>) {
    let mut blocks = blocks
        .iter()
        // .map(|(k, v)| (k.clone(), v.clone()))
        .peekable();

    print!("blocks: ");

    while let Some((position, block)) = blocks.next() {
        let c = CHARS[block.id % CHARS.len()];

        for _ in 0..block.size {
            print!("{}", c);
        }

        if let Some((next_position, _next_block)) = blocks.peek() {
            let width = usize::try_from(*next_position - position - block.size).expect("overflow");
            print!("{:.<width$}", "");
        }
    }

    println!();
}

fn solve(input: String) -> i64 {
    // println!("input: {}", input);
    // println!();

    let (mut blocks, mut gaps, total_size) = input
        .chars()
        .map(|c| u32::from(c) - u32::from('0'))
        // We expect the input to be odd, beginning and ending in a block. If it is odd then this
        // appends a single empty gap at the end. If that wasn't present the last block would be
        // ignored. We will filter out empty gaps later. If the input is even then this single item
        // will be ignored because it creates a trailing incomplete tuple.
        .chain(std::iter::once(0))
        .tuples()
        .enumerate()
        .fold(
            (BTreeMap::new(), BTreeMap::new(), 0),
            |(mut blocks, mut gaps, mut offset), (id, (block_size, gap_size))| {
                assert_ne!(block_size, 0);
                blocks.insert(
                    offset,
                    Block {
                        id,
                        size: block_size,
                    },
                );
                offset += block_size;
                if gap_size > 0 {
                    gaps.insert(offset, Gap { size: gap_size });
                }
                offset += gap_size;

                (blocks, gaps, offset)
            },
        );

    fn position_and_size((position, block): (&u32, &Block)) -> (u32, u32) {
        (*position, block.size)
    }

    let mut maybe_next_block_position_and_size = blocks.iter().map(position_and_size).next_back();

    {
        let prefix_width = "blocks: ".len();
        let width = total_size as usize;
        // println!("{empty:prefix_width$}{empty:#<width$}", empty = "");

        for k in (0..=total_size.ilog10()).rev() {
            // print!("{:prefix_width$}", "");

            for i in 0..total_size {
                let y = 10u32.pow(k);
                let d = i / y;
                let m = i % y;
                let t = d % 10;

                if m == 0 && (t != 0 || k == 0) {
                    // print!("{}", t);
                } else {
                    // print!(" ");
                }
            }

            // println!();
        }

        // println!("{empty:prefix_width$}{empty:#<width$}", empty = "");
    }

    while let Some((block_position, block_size)) = maybe_next_block_position_and_size {
        // if block_position == 0 || block_position == *blocks.last_entry().unwrap().key() {
        //     print_blocks(&blocks);
        // }
        // println!();
        // println!(
        //     "position: {}, block {:?}",
        //     block_position,
        //     blocks
        //         .get(&block_position)
        //         .map(|block| block.id.to_string().repeat(block.size as usize))
        //         .unwrap(),
        // );

        let Some(gap_position) = gaps
            .range(..block_position)
            .find_map(|(position, gap)| (gap.size >= block_size).then_some(position))
            .cloned()
        else {
            // println!("no gap!");
            // println!();
            maybe_next_block_position_and_size = blocks
                .range(..block_position)
                .map(position_and_size)
                .next_back();
            continue;
        };

        // eprintln!("gap_position: {gap_position}");
        // println!();

        let block = blocks.remove(&block_position).unwrap();
        blocks
            .insert(gap_position, block)
            .map(|_| unreachable!("already a block at this position!?"));

        let gap = gaps.remove(&gap_position).unwrap();
        let new_gap_size = gap.size - block_size;

        if new_gap_size > 0 {
            gaps.insert(gap_position + block_size, Gap { size: new_gap_size });
        }

        maybe_next_block_position_and_size = blocks
            .range(..block_position)
            .map(position_and_size)
            .next_back();
    }

    // print_blocks(&blocks);
    // println!();

    blocks
        .iter()
        .fold(0, |s, (position, block)| {
            s + ((0..block.size).sum::<u32>() as usize + (position * block.size) as usize)
                * block.id
        })
        .try_into()
        .expect("overflow")
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = read_input()?;
    let answer = solve(input);
    println!("result: {answer}");
    Ok(())
}
