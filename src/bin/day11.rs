use aoc20::utils::get_input;

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MyResult<()> {
    let input = get_input()?;

    println!("Exo 1: {}", exo1(&input)?);
    println!("Exo 2: {}", exo2(&input)?);
    Ok(())
}

type Grid = Vec<Vec<char>>;

fn exo1(input: &String) -> MyResult<usize> {
    let mut grid: Grid = input
        .trim()
        .lines()
        .map(|line| {
            let mut l: Vec<char> = line.chars().collect();
            l.push('.');
            l.insert(0, '.');
            l
        })
        .collect();

    grid.insert(0, vec!['.'; grid[0].len()]);
    grid.push(vec!['.'; grid[0].len()]);

    loop {
        let (new_grid, changed) = update(&grid, &neighbours, 4);
        if !changed {
            break;
        }
        grid = new_grid;
    }
    Ok(grid.iter().flatten().filter(|c| **c == '#').count())
}

fn exo2(input: &String) -> MyResult<usize> {
    let mut grid: Grid = input
        .trim()
        .lines()
        .map(|line| {
            let mut l: Vec<char> = line.chars().collect();
            l.push('.');
            l.insert(0, '.');
            l
        })
        .collect();

    grid.insert(0, vec!['.'; grid[0].len()]);
    grid.push(vec!['.'; grid[0].len()]);

    loop {
        let (new_grid, changed) = update(&grid, &visible, 5);
        if !changed {
            break;
        }
        grid = new_grid;
    }
    Ok(grid.iter().flatten().filter(|c| **c == '#').count())
}

fn update(
    grid: &Grid,
    neighbours: &dyn Fn(&Grid, usize, usize) -> usize,
    pop_limit: usize,
) -> (Grid, bool) {
    let mut new_grid: Grid = grid.clone();
    let mut changed = false;

    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            let n = neighbours(grid, i, j);

            new_grid[i][j] = match grid[i][j] {
                '.' => '.',
                '#' => {
                    if n >= pop_limit {
                        changed = true;
                        'L'
                    } else {
                        '#'
                    }
                }
                'L' => {
                    if n == 0 {
                        changed = true;
                        '#'
                    } else {
                        'L'
                    }
                }
                _ => panic!("Error parsing grid"),
            }
        }
    }
    // println!("Grid:");
    // for l in grid {
    //     println!("{:?}", l);
    // }
    (new_grid, changed)
}

fn neighbours(grid: &Grid, i: usize, j: usize) -> usize {
    let mut prox: Vec<char> = grid[i - 1..i + 2]
        .to_vec()
        .iter()
        .map(|v| v[j - 1..j + 2].to_vec())
        .flatten()
        .collect();
    prox.remove(4);
    prox.iter().filter(|c| **c == '#').count()
}

fn visible(grid: &Grid, i: usize, j: usize) -> usize {
    let mut res = 0;
    for x in 0..3 {
        for y in 0..3 {
            if sees_taken_seat(grid, i as i64, j as i64, x - 1, y - 1) {
                res += 1;
            }
        }
    }
    res
}

fn sees_taken_seat(seats: &Grid, i: i64, j: i64, angle_x: i64, angle_y: i64) -> bool {
    let mut i = i;
    let mut j = j;
    if (angle_x, angle_y) == (0, 0) {
        return false;
    }
    i += angle_x;
    j += angle_y;
    while 0 <= i && i < (seats.len() as i64) && 0 <= j && j < (seats[0].len() as i64) {
        match seats[i as usize][j as usize] {
            '#' => return true,
            'L' => return false,
            _ => (),
        }
        i += angle_x;
        j += angle_y;
    }
    false
}
