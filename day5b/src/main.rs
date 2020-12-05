use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let passes: HashSet<u16> = input
        .lines()
        .map(|x| x.chars().rev().map(|y| match y {
            'F' => 0,
            'B' => 1,
            'L' => 0,
            'R' => 1,
            _ => 0,
        }).enumerate().fold(0, |acc, x| acc + (x.1 << x.0)))
        .collect();

    for i in 0..(1 << 10) - 1 {
        if passes.contains(&(i - 1)) && passes.contains(&(i + 1)) && !passes.contains(&i) {
            println!("{}", i);
            break;
        }
    }
}
