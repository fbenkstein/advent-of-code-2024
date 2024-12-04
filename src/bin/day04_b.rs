use std::io::Read;
use std::ops::{Deref, Index};

use eyre::Result;

fn read_input() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

fn get_char<R, S>(rows: &R, row: usize, column: usize) -> char
where
    R: Index<usize, Output = S> + ?Sized,
    S: Deref<Target = str>,
{
    debug_assert!(rows[row].is_ascii());
    rows[row].as_bytes()[column].into()
}

const PATTERN_TEMPLATE: &str = "
    M.S
    .A.
    M.S
";

fn rotate_pattern(pattern: &[String]) -> Vec<String> {
    (0..pattern.len())
        .map(|row| {
            (0..pattern[row].len())
                .map(move |column| get_char(pattern, pattern.len() - 1 - column, row))
                .collect()
        })
        .collect()
}

fn get_patterns() -> Vec<Vec<String>> {
    let initial_pattern: Vec<String> = PATTERN_TEMPLATE
        .trim()
        .lines()
        .map(|line| line.trim().to_string())
        .collect();
    let mut patterns = vec![initial_pattern];
    patterns.push(rotate_pattern(patterns.last().unwrap()));
    patterns.push(rotate_pattern(patterns.last().unwrap()));
    patterns.push(rotate_pattern(patterns.last().unwrap()));
    patterns
}

fn does_pattern_match(rows: &[String], (row, column): (usize, usize), pattern: &[String]) -> bool {
    (0..pattern.len())
        .flat_map(|pattern_row| {
            (0..pattern[pattern_row].len()).map(move |pattern_column| (pattern_row, pattern_column))
        })
        .all(|(pattern_row, pattern_column)| {
            let pattern_char = get_char(pattern, pattern_row, pattern_column);

            pattern_char == '.'
                || rows
                    .get(row + pattern_row)
                    .and_then(|line| line.as_bytes().get(column + pattern_column))
                    .map(|b| b.clone().into())
                    == Some(pattern_char)
        })
}

fn find_pattern_matches<'a>(
    rows: &'a [String],
    pattern: &'a [String],
) -> impl Iterator<Item = (usize, usize)> + 'a {
    (0..rows.len())
        .flat_map(|row| (0..rows[row].len()).map(move |column| (row, column)))
        .filter(move |(row, column)| does_pattern_match(rows, (*row, *column), pattern))
}

fn get_rows(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn solve(input: String) -> i64 {
    let patterns = get_patterns();
    let rows = get_rows(&input);

    patterns
        .into_iter()
        .map(|pattern| find_pattern_matches(&rows, &pattern).count())
        .sum::<usize>()
        .try_into()
        .expect("overflow")
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = read_input()?;
    let answer = solve(input);
    println!("{answer}");
    Ok(())
}
