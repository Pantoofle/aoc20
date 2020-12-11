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
    let mut numbers: Vec<i64> = input.lines().map(|v| v.parse::<i64>().unwrap()).collect();
    numbers.push(0);
    numbers.sort();
    numbers.push(numbers.last().unwrap() + 3);

    let steps: Vec<i64> = numbers.windows(2).map(|v| v[1] - v[0]).collect();

    Ok(steps.iter().filter(|x| **x == 1).count() * steps.iter().filter(|x| **x == 3).count())
}

fn exo2(input: &String) -> MyResult<i64> {
    let mut adaptators: Vec<i64> = input.lines().map(|v| v.parse::<i64>().unwrap()).collect();
    adaptators.push(0);
    adaptators.sort();

    let mut combi_to_goal: HashMap<i64, i64> = HashMap::new();
    combi_to_goal.insert(*adaptators.last().unwrap() + 3, 1);

    for jolt in adaptators.iter().rev() {
        let mut v = 0;
        for delta in 1..4 {
            v += combi_to_goal.get(&(jolt + delta)).unwrap_or(&(0));
        }
        combi_to_goal.insert(*jolt, v);
    }

    Ok(*combi_to_goal.get(&0).unwrap())
}
