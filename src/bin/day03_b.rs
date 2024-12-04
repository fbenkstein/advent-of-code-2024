use regex::Regex;
use std::io::Read;

use eyre::Result;

fn read_input() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

fn solve(input: String) -> i64 {
    let mul_regex =
        Regex::new(r"(?P<off>don't)|(?P<on>do)|(?P<op>mul[(](?P<first>\d+),(?P<second>\d+)[)])")
            .unwrap();
    let mut result = 0;
    let mut on_off = true;

    for m in mul_regex.captures_iter(&input) {
        if m.name("on").is_some() {
            on_off = true;
        } else if m.name("off").is_some() {
            on_off = false;
        } else if on_off && m.name("op").is_some() {
            let first: i64 = m.name("first").unwrap().as_str().parse().unwrap();
            let second: i64 = m.name("second").unwrap().as_str().parse().unwrap();
            result += first * second;
        }
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
