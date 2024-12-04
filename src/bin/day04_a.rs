use std::io::Read;
use std::ops::{Deref, Index};

use eyre::Result;

fn get_char<R, S>(rows: &R, row: usize, column: usize) -> char
where
    R: Index<usize, Output = S>,
    S: Deref<Target = str>,
{
    debug_assert!(rows[row].is_ascii());
    rows[row].as_bytes()[column].into()
}

fn read_input() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

fn get_rows(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn get_columns(input: &str) -> Vec<String> {
    let rows = get_rows(input);
    let row_count = rows.len();
    let column_count = rows[0].len();

    (0..column_count)
        .map(|column| {
            (0..row_count)
                .map(|row| get_char(&rows, row, column))
                .collect()
        })
        .collect()
}

fn get_first_diagonals(input: &str) -> Vec<String> {
    let rows = get_rows(input);
    let row_count = rows.len();
    let column_count = rows[0].len();

    let start_points = (0..column_count)
        .into_iter()
        .map(|column| (0, column))
        .chain((1..row_count).into_iter().map(|row| (row, 0)));

    start_points
        .map(|(start_row, start_column)| {
            (0..)
                .into_iter()
                .take_while(|i| start_row + i < row_count && start_column + i < column_count)
                .map(|i| (start_row + i, start_column + i))
                .map(|(row, column)| get_char(&rows, row, column))
                .collect()
        })
        .collect()
}

fn get_second_diagonals(input: &str) -> Vec<String> {
    let rows = get_rows(input);
    let row_count = rows.len();
    let column_count = rows[0].len();

    let start_points = (0..column_count)
        .into_iter()
        .map(|column| (0, column))
        .chain(
            (1..row_count)
                .into_iter()
                .map(|row| (row, column_count - 1)),
        );

    start_points
        .map(|(start_row, start_column)| {
            (0..)
                .into_iter()
                .take_while(|i| start_row + i < row_count && start_column >= *i)
                .map(|i| (start_row + i, start_column - i))
                .map(|(row, column)| get_char(&rows, row, column))
                .collect()
        })
        .collect()
}

fn solve(input: String) -> i64 {
    let search_word = "XMAS".to_string();
    let reverse_search_word = {
        let mut v = search_word.as_bytes().to_vec();
        v.reverse();
        String::from_utf8(v).unwrap()
    };

    let line_getters = [
        get_rows,
        get_columns,
        get_first_diagonals,
        get_second_diagonals,
    ];

    eprintln!("input:\n{input}\n");

    line_getters
        .into_iter()
        .inspect(|f| match f {
            f if *f == get_rows => eprintln!("rows"),
            f if *f == get_columns => eprintln!("columns"),
            f if *f == get_first_diagonals => eprintln!("1st diagonals"),
            f if *f == get_second_diagonals => eprintln!("2nd diagonals"),
            f => unreachable!("unknown function {f:?}"),
        })
        .map(|f| f(&input))
        .map(|v| v.into_iter())
        .flatten()
        .inspect(|line| eprint!("{line}"))
        .map(|line| {
            (
                line.matches(&search_word).count(),
                line.matches(&reverse_search_word).count(),
            )
        })
        .inspect(|(count, reverse_count)| eprintln!(" {count} {reverse_count}"))
        .map(|(count, reverse_count)| count + reverse_count)
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
