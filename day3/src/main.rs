use anyhow::Result;
use std::io::Read;

fn main() -> Result<()> {
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let map = parse_input()?;

    let result: usize = slopes.iter().map(|s| calculate_collisions(s, &map)).product();
    println!("{}", result);

    Ok(())
}

fn calculate_collisions(slope: &(usize, usize), map: &Vec<String>) -> usize {
        let (mut x, mut y, mut count) = (0usize, 0usize, 0usize);

        while y < map.len() {
            let row = &map[y];
            let part: String = row.chars().take(x % row.len() + 1).collect();

            if let Some(c) = part.chars().last() {
                if c == '#' {
                    count += 1;
                }
            }

            x += slope.0;
            y += slope.1;
        }

        count
}

fn parse_input() -> Result<Vec<String>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    Ok(input.lines().map(|s| String::from(s)).collect())
}
