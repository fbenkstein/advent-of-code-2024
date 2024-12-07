use std::io::Read;

use eyre::Result;
use itertools::Itertools;

fn read_input() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

fn concat(a: i64, b: i64) -> i64 {
    a * (10i64).pow(b.max(1).ilog10() + 1) + b
}

#[test]
fn test_concat() {
    macro_rules! assert_concat {
        ($a:expr, $b:expr, $c:expr) => {{
            let actual = concat($a, $b);
            let expected = $c;
            assert!(
                actual == expected,
                "assertion failed: {} == {actual}, expected {expected}",
                stringify!(concat($a, $b))
            );
        }};
    }

    assert_concat!(15, 6, 156);
    assert_concat!(72, 90, 7290);
    assert_concat!(100, 1, 1001);
    assert_concat!(100, 0, 1000);
    assert_concat!(0, 100, 100);
    assert_concat!(0, 0, 0);
}

const OPERATOR_CHOICES: &[(&str, fn(i64, i64) -> i64)] = &[
    ("+", <i64 as std::ops::Add>::add),
    ("*", <i64 as std::ops::Mul>::mul),
    ("||", concat),
];

fn can_produce(operands: &[i64], result: i64) -> bool {
    let operator_count = operands.len() - 1;
    let mut operator_combinations = std::iter::repeat(OPERATOR_CHOICES)
        .take(operator_count)
        .multi_cartesian_product();

    // eprintln!("{result} from {operands:?}?");

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
