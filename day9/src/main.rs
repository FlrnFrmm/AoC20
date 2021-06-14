use anyhow::Result;
use std::io::Read;

fn main() -> Result<()> {
    let data = parse_input()?;

    println!("Result one: {:?}", find_bug(&data)); 

    println!("Result two: {:?}", fix_bug(&data));

    Ok(())
}

fn find_bug(data: &Vec<usize>) -> Option<usize> {
    let preamble_length = 25;

    for i in preamble_length..data.len() {
        let preamble_sums = gen_preamble_sums(&data[i-preamble_length..i]);

        let value_to_check = data[i];
        if !preamble_sums.contains(&value_to_check) {
            return Some(data[i]);
        }
    }

    None
}

fn gen_preamble_sums(preamble: &[usize]) -> Vec<usize> {
    preamble.iter()
        .enumerate()
        .take(preamble.len() - 1)
        .map(|(i, x)| preamble.iter().skip(i + 1).map(move |y| x + y))
        .flatten()
        .collect()
}

fn fix_bug(data: &Vec<usize>) -> Option<usize> {
    let bug_value = find_bug(data).unwrap();

    *(0..data.len()-1)
        .map(|i| {
            (i+1..data.len())
                .map(move |j| {
                    if data[i..j].iter().sum::<usize>() == bug_value {
                        Some(data[i..j].iter().min().unwrap() + data[i..j].iter().max().unwrap())
                    } else {
                        None
                    }
                })
        })
        .flatten()
        .skip_while(std::option::Option::is_none)
        .take(1)
        .collect::<Vec<Option<usize>>>()
        .first()
        .unwrap_or(&None)
}

fn parse_input() -> Result<Vec<usize>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let data = input.lines().map(|s| s.parse::<usize>().unwrap()).collect();

    Ok(data)
}
