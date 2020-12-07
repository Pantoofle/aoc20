use regex::Regex;
use std::io;
use std::{fs, io::BufRead};

pub fn exercice1(input: &str) -> u64 {
    let file = fs::File::open(input).expect("Could not open file");

    let re = Regex::new(r"^(\d*)-(\d*) (.): (.*)$").expect("Error creating regex");
    let mut res = 0;

    for line in io::BufReader::new(file).lines() {
        if let Ok(data) = line {
            let fields = re.captures(data.as_str()).expect("Line did not match");
            let min = fields[1].parse::<usize>().expect("Error parsing");
            let max = fields[2].parse::<usize>().expect("Error parsing");
            let c = fields[3]
                .to_owned()
                .chars()
                .next()
                .expect("Could not read char");
            let pass = fields[4].to_owned();
            let nb = pass.match_indices(c).count();
            if min <= nb && nb <= max {
                res += 1;
            }
        }
    }

    res
}

pub fn exercice2(input: &str) -> u64 {
    let file = fs::File::open(input).expect("Could not open file");

    let re = Regex::new(r"^(\d*)-(\d*) (.): (.*)$").expect("Error creating regex");
    let mut res = 0;

    for line in io::BufReader::new(file).lines() {
        if let Ok(data) = line {
            let fields = re.captures(data.as_str()).expect("Line did not match");
            let a = fields[1].parse::<usize>().expect("Error parsing");
            let b = fields[2].parse::<usize>().expect("Error parsing");
            let c = fields[3]
                .to_owned()
                .chars()
                .next()
                .expect("Could not read char");
            let pass = fields[4].to_owned();
            let ina = pass.chars().nth(a - 1).expect("Pos a not found") == c;
            let inb = pass.chars().nth(b - 1).expect("Pos b not found") == c;

            if !(ina && inb) & (ina || inb) {
                res += 1;
            }
        }
    }

    res
}
