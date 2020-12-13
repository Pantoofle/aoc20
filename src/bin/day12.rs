use aoc20::utils::get_input;

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MyResult<()> {
    let input = get_input()?;

    println!("Exo 1: {}", exo1(&input)?);
    println!("Exo 2: {}", exo2(&input)?);
    Ok(())
}

fn exo1(input: &String) -> MyResult<i64> {
    let mut pos: (i64, i64) = (0, 0);
    let mut dir = 0;

    for line in input.trim().lines() {
        let ins: char = line.chars().nth(0).unwrap();
        let val = line[1..].parse::<i64>().unwrap();
        match ins {
            'N' => pos.1 += val,
            'S' => pos.1 -= val,
            'E' => pos.0 += val,
            'W' => pos.0 -= val,
            'L' => dir = (dir + val + 360) % 360,
            'R' => dir = (dir - val + 360) % 360,
            'F' => match dir {
                0 => pos.0 += val,
                90 => pos.1 += val,
                180 => pos.0 -= val,
                270 => pos.1 -= val,
                _ => (),
            },
            _ => (),
        }
    }
    Ok(pos.0.abs() + pos.1.abs())
}

fn exo2(input: &String) -> MyResult<i64> {
    let mut pos: (i64, i64) = (0, 0);
    let mut waypoint: (i64, i64) = (10, 1);

    for line in input.trim().lines() {
        let ins: char = line.chars().nth(0).unwrap();
        let val = line[1..].parse::<i64>().unwrap();
        println!("{} -> Ins : {}, Val : {}", line, ins, val);
        println!("Old : {:?}, {:?}", pos, waypoint);
        match ins {
            'N' => waypoint.1 += val,
            'S' => waypoint.1 -= val,
            'E' => waypoint.0 += val,
            'W' => waypoint.0 -= val,
            'L' => waypoint = rotate(waypoint, val),
            'R' => waypoint = rotate(waypoint, -val),
            'F' => pos = (pos.0 + waypoint.0 * val, pos.1 + waypoint.1 * val),
            _ => (),
        }
        println!("New : {:?}, {:?}", pos, waypoint);
    }
    Ok(pos.0.abs() + pos.1.abs())
}

fn rotate(waypoint: (i64, i64), angle: i64) -> (i64, i64) {
    let angle = (angle + 360) % 360;
    match angle {
        0 => waypoint,
        90 => (-waypoint.1, waypoint.0),
        180 => (-waypoint.0, -waypoint.1),
        270 => (waypoint.1, -waypoint.0),
        _ => waypoint,
    }
}
