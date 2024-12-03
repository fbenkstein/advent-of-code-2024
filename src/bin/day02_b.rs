use std::io::Read;

use eyre::Result;

fn read_input() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

fn is_safe<I>(mut row: I) -> bool
where
    I: Iterator<Item = i64>,
{
    #[derive(Debug, PartialEq, Eq)]
    enum Order {
        Ascending,
        Descending,
    }

    #[derive(Debug)]
    enum State {
        Init,
        Second { previous_value: i64 },
        Default { order: Order, previous_value: i64 },
    }

    const SAFE_RANGE: std::ops::RangeInclusive<i64> = 1..=3;

    fn get_order(first: i64, second: i64) -> Result<Order> {
        eyre::ensure!(
            SAFE_RANGE.contains(&(first - second).abs()),
            "out of range: {first} - {second}"
        );
        Ok(if first < second {
            Order::Ascending
        } else {
            Order::Descending
        })
    }

    row.try_fold(State::Init, |state, current_value| match state {
        State::Init => Ok(State::Second {
            previous_value: current_value,
        }),
        State::Second { previous_value } => {
            get_order(previous_value, current_value).map(|order| State::Default {
                order,
                previous_value: current_value,
            })
        }
        State::Default {
            order,
            previous_value,
        } => {
            eyre::ensure!(
                order == get_order(previous_value, current_value)?,
                "different order: {order:?}, {current_value}, {previous_value}"
            );
            Ok(State::Default {
                order,
                previous_value: current_value,
            })
        }
    })
    .is_ok()
}

fn solve(input: String) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse().expect(&format!("cannot parse {s:?}")))
        })
        .filter_map(|row| {
            let row: Vec<i64> = row.collect();

            for delete_index in 0..row.len() {
                let filtered_row = row
                    .iter()
                    .cloned()
                    .enumerate()
                    .filter_map(|(i, value)| (i != delete_index).then_some(value));
                if is_safe(filtered_row) {
                    return Some(());
                }
            }

            None
        })
        .count()
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
