use aoc20::utils::get_input;
use std::collections::HashMap;

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MyResult<()> {
    let input = get_input()?;

    println!("Exo 1: {}", exo1(&input)?);
    println!("Exo 2: {}", exo2(&input)?);
    Ok(())
}

fn exo1(input: &String) -> MyResult<u64> {
    let nums: Vec<u64> = input
        .trim()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse::<u64>().unwrap())
        .collect();

    Ok(find_nth(nums, 2020))
}
fn exo2(input: &String) -> MyResult<u64> {
    let nums: Vec<u64> = input
        .trim()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse::<u64>().unwrap())
        .collect();

    Ok(find_nth(nums, 30000000))
}

fn find_nth(nums: Vec<u64>, n: usize) -> u64 {
    let mut last_seen: HashMap<u64, usize> = HashMap::new();
    for (i, v) in nums[..nums.len() - 1].iter().enumerate() {
        last_seen.insert(*v, i);
    }

    let mut next: u64 = *nums.last().unwrap();

    for i in (nums.len() - 1)..(n - 1) {
        let future = match last_seen.get(&next) {
            None => 0,
            Some(j) => (i - j) as u64,
        };
        last_seen.insert(next, i);
        next = future;
    }
    next
}
