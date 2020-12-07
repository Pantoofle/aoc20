use std::io;
use std::{fs, io::BufRead};

pub fn exercice1(input: &str) -> u64 {
    let file = fs::File::open(input).expect("Could not open file");
    let mut entries: Vec<u64> = vec![];

    for line in io::BufReader::new(file).lines() {
        if let Ok(data) = line {
            entries.push(data.parse::<u64>().expect("Error parsing number"));
        }
    }

    entries.sort();

    let mut res: u64 = 0;
    'base: for a in entries.iter() {
        for b in entries.iter() {
            if a + b == 2020 {
                res = a * b;
                break 'base;
            } else if a + b > 2020 {
                continue 'base;
            }
        }
    }
    res
}

pub fn exercice2(input: &str) -> u64 {
    let file = fs::File::open(input).expect("Could not open file");
    let mut entries: Vec<u64> = vec![];

    for line in io::BufReader::new(file).lines() {
        if let Ok(data) = line {
            entries.push(data.parse::<u64>().expect("Error parsing number"));
        }
    }

    entries.sort();

    let mut res: u64 = 0;
    'base: for a in entries.iter() {
        'second: for b in entries.iter() {
            if a + b > 2020 {
                continue 'base;
            }
            for c in entries.iter() {
                if a + b + c > 2020 {
                    continue 'second;
                }
                if a + b + c == 2020 {
                    res = a * b * c;
                    break 'base;
                }
            }
        }
    }
    res
}
