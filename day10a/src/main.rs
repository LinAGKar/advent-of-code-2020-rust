use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut joltages: Vec<u8> = input.lines().map(|x| x.parse().unwrap()).collect();
    joltages.push(0);  // Outlet
    joltages.sort_unstable();
    joltages.push(joltages[joltages.len() - 1] + 3);  // Device
    let (ones, threes) = joltages[1..].iter().zip(joltages.iter()).fold((0, 0), |(ones, threes), (a, b)| match a - b {
        1 => (ones + 1, threes),
        3 => (ones, threes + 1),
        _ => (ones, threes),
    });
    println!("{}", ones * threes);
}