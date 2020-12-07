use regex::Regex;
use std::i64;
use std::io;
use std::{fs, io::BufRead};

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

pub fn exercice1(input: &str) -> u64 {
    let file = fs::File::open(input).expect("Could not open file");

    let re = Regex::new(r"([a-z]*):[^ ]*").expect("Error creating regex");
    let mut current: Vec<String> = vec![];
    let mut res = 0;

    for line in io::BufReader::new(file).lines() {
        if let Ok(data) = line {
            if data.len() == 0 {
                if check_fields(&current) {
                    res += 1;
                }
                current = vec![];
                continue;
            }
            for cap in re.captures_iter(data.as_str()) {
                current.push(cap[1].to_owned());
            }
        }
    }
    if check_fields(&current) {
        res += 1;
    }
    res
}

pub fn exercice2(input: &str) -> u64 {
    let file = fs::File::open(input).expect("Could not open file");

    let re = Regex::new(r"([a-z]*):([^ ]*)").expect("Error creating regex");
    let mut current: Vec<(String, String)> = vec![];
    let mut res = 0;

    for line in io::BufReader::new(file).lines() {
        if let Ok(data) = line {
            if data.len() == 0 {
                if check_passport(&current) {
                    res += 1;
                }
                current = vec![];
                continue;
            }
            for cap in re.captures_iter(data.as_str()) {
                current.push((cap[1].to_owned(), cap[2].to_owned()));
            }
        }
    }
    if check_passport(&current) {
        res += 1;
    }
    res
}
