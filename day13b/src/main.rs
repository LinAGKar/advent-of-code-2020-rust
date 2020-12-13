use std::io;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let (_, offset) = input
        .trim()
        .split(",")
        .enumerate()
        .filter(|&(_, x)| x != "x")
        .fold((1, 0), |(old_period, offset), (n, x)| {
            let new_period = x.parse().unwrap();
            (
                lcm(old_period, new_period),
                (0..).map(|y| old_period * y + offset).find(|y| (y + n as u64) % new_period == 0).unwrap(),
            )
        });

    println!("{}", offset);
}
