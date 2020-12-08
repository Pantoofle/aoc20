use aoc20::utils::get_input;
use std::collections::HashSet;

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MyResult<()> {
    let input = get_input()?;

    println!("Exo 1: {}", exo1(&input)?);
    println!("Exo 2: {}", exo2(&input)?);
    Ok(())
}

fn exo1(input: &String) -> MyResult<i64> {
    let lines = parse_prog(input);
    if let Err(res) = run(&lines) {
        Ok(res)
    } else {
        Ok(0)
    }
}

fn exo2(input: &String) -> MyResult<i64> {
    let mut lines = parse_prog(input);

    for l in 0..lines.len() {
        let (_, ins, _) = lines.get_mut(l).unwrap();
        if ins.as_str() == "jmp" {
            *ins = "nop".to_owned();
        } else if ins.as_str() == "nop" {
            *ins = "jmp".to_owned();
        } else {
            continue;
        }

        if let Ok(res) = run(&lines) {
            return Ok(res);
        }

        let (_, ins, _) = lines.get_mut(l).unwrap();
        if ins.as_str() == "jmp" {
            *ins = "nop".to_owned();
        } else if ins.as_str() == "nop" {
            *ins = "jmp".to_owned();
        } else {
            continue;
        }
    }
    Ok(0)
}

fn parse_prog(input: &String) -> Vec<(usize, String, i64)> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(n, s)| {
            let words: Vec<&str> = s.split(' ').collect();
            (n, words[0].to_owned(), words[1].parse::<i64>().unwrap())
        })
        .collect()
}

fn run(prog: &Vec<(usize, String, i64)>) -> Result<i64, i64> {
    let mut seen: HashSet<usize> = HashSet::new();
    let mut acc: i64 = 0;

    let mut actual = 0;

    loop {
        if actual == prog.len() {
            return Ok(acc);
        } else if actual > prog.len() {
            return Err(0);
        }
        let (n, ins, c) = &prog[actual];
        if seen.contains(&n) {
            return Err(acc);
        }

        seen.insert(*n);

        match ins.as_str() {
            "nop" => (),
            "acc" => acc = acc + c,
            "jmp" => actual = (actual as i64 + c - 1) as usize,
            _ => (),
        }
        actual += 1;
    }
}
