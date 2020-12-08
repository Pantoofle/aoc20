use aoc20::utils::get_input;
use std::collections::HashMap;

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MyResult<()> {
    let input = get_input()?;

    println!("Exo 1: {}", exo1(&input)?);
    println!("Exo 2: {}", exo2(&input)?);
    Ok(())
}

fn exo1(input: &String) -> MyResult<usize> {
    let mut res: usize = 0;
    for group in input.trim().split("\n\n") {
        let mut fields: Vec<char> = group.to_owned().replace('\n', "").chars().collect();
        fields.sort();
        fields.dedup();
        res += fields.len();
    }
    Ok(res)
}
fn exo2(input: &String) -> MyResult<usize> {
    let mut res: usize = 0;
    for group in input.trim().split("\n\n") {
        let mut counts: HashMap<char, usize> = HashMap::new();
        let members: usize = group.to_owned().lines().count();
        let mut fields: Vec<char> = group.to_owned().replace('\n', "").chars().collect();

        fields.sort();
        fields
            .iter()
            .for_each(|c| *counts.entry(*c).or_insert(0) += 1);
        res += counts.iter().filter(|(_, v)| **v == members).count();
    }
    Ok(res)
}
