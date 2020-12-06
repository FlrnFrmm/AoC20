use anyhow::Result;
use std::io::Read;

fn main() -> Result<()> {
    let groups = parse_input()?;

    println!("Result one: {}", calculate_all_yes(&groups));

    println!("Result two: {}", calculate_all_yes_for_whole_group(&groups));

    Ok(())
}

fn calculate_all_yes(groups: &Vec<String>) -> usize {
    groups.iter()
        .map(|s| {
            let mut r: Vec<char> = s.chars()
                .filter(|c| c.is_ascii_alphabetic())
                .collect();
            
            r.sort();
            r.dedup();

            r.len()    
        })
        .sum()
}

fn calculate_all_yes_for_whole_group(groups: &Vec<String>) -> usize {
    groups.iter()
        .map(|s| {
            let group_members_count = s.lines().count();

            let mut r: Vec<char> = s.chars()
                .filter(|c| c.is_ascii_alphabetic())
                .filter(|c| {
                    s.chars().filter(|cc| c == cc).count() == group_members_count
                })
                .collect();
            
            r.sort();
            r.dedup();

            r.len()    
        })
        .sum()
}

fn parse_input() -> Result<Vec<String>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    Ok(input.split("\n\n").map(|s| String::from(s)).collect())
}
