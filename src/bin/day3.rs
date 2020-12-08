use aoc20::utils::get_input;

type MyResult<T> = std::result::Result<T, String>;

fn main() -> MyResult<()> {
    let input = get_input().unwrap();

    println!("Exo 1: {}", exo1(&input)?);
    println!("Exo 2: {}", exo2(&input)?);
    Ok(())
}

pub fn exo1(input: &String) -> MyResult<u64> {
    let trees = parse_trees(input);
    Ok(try_slope(3, 1, &trees))
}

pub fn exo2(input: &String) -> MyResult<u64> {
    let trees = parse_trees(input);
    let a = try_slope(1, 1, &trees);
    let b = try_slope(3, 1, &trees);
    let c = try_slope(5, 1, &trees);
    let d = try_slope(7, 1, &trees);
    let e = try_slope(1, 2, &trees);

    Ok(a * b * c * d * e)
}

fn try_slope(slope_x: usize, slope_y: usize, trees: &Vec<Vec<bool>>) -> u64 {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while y < trees.len() {
        if trees[y][x] {
            count += 1;
        }
        x += slope_x;
        x %= trees[0].len();

        y += slope_y;
    }
    count
}

fn parse_trees(input: &String) -> Vec<Vec<bool>> {
    let mut trees: Vec<Vec<bool>> = vec![];

    for line in input.trim().split('\n') {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c == '#')
        }
        trees.push(row);
    }
    trees
}
