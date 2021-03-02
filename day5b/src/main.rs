use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut seats: Vec<_> = input
        .lines()
        .map(|l| {
            l.bytes().fold(0, |a, b| {
                a << 1 | if b == b'B' || b == b'R' { 1 } else { 0 }
            })
        })
        .collect();
    seats.sort();
 
    println!(
        "{}",
        seats
            .windows(2)
            .find(|s| s[1] - s[0] == 2)
            .map(|s| s[0] + 1)
            .unwrap()
    );
}
