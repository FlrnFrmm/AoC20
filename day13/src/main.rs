use anyhow::Result;
use std::io::Read;
use num::BigInt;

fn main() -> Result<()> {
    let (earliest_departure, bus_ids) = parse_input()?;

    println!("Result one: {:?}", p1(earliest_departure, &bus_ids)); 

    println!("Result two: {:?}", p2(&bus_ids));

    Ok(())
}

fn p1(earliest_departure: usize, bus_ids: &Vec<Option<usize>>) -> Result<usize> {
    let mut result = 0;

    'outer: for time in earliest_departure.. {
        for id in bus_ids.iter() {
            if let Some(id_value) = id {
                if time % id_value == 0 {
                    result = id_value * (time - earliest_departure);
                    break 'outer;
                } 
            }
        }
    }

    Ok(result)
}

fn p2(bus_ids: &Vec<Option<usize>>) -> Result<BigInt> {
    let offsets_and_ids = bus_ids.iter()
        .enumerate()
        .map(|(index , bus_id)|{
            match bus_id {
                Some(id) => Some((BigInt::from(index % id), BigInt::from(*id))),
                None => None
            }
        })
        .flatten()
        .collect::<Vec<(BigInt, BigInt)>>();

    Ok(chinese_remainder(offsets_and_ids).unwrap())
}

fn egcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a == &BigInt::from(0) {
        (b.clone(), BigInt::from(0), BigInt::from(1))
    } else {
        let (g, x, y) = egcd(&(b % a), a);
        (g, y - (b / a) * &x, x)
    }
}
 
fn mod_inv(x: &BigInt, n: &BigInt) -> Option<BigInt> {
    let (g, x, _) = egcd(x, n);
    if g == BigInt::from(1) {
        Some((x % n + n) % n)
    } else {
        None
    }
}
 
fn chinese_remainder(offsets_and_ids: Vec<(BigInt, BigInt)>) -> Option<BigInt> {
    let prod = offsets_and_ids.iter().map(|(_, id)| id).product::<BigInt>();
 
    let mut sum = BigInt::from(0);
 
    for (offset, id) in offsets_and_ids {
        let p = &prod / &id;
        sum += offset * mod_inv(&p, &id)? * p
    }
 
    Some(sum % prod)
}

fn parse_input() -> Result<(usize, Vec<Option<usize>>)> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let mut input_iterator = input.lines();

    let earliest_departure = input_iterator.next().unwrap().parse::<usize>()?;
    let bus_ids = input_iterator.next().unwrap()
        .split(",")
        .map(|id| {
            if id == "x" {
                None
            } else {
                Some(id.parse::<usize>().unwrap())
            }
        })
        .collect();


    Ok((earliest_departure, bus_ids))
}
