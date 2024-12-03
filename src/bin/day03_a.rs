use regex::Regex;
use std::io::Read;

use eyre::Result;

fn read_input() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

fn solve(input: String) -> i64 {
    let mul_regex = Regex::new(r"mul[(](?P<first>\d+),(?P<second>\d+)[)]").unwrap();
    let mut result = 0;

    for m in mul_regex.captures_iter(&input) {
        let first: i64 = m.name("first").unwrap().as_str().parse().unwrap();
        let second: i64 = m.name("second").unwrap().as_str().parse().unwrap();
        result += first * second;
    }

    result
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = read_input()?;
    let answer = solve(input);
    println!("{answer}");
    Ok(())
}
