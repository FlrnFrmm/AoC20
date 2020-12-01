use anyhow::Result;
use std::io::Read;

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let expenses = input
        .split_whitespace()
        .map(|v| v.parse::<i32>().expect("Error: Parse input value."))
        .collect::<Vec<i32>>();

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
