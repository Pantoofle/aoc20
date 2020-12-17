use aoc20::utils::get_input;
use itertools::iproduct;

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MyResult<()> {
    let input = get_input()?;
    let layer = parse_input(&input);

    println!("Exo 1: {}", exo1(&layer)?);
    println!("Exo 2: {}", exo2(&layer)?);
    Ok(())
}

fn parse_input(input: &String) -> Vec<Vec<bool>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn exo1(layer: &Vec<Vec<bool>>) -> MyResult<usize> {
    let iterations = 6;
    let n = iterations + 1;
    let mut map = vec![vec![vec![false; layer[0].len() + 2 * n]; layer.len() + 2 * n]; 2 * n + 1];
    for (i, line) in layer.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            map[n][i + n][j + n] = *cell;
        }
    }
    // print_map(&map);

    for _ in 0..iterations {
        map = update(map);
        // print_map(&map);
    }

    Ok(map.iter().flatten().flatten().filter(|x| **x).count())
}

fn exo2(layer: &Vec<Vec<bool>>) -> MyResult<usize> {
    let iterations = 6;
    let n = iterations + 1;
    let mut map =
        vec![
            vec![vec![vec![false; layer[0].len() + 2 * n]; layer.len() + 2 * n]; 2 * n + 1];
            2 * n + 1
        ];
    for (i, line) in layer.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            map[n][n][i + n][j + n] = *cell;
        }
    }
    // print_map(&map);

    for _ in 0..iterations {
        map = update4D(map);
        // print_map(&map);
    }

    Ok(map
        .iter()
        .flatten()
        .flatten()
        .flatten()
        .filter(|x| **x)
        .count())
}

type Line = Vec<bool>;
type Plan = Vec<Line>;
type Volume = Vec<Plan>;
type Hyper = Vec<Volume>;

fn update(map: Volume) -> Volume {
    let mut new_map = map.clone();
    let (size_x, size_y, size_z) = (map.len(), map[0].len(), map[0][0].len());

    for (i, j, k) in iproduct!(1..size_x - 1, 1..size_y - 1, 1..size_z - 1) {
        let neighbours: Vec<bool> = iproduct!(-1..2, -1..2, -1..2)
            .filter(|(a, b, c)| (*a, *b, *c) != (0, 0, 0))
            .map(|(a, b, c)| {
                map[(i as isize + a) as usize][(j as isize + b) as usize][(k as isize + c) as usize]
            })
            .collect();
        // println!("Neighbours of {}, {}, {} : {:?}", i, j, k, neighbours);
        match (map[i][j][k], neighbours.iter().filter(|x| **x).count()) {
            (true, x) => {
                if (x != 2) & (x != 3) {
                    new_map[i][j][k] = false
                }
            }
            (false, x) => {
                if x == 3 {
                    new_map[i][j][k] = true
                }
            }
        }
    }
    new_map
}
fn update4D(map: Hyper) -> Hyper {
    let mut new_map = map.clone();
    let (size_x, size_y, size_z, size_w) =
        (map.len(), map[0].len(), map[0][0].len(), map[0][0][0].len());

    for (i, j, k, l) in iproduct!(1..size_x - 1, 1..size_y - 1, 1..size_z - 1, 1..size_w - 1) {
        let neighbours: Vec<bool> = iproduct!(-1..2, -1..2, -1..2, -1..2)
            .filter(|(a, b, c, d)| (*a, *b, *c, *d) != (0, 0, 0, 0))
            .map(|(a, b, c, d)| {
                map[(i as isize + a) as usize][(j as isize + b) as usize][(k as isize + c) as usize]
                    [(l as isize + d) as usize]
            })
            .collect();
        // println!("Neighbours of {}, {}, {} : {:?}", i, j, k, neighbours);
        match (map[i][j][k][l], neighbours.iter().filter(|x| **x).count()) {
            (true, x) => {
                if (x != 2) & (x != 3) {
                    new_map[i][j][k][l] = false
                }
            }
            (false, x) => {
                if x == 3 {
                    new_map[i][j][k][l] = true
                }
            }
        }
    }
    new_map
}

fn print_map(map: &Volume) -> () {
    for plan in map {
        for line in plan {
            println!(
                "{}",
                line.iter()
                    .map(|x| if *x { '#' } else { '.' })
                    .collect::<String>()
            );
        }
        println!(" --- ");
    }
    println!("                       ");
}
