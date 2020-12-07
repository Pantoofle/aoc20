use aoc20::utils;
use itertools::Itertools;

type MyResult<T> = std::result::Result<T, String>;

fn main() -> MyResult<()> {
    let input = utils::get_input().unwrap();

    println!("Exo 1: {}", exo1(&input)?);
    println!("Exo 2: {}", exo2(&input)?);
    Ok(())
}

fn exo1(input: &String) -> MyResult<u64> {
    let mut entries: Vec<u64> = input
        .trim()
        .split('\n')
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect();

    entries.sort();

    for combi in entries.iter().combinations(2) {
        if combi.iter().copied().sum::<u64>() == 2020 {
            return Ok(combi[0] * combi[1]);
        }
    }
    Err("No combination matches 2020".to_owned())
}

fn exo2(input: &String) -> MyResult<u64> {
    let mut entries: Vec<u64> = input
        .trim()
        .split('\n')
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect();

    entries.sort();

    for combi in entries.iter().combinations(3) {
        if combi.iter().copied().sum::<u64>() == 2020 {
            return Ok(combi[0] * combi[1] * combi[2]);
        }
    }
    Err("No combination matches 2020".to_owned())
}
