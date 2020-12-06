use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", input.split("\n\n").map(|x| {
        x.chars().filter(|y| y.is_ascii_lowercase()).collect::<HashSet<char>>().len()
    }).sum::<usize>());
}
