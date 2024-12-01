use std::collections::HashMap;
use std::io::Read;

use eyre::Result;

fn read_input() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

fn solve(input: String) -> Result<i64> {
    let (left_values, right_values): (Vec<_>, Vec<_>) = input
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
    let right_value_counts = right_values
        .into_iter()
        .fold(HashMap::new(), |mut counts, value| {
            counts
                .entry(value)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
            counts
        });

    let r = left_values
        .into_iter()
        .map(|left| left * right_value_counts.get(&left).unwrap_or(&0))
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
