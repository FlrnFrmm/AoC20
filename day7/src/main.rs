use anyhow::Result;
use std::collections::HashMap;
use std::io::Read;

type Rules = HashMap<String, Vec<(usize, String)>>;

fn main() -> Result<()> {
    let rules = parse_input()?;
    let target_bag = "shiny gold".to_string();

    println!(
        "Result one: {}",
        count_bags_that_can_hold_target_bag(&target_bag, &rules)
    );

    println!("Result two: {}", count_total_bags(&target_bag, &rules));

    Ok(())
}

fn count_bags_that_can_hold_target_bag(target_bag: &String, rules: &Rules) -> usize {
    rules.iter().fold(0, |acc, rule| {
        if can_hold_target_bag(&target_bag, rule.0, rules) {
            acc + 1
        } else {
            acc
        }
    })
}

fn can_hold_target_bag(target_bag: &String, search_bag: &String, rules: &Rules) -> bool {
    match rules.get(search_bag) {
        Some(bags) => bags.iter().any(|(_, bag_name)| {
            bag_name == target_bag || can_hold_target_bag(target_bag, bag_name, rules)
        }),
        None => false,
    }
}

fn count_total_bags(target_bag: &String, rules: &Rules) -> usize {
    count_bags(target_bag, rules) - 1
}

fn count_bags(target_bag: &String, rules: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let counted = match rules.get(target_bag) {
        Some(bags) => {
            if bags.is_empty() {
                1
            } else {
                bags.iter().fold(1, |acc, (count, bag_name)| {
                    acc + count * count_bags(bag_name, rules)
                })
            }
        }
        None => 1,
    };
    counted - 1
}

fn parse_input() -> Result<Rules> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let rules = input
        .lines()
        .map(|l| {
            let splitted_string = l.split(" bags contain ").collect::<Vec<&str>>();
            let key = splitted_string[0];
            if splitted_string[1] == "no other bags." {
                (key.to_string(), vec![])
            } else {
                let val = splitted_string[1]
                    .split(", ")
                    .map(|s| {
                        let mut iter = s.split(" ");
                        let count = iter.next().unwrap().parse::<usize>().unwrap();
                        let name = iter.next().unwrap().to_string() + " " + iter.next().unwrap();
                        (count, name)
                    })
                    .collect::<Vec<(usize, String)>>();
                (key.to_string(), val)
            }
        })
        .collect();

    Ok(rules)
}
