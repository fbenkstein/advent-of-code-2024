use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;

use eyre::Result;

fn read_input() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

fn is_valid(rules: &HashMap<i64, HashSet<i64>>, update: &[i64]) -> bool {
    update.iter().enumerate().skip(1).all(|(i, x)| {
        let are_before = &update[..i];
        if let Some(must_after) = rules.get(x) {
            are_before.iter().all(|y| !must_after.contains(y))
        } else {
            true
        }
    })
}

fn solve(input: String) -> i64 {
    let lines: Vec<_> = input.lines().collect();
    let divider_index = lines
        .iter()
        .position(|s| s.is_empty())
        .expect("divider not found");
    let rules: Vec<(i64, i64)> = lines[..divider_index]
        .iter()
        .map(|s| {
            let (a, b) = s.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    let rules_map: HashMap<i64, HashSet<i64>> =
        rules.iter().fold(HashMap::new(), |mut m, (a, b)| {
            m.entry(*a).or_default().insert(*b);
            m
        });
    let updates: Vec<Vec<i64>> = lines[divider_index + 1..]
        .iter()
        .map(|s| s.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();
    updates
        .iter()
        .filter(|update| is_valid(&rules_map, &update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = read_input()?;
    let answer = solve(input);
    println!("{answer}");
    Ok(())
}
