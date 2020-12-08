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
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for rule in input.trim().lines() {
        let tmp: Vec<&str> = rule.split("contain").collect();
        let bag: Vec<&str> = tmp[0].split(" ").collect();
        let bag: String = bag[..2].join(" ");

        let containers: Vec<String> = tmp[1]
            .split(",")
            .map(|e| {
                let w: Vec<&str> = e.split(' ').collect();
                w[2..4].join(" ")
            })
            .collect();

        for c in containers {
            map.entry(c).or_insert(vec![]).push(bag.clone());
        }
    }

    let mut containers: Vec<String> = down_roam(&mut map, &"shiny gold".to_owned());
    containers.sort();
    containers.dedup();

    Ok(containers.len())
}

fn down_roam(tree: &HashMap<String, Vec<String>>, node: &String) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    if tree.contains_key(node) {
        for bag in &tree[node] {
            res.push(bag.clone());
            res.append(down_roam(tree, bag).as_mut());
        }
    }
    res
}

fn exo2(input: &String) -> MyResult<u64> {
    let mut bag_tree: HashMap<String, Vec<(String, u64)>> = HashMap::new();

    for rule in input.trim().lines() {
        let tmp: Vec<&str> = rule.split("contain").collect();
        let bag: Vec<&str> = tmp[0].split(" ").collect();
        let bag: String = bag[..2].join(" ");

        let sub_bags: Vec<(String, u64)> = tmp[1]
            .split(",")
            .map(|e| {
                let w: Vec<&str> = e.split(' ').collect();
                let entry = w[2..w.len() - 1].join(" ");
                let c = w[1].parse::<u64>().unwrap_or(0);
                (entry, c)
            })
            .filter(|(_, c)| *c != 0)
            .collect();

        for (name, count) in sub_bags {
            bag_tree
                .entry(bag.clone())
                .or_insert(vec![])
                .push((name, count));
        }
    }

    Ok(down_count(&bag_tree, &"shiny gold".to_owned()) - 1)
}
fn down_count(tree: &HashMap<String, Vec<(String, u64)>>, node: &String) -> u64 {
    let mut res: u64 = 1;
    if tree.contains_key(node) {
        for (bag, count) in &tree[node] {
            res += *count * down_count(tree, bag);
        }
    } else {
        println!("Reached end node {}", node);
    }
    res
}
