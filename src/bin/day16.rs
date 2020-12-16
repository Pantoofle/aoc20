use std::collections::HashSet;

use aoc20::utils::get_input;

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MyResult<()> {
    let input = get_input()?;
    let (fields, my_pass, other_pass) = parse_input(&input);

    println!("Exo 1: {}", exo1(&fields, &my_pass, &other_pass)?);
    println!("Exo 2: {}", exo2(&fields, &my_pass, &other_pass)?);
    Ok(())
}

fn parse_input(input: &String) -> (Vec<[(usize, usize); 2]>, Vec<usize>, Vec<Vec<usize>>) {
    let mut lines = input.trim().lines();
    let mut fields: Vec<[(usize, usize); 2]> = vec![];

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut f = line.split(": ").nth(1).unwrap().split(" or ").map(|v| {
            let mut vals = v.split('-').map(|a| a.parse::<usize>().unwrap());
            (vals.next().unwrap(), vals.next().unwrap())
        });
        fields.push([f.next().unwrap(), f.next().unwrap()]);
    }

    let my_pass: Vec<usize> = lines
        .nth(1)
        .unwrap()
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let other_pass: Vec<Vec<usize>> = lines
        .skip(2)
        .map(|l| l.split(',').map(|v| v.parse::<usize>().unwrap()).collect())
        .collect();

    (fields, my_pass, other_pass)
}

fn fits_in_field(val: usize, field: &[(usize, usize); 2]) -> bool {
    return (field[0].0 <= val && val <= field[0].1) || (field[1].0 <= val && val <= field[1].1);
}
fn fits_in_a_field(val: usize, fields: &Vec<[(usize, usize); 2]>) -> bool {
    return fields.iter().any(|field| fits_in_field(val, field));
}

fn exo1(
    fields: &Vec<[(usize, usize); 2]>,
    _: &Vec<usize>,
    other_pass: &Vec<Vec<usize>>,
) -> MyResult<usize> {
    Ok(other_pass
        .iter()
        .flatten()
        .filter(|&val| !fits_in_a_field(*val, &fields))
        .sum())
}

fn filter_with_pass(
    entry_can_be: &mut Vec<HashSet<usize>>,
    pass: &Vec<usize>,
    fields: &Vec<[(usize, usize); 2]>,
) -> () {
    for (i, val) in pass.iter().enumerate() {
        entry_can_be[i] = entry_can_be[i]
            .iter()
            .filter(|&f| fits_in_field(*val, &fields[*f]))
            .map(|v| v.to_owned())
            .collect();
    }
}

fn exo2(
    fields: &Vec<[(usize, usize); 2]>,
    my_pass: &Vec<usize>,
    other_pass: &Vec<Vec<usize>>,
) -> MyResult<usize> {
    let valid_pass: Vec<Vec<usize>> = other_pass
        .iter()
        .filter(|&pass| pass.iter().all(|val| fits_in_a_field(*val, &fields)))
        .map(|v| v.to_owned())
        .collect();

    // Find the possible matchings between field and entry number
    let mut entry_can_be: Vec<HashSet<usize>> =
        vec![HashSet::from((0..fields.len()).collect()); my_pass.len()];
    for pass in valid_pass {
        filter_with_pass(&mut entry_can_be, &pass, fields);
    }
    filter_with_pass(&mut entry_can_be, &my_pass, fields);

    let mut association: Vec<Option<usize>> = vec![None; fields.len()];
    for _ in 0..fields.len() {
        for (i, unique_set) in entry_can_be
            .iter()
            .enumerate()
            .filter(|(_, poss)| poss.len() == 1)
        {
            association[i] = Some(*unique_set.iter().next().unwrap());
        }
        association.iter().filter(|v| v.is_some()).for_each(|v| {
            entry_can_be.iter_mut().for_each(|s| {
                s.remove(&v.unwrap());
                ()
            })
        });
    }
    println!("{:?}", association);
    let mut res = 1;
    for i in 0..6 {
        let f = association.iter().position(|x| *x == Some(i));
        res *= my_pass[f.unwrap()];
    }

    Ok(res)
}
