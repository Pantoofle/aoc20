use aoc20::utils::get_input;
use regex::Regex;

type MyResult<T> = std::result::Result<T, String>;
fn main() -> MyResult<()> {
    let input = get_input().unwrap();

    println!("Exo 1: {}", exo1(&input)?);
    println!("Exo 2: {}", exo2(&input)?);
    Ok(())
}

pub fn exo1(input: &String) -> MyResult<i64> {
    let re = Regex::new(r"^(\d*)-(\d*) (.): (.*)$").expect("Error creating regex");
    let mut res = 0;

    for line in input.trim().split('\n') {
        let fields = re.captures(line).expect("Line did not match");
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
    Ok(res)
}

pub fn exo2(input: &String) -> MyResult<u64> {
    let re = Regex::new(r"^(\d*)-(\d*) (.): (.*)$").expect("Error creating regex");
    let mut res = 0;

    for line in input.trim().split('\n') {
        let fields = re.captures(line).expect("Line did not match");
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
    Ok(res)
}
