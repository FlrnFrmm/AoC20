use anyhow::Result;
use std::io::Read;

fn main() -> Result<()> {
    let _input = parse_input()?;

    println!("Result one: {:?}", p1()); 

    println!("Result two: {:?}", p2());

    Ok(())
}

fn p1() -> Result<usize> {
    Ok(0)
}

fn p2() -> Result<usize> {
    Ok(0)
}

fn parse_input() -> Result<Vec<String>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    Ok(input.lines().map(String::from).collect())
}
