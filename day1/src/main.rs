use anyhow::Result;
use std::io::Read;

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let expenses = parse_input()?;

    for i in 0..expenses.len() {
        for j in i..expenses.len() {
            for k in j..expenses.len() {
                if expenses[i] + expenses[j] + expenses[k] == 2020 {
                    let result = expenses[i] * expenses[j] * expenses[k];
                    println!("{}", result)
                }
            }
        }
    }

    Ok(())
}

fn parse_input() -> Result<Vec<i32>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    input.split_whitespace()
        .map(|v| v.parse::<i32>().map_err(|err| anyhow::Error::new(err)))
        .collect()
}
