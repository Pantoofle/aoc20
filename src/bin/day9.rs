use aoc20::utils::get_input;
use std::{collections::HashSet, vec};

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MyResult<()> {
    let input = get_input()?;

    println!("Exo 1: {}", exo1(&input)?);
    println!("Exo 2: {}", exo2(&input)?);
    Ok(())
}

fn exo1(input: &String) -> MyResult<i64> {
    find_error(input)
}

fn find_error(input: &String) -> MyResult<i64> {
    let mut window: Vec<i64> = vec![];
    let mut set: HashSet<i64> = HashSet::new();
    let mut lines = input.trim().lines();

    // Load the preamble
    for _ in 0..25 {
        let val = lines.next().unwrap().parse::<i64>().unwrap();
        window.push(val);
        set.insert(val);
    }

    // Start the test
    'line: for s in lines {
        let v = s.parse::<i64>().unwrap();
        for b in &set {
            if set.contains(&(v - *b)) {
                set.insert(v);
                set.remove(&window.remove(0));
                window.push(v);
                continue 'line;
            }
        }
        return Ok(v);
    }
    Ok(0)
}

fn exo2(input: &String) -> MyResult<i64> {
    let goal = find_error(input).unwrap();
    let mut window: Vec<i64> = vec![];
    let mut sum: i64 = 0;
    let mut lines = input.trim().lines();

    while sum != goal {
        if sum < goal {
            let v = lines.next().unwrap().parse::<i64>().unwrap();
            window.push(v);
            sum += v;
        } else {
            sum -= window.remove(0);
        }
    }
    Ok(window.iter().min().unwrap() + window.iter().max().unwrap())
}
