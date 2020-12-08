use regex::Regex;
use std::i64;

use aoc20::utils::get_input;

type MyResult<T> = std::result::Result<T, String>;

fn main() -> MyResult<()> {
    let input = get_input().unwrap();

    println!("Exo 1: {}", exo1(&input)?);
    println!("Exo 2: {}", exo2(&input)?);
    Ok(())
}

fn check_fields(current: &Vec<String>) -> bool {
    let valid = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut res = true;
    print!("Passport has ");
    for entry in current {
        print!("{} ", entry);
    }
    println!("");
    println!("Result :");
    for a in valid.iter() {
        if !current.iter().any(|i| i == *a) {
            println!("no {} field", a);
            res = false;
            break;
        }
    }
    println!("{}", res);
    res
}

fn check_passport(current: &Vec<(String, String)>) -> bool {
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut res = true;

    for a in fields.iter() {
        if !current.iter().any(|(f, _)| f == *a) {
            println!("no {} field", a);
            res = false;
            break;
        }
    }

    if res {
        for (field, val) in current {
            if !(match field.as_str() {
                "byr" => {
                    if val.len() == 4 {
                        if let Ok(v) = val.parse::<usize>() {
                            (1920 <= v) && (v <= 2002)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                "iyr" => {
                    if val.len() == 4 {
                        if let Ok(v) = val.parse::<usize>() {
                            (2010 <= v) && (v <= 2020)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                "eyr" => {
                    if val.len() == 4 {
                        if let Ok(v) = val.parse::<usize>() {
                            (2020 <= v) && (v <= 2030)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                "hgt" => {
                    if let Ok(v) = val[..val.len() - 2].parse::<usize>() {
                        match &val[val.len() - 2..] {
                            "in" => (59 <= v) && (v <= 76),
                            "cm" => (150 <= v) && (v <= 193),
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
                "hcl" => {
                    if val.len() == 7 {
                        if let Ok(_) = i64::from_str_radix(&val[1..], 16) {
                            val.chars().next().unwrap() == '#'
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                "ecl" => match val.as_str() {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false,
                },
                "pid" => regex::Regex::new(r"^\d{9}$")
                    .unwrap()
                    .is_match(val.as_str()),
                "cid" => true,
                _ => false,
            }) {
                println!("{}:{} did not match", field, val);
                res = false
            }
        }
    }
    res
}

pub fn exo1(input: &String) -> MyResult<u64> {
    let re = Regex::new(r"([a-z]*):[^ ]*").expect("Error creating regex");
    let mut res = 0;

    for data in input.trim().split("\n\n") {
        let fields: Vec<String> = re
            .captures_iter(data.to_owned().replace('\n', " ").as_str())
            .map(|c| c[1].to_owned())
            .collect();
        if check_fields(&fields) {
            res += 1;
        }
    }
    Ok(res)
}

pub fn exo2(input: &String) -> MyResult<u64> {
    let re = Regex::new(r"([a-z]*):([^ ]*)").expect("Error creating regex");
    let mut res = 0;

    for data in input.trim().split("\n\n") {
        let fields: Vec<(String, String)> = re
            .captures_iter(data.to_owned().replace('\n', " ").as_str())
            .map(|c| (c[1].to_owned(), c[2].to_owned()))
            .collect();
        if check_passport(&fields) {
            res += 1;
        }
    }
    Ok(res)
}
