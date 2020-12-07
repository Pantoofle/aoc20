use std::fs;
use std::io;
use std::io::BufRead;

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

fn load_data(input: &str) -> Vec<Vec<bool>> {
    let file = fs::File::open(input).expect("Could not open file");
    let mut trees: Vec<Vec<bool>> = vec![];

    for line in io::BufReader::new(file).lines() {
        if let Ok(data) = line {
            let mut row = vec![];
            for c in data.chars() {
                row.push(c == '#')
            }
            trees.push(row);
        }
    }
    trees
}

pub fn exercice1(input: &str) -> u64 {
    let trees = load_data(input);
    try_slope(3, 1, &trees)
}

pub fn exercice2(input: &str) -> u64 {
    let trees = load_data(input);
    let a = try_slope(1, 1, &trees);
    let b = try_slope(3, 1, &trees);
    let c = try_slope(5, 1, &trees);
    let d = try_slope(7, 1, &trees);
    let e = try_slope(1, 2, &trees);

    a * b * c * d * e
}
