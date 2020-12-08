use aoc20::utils::get_input;

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MyResult<()> {
    let input = get_input()?;

    println!("Exo 1: {}", exo1(&input)?);
    println!("Exo 2: {}", exo2(&input)?);
    Ok(())
}

fn exo1(input: &String) -> MyResult<i64> {
    let values: Vec<i64> = input.trim().split('\n').map(compute_id).collect();
    Ok(values.iter().max().unwrap().to_owned())
}

fn exo2(input: &String) -> MyResult<i64> {
    let mut values: Vec<i64> = input.trim().split('\n').map(compute_id).collect();
    values.sort();
    let mut prev = values[0];

    for v in values {
        if v - prev == 2 {
            break;
        } else {
            prev = v
        }
    }
    Ok(prev + 1)
}

fn compute_id(line: &str) -> i64 {
    let mut position = String::from(line);
    position = position.replace('F', "0");
    position = position.replace('B', "1");
    position = position.replace('L', "0");
    position = position.replace('R', "1");

    let row = i64::from_str_radix(&position.as_str()[..7], 2).unwrap();
    let col = i64::from_str_radix(&position.as_str()[7..], 2).unwrap();
    row * 8 + col
}
