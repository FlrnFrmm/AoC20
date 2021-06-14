use anyhow::Result;
use std::io::Read;
use std::collections::HashMap;

fn main() -> Result<()> {
    let mut joltages = parse_input()?;
    joltages.sort();

    println!("Result one: {:?}", p1(&joltages)); 

    println!("Result two: {:?}", p2(&joltages));

    Ok(())
}

fn p1(joltages: &Vec<usize>) -> Result<usize> {
    let (_, differences) = joltages.iter().fold((0, vec![]), |(last, mut differences), joltage| {
        differences.push(*joltage - last);
        (*joltage, differences)
    });

    let ones = differences.iter().filter(|v| 1 == **v).count();
    let threes = differences.iter().filter(|v| 3 == **v).count() + 1;

    Ok(ones * threes)
}

fn p2(joltages: &Vec<usize>) -> Result<usize> {
    println!("{:?}", joltages);

    let mut possible_ways: HashMap<usize, usize> = HashMap::new();

    for i in 0..joltages.len() - 1 {
        for j in 1..=3 {
            let node = i + j;
            if node < joltages.len() && joltages[node] - joltages[i] < 4 {
                let mut value = 1;
                if let Some(v) = possible_ways.get(&i) {
                    value = *v;
                }
                *possible_ways.entry(node).or_insert(0) += value;
            }
        }
    }

    Ok(*possible_ways.get(&(joltages.len() - 1)).unwrap())
}

fn parse_input() -> Result<Vec<usize>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let mut joltages: Vec<usize> = input.lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    joltages.insert(0, 0);
    joltages.push(*joltages.iter().max().unwrap() + 3);

    Ok(joltages)
}
