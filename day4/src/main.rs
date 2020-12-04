use anyhow::Result;
use std::io::Read;
use std::collections::HashMap;

fn main() -> Result<()> {
    let passport_strings = parse_input()?;

    let count = passport_strings.iter()
        .filter(|passport_string| validate_passport(*passport_string))
        .count();

    println!("{}", count);

    Ok(())
}

fn validate_passport(passport_string: &String) -> bool {
    let passport = parse_passport(&passport_string);

    let validators: Vec<&dyn Fn(&HashMap<String,String>) -> bool> = vec![
        &validate_byr,
        &validate_iyr,
        &validate_eyr,
        &validate_hgt,
        &validate_hcl,
        &validate_ecl,
        &validate_pid
    ];

    validators.iter()
        .all(|validator| validator(&passport))
}

fn validate_byr(passport: &HashMap<String, String>) -> bool {
    if let Some(value) = passport.get("byr") {
        if value.len() == 4 && value.chars().all(char::is_numeric) {
            let numeric_value = value.parse::<usize>().unwrap();
            numeric_value > 1919 && numeric_value < 2003
        } else {
            false
        }
    } else {
        false
    }
}

fn validate_iyr(passport: &HashMap<String, String>) -> bool {
    if let Some(value) = passport.get("iyr") {
        if value.len() == 4 && value.chars().all(char::is_numeric) {
            let numeric_value = value.parse::<usize>().unwrap();
            numeric_value > 2009 && numeric_value < 2021
        } else {
            false
        }
    } else {
        false
    }
}

fn validate_eyr(passport: &HashMap<String, String>) -> bool {
    if let Some(value) = passport.get("eyr") {
        if value.len() == 4 && value.chars().all(char::is_numeric) {
            let numeric_value = value.parse::<usize>().unwrap();
            numeric_value > 2019 && numeric_value < 2031
        } else {
            false
        }
    } else {
        false
    }
}

fn validate_hgt(passport: &HashMap<String, String>) -> bool {
    if let Some(value) = passport.get("hgt") {
        let number = value.chars()
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let unit = value.chars()
            .skip_while(|c| c.is_numeric())
            .collect::<String>();
        if unit == "cm" || unit == "in" {
            if unit == "cm" {
                number > 149 && number < 194
            } else {
                number > 58 && number < 77
            }
        } else {
            false            
        }
    } else {
        false
    }
}

fn validate_hcl(passport: &HashMap<String, String>) -> bool {
    if let Some(value) = passport.get("hcl") {
        value.starts_with('#') && value.chars().skip(1).all(|c| c.is_numeric() || c.is_ascii_alphabetic())
    } else {
        false
    }
}

fn validate_ecl(passport: &HashMap<String, String>) -> bool {
    if let Some(value) = passport.get("ecl") {
        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value.as_str())
    } else {
        false
    }
}

fn validate_pid(passport: &HashMap<String, String>) -> bool {
    if let Some(value) = passport.get("pid") {
        value.len() == 9 && value.chars().all(char::is_numeric)
    } else {
        false
    }
}

fn parse_input() -> Result<Vec<String>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    Ok(input.split("\n\n").map(|s| String::from(s)).collect())
}

fn parse_passport(passport_string: &String) -> HashMap<String, String> {
    passport_string.split_whitespace()
        .map(|s| {
            let mut iter = s.split(":").map(|s| String::from(s));
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect()
}
