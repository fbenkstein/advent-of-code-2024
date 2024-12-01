use std::io::Read;

use eyre::Result;

fn read_input() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

fn solve(input: String) -> Result<i64> {
    let (mut left_values, mut right_values): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let values: Vec<i64> = line
                .split_ascii_whitespace()
                .map(|s| s.parse().expect(&format!("cannot parse {s:?}")))
                .collect();
            match &values[..] {
                &[left, right] => (left, right),
                _ => panic!("unexpected values: {values:?}"),
            }
        })
        .unzip();
    left_values.sort();
    right_values.sort();
    let r = std::iter::zip(left_values, right_values)
        .map(|(left, right)| (left - right).abs())
        .sum();
    Ok(r)
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = read_input()?;
    let answer = solve(input)?;
    println!("{answer}");
    Ok(())
}
