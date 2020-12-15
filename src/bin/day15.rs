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
    let nums: Vec<usize> = input
        .trim()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    Ok(find_nth(nums, 2020))
}
fn exo2(input: &String) -> MyResult<usize> {
    let nums: Vec<usize> = input
        .trim()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    Ok(find_nth(nums, 30000000))
}

fn find_nth(nums: Vec<usize>, n: usize) -> usize {
    let mut last_seen: Vec<Option<usize>> = Vec::with_capacity(1_000);
    last_seen.resize(1_000, None);
    for (i, v) in nums[..nums.len() - 1].iter().enumerate() {
        last_seen[*v] = Some(i);
    }

    let mut next: usize = *nums.last().unwrap();

    for i in (nums.len() - 1)..(n - 1) {
        let future = match last_seen.get(next) {
            None => {
                last_seen.resize(2 * next, None);
                0
            }
            Some(val) => match val {
                None => 0,
                Some(j) => (i - j),
            },
        };
        last_seen[next] = Some(i);
        next = future;
    }
    next
}
