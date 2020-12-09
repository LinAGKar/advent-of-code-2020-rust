use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers: Vec<u64> = input.lines().map(|x| x.parse().unwrap()).collect();

    const WINDOW: usize = 25;

    let incorrect = numbers.iter().enumerate().skip(WINDOW).find_map(|(n, &x)| {
        if numbers.iter().enumerate().skip(n - WINDOW).take(WINDOW).any(|(m, &y)| {
            numbers.iter().skip(m + 1).take(n - m - 1).any(|&z| y != z && y + z == x)
        }) {
            None
        } else {
            Some(x)
        }
    }).unwrap();

    let (start, len) = (0..numbers.len()).find_map(|n| {
        numbers.iter().skip(n).enumerate().try_fold(0, |acc, (m, &x)| {
            let sum = acc + x;
            if sum == incorrect {
                Err(m + 1)
            } else {
                Ok(sum)
            }
        }).err().map(|x| (n, x))
    }).unwrap();

    println!(
        "{}",
        numbers.iter().skip(start).take(len).min().unwrap() +
        numbers.iter().skip(start).take(len).max().unwrap(),
    );
}
