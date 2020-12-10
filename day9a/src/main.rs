use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers: Vec<u64> = input.lines().map(|x| x.parse().unwrap()).collect();

    const WINDOW: usize = 25;

    println!("{}", numbers.iter().enumerate().skip(WINDOW).find_map(|(n, &x)| {
        if numbers.iter().enumerate().skip(n - WINDOW).take(WINDOW).any(|(m, &y)| {
            numbers[m + 1..n].iter().any(|&z| y != z && y + z == x)
        }) {
            None
        } else {
            Some(x)
        }
    }).unwrap());
}
