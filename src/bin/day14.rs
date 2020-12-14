use aoc20::utils::get_input;
use std::collections::HashMap;
use std::collections::HashSet;

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MyResult<()> {
    let input = get_input()?;

    println!("Exo 1: {}", exo1(&input)?);
    println!("Exo 2: {}", exo2(&input)?);
    Ok(())
}

fn exo1(input: &String) -> MyResult<i64> {
    let mut mem: HashMap<u64, i64> = HashMap::new();
    let mut mask0: i64 = 0;
    let mut mask1: i64 = 0;

    for l in input.trim().lines() {
        let v: Vec<&str> = l.split('=').collect();
        let (ins, val) = (v[0], v[1]);
        match &ins[..3] {
            "mas" => {
                mask0 = 0;
                mask1 = 0;
                for (i, c) in val.to_string().trim().chars().rev().enumerate() {
                    match c {
                        '0' => mask0 += 1 << i,
                        '1' => mask1 += 1 << i,
                        _ => (),
                    }
                }
            }
            "mem" => {
                let addr = ins[4..ins.len() - 2].to_string().parse::<u64>().unwrap();
                let v = val.to_string().trim().parse::<i64>().unwrap();
                mem.insert(addr, (v | mask1) & !mask0);
            }
            _ => (),
        }
    }

    Ok(mem.values().sum())
}

fn exo2(input: &String) -> MyResult<i64> {
    let mut mem: HashMap<u64, i64> = HashMap::new();
    let mut mask: Vec<Option<u8>> = vec![Some(0); 36];

    for l in input.trim().lines() {
        let v: Vec<&str> = l.split('=').collect();
        let (ins, val) = (v[0], v[1]);
        match &ins[..3] {
            "mas" => {
                for (i, c) in val.to_string().trim().chars().enumerate() {
                    mask[i] = c.to_string().parse::<u8>().ok();
                }
            }
            "mem" => {
                let mut addr: HashSet<u64> = HashSet::new();
                addr.insert(ins[4..ins.len() - 2].to_string().parse::<u64>().unwrap());
                for (i, c) in mask.iter().rev().enumerate() {
                    match c {
                        Some(1) => addr = addr.iter().map(|v| v | (1 << i)).collect(),
                        None => {
                            addr = addr
                                .iter()
                                .map(|v| vec![v & !(1 << i), v | (1 << i)])
                                .flatten()
                                .collect()
                        }
                        _ => (),
                    };
                }
                let v = val.to_string().trim().parse::<i64>().unwrap();
                for a in addr {
                    mem.insert(a, v);
                }
            }
            _ => (),
        }
    }

    Ok(mem.values().sum())
}
