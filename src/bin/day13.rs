use aoc20::utils::get_input;

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MyResult<()> {
    let input = get_input()?;

    println!("Exo 1: {}", exo1(&input)?);
    println!("Exo 2: {}", exo2(&input)?);
    Ok(())
}

fn exo1(input: &String) -> MyResult<u64> {
    let time = input.lines().nth(0).unwrap().parse::<u64>().unwrap();
    let ids = input
        .trim()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .filter(|x| *x != "x")
        .map(|v| v.parse::<u64>().unwrap());

    let dist = ids
        .clone()
        .map(|id| ((id - (time % id)), id))
        .min()
        .unwrap();
    Ok(dist.0 * dist.1)
}

fn exo2(input: &String) -> MyResult<i64> {
    let ids: Vec<(i64, usize)> = input
        .trim()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(c, v)| (v.parse::<i64>().unwrap(), c))
        .collect();
    let n: i64 = ids.iter().map(|v| v.0).product();
    let mut res = 0;

    for (ni, ai) in ids {
        let n_chap: i64 = n / ni;
        let (_, _, v) = extended_gcd(ni, n_chap);
        let e = v * n_chap;
        println!("ni : {}, n_chap : {}, v : {}, e : {}", ni, n_chap, v, e);
        res += e * (-(ai as i64));
    }
    res = ((res % n) + n) % n;

    Ok(res)
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a.abs(), a.signum(), 0)
    } else {
        let (d, coef_b, coef_a) = extended_gcd(b, a % b);
        (d, coef_a, coef_b - coef_a * (a / b))
    }
}
