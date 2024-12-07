use std::io::Read;

use eyre::Result;
use itertools::Itertools;

fn read_input() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

const OPERATOR_CHOICES: &[(char, fn(i64, i64) -> i64)] = &[
    ('+', <i64 as std::ops::Add>::add),
    ('*', <i64 as std::ops::Mul>::mul),
];

fn can_produce(operands: &[i64], result: i64) -> bool {
    if operands.iter().sum::<i64>() > result || operands.iter().product::<i64>() < result {
        return false;
    }

    let operator_count = operands.len() - 1;
    let mut operator_combinations = (0..(1 << operator_count))
        .map(|chooser| (0..operator_count).map(move |i| OPERATOR_CHOICES[(chooser >> i) & 1]));

    let first_operand = operands[0];
    let operands = &operands[1..];

    operator_combinations.any(|operators| {
        // eprint!("  {first_operand}");
        let combined_result = std::iter::zip(operators, operands).fold(
            first_operand,
            |first_operand, ((operator_symbol, operator), second_operand)| {
                // eprint!(" {operator_symbol} {second_operand}");
                operator(first_operand, *second_operand)
            },
        );
        // eprintln!(
        //     " = {combined_result} {}",
        //     (combined_result == result).then_some('✅').unwrap_or('❌')
        // );
        combined_result == result
    })
}

fn solve(input: String) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split(|c: char| !c.is_ascii_digit())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap())
        })
        .filter_map(|mut row| {
            let result = row.next().unwrap();
            let operands: Vec<_> = row.collect();
            // eprintln!("{result} from {operands:?}?");
            can_produce(&operands, result).then_some(result)
        })
        .sum()
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = read_input()?;
    let answer = solve(input);
    println!("{answer}");
    Ok(())
}
