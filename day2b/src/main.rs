use regex::Regex;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"(?m)^(\d+)-(\d+) (\w): (\w+)$", ).unwrap();
    let count = re.captures_iter(&input).filter(|x| {
        let (first, second, letter, password) = (&x[1], &x[2], &x[3], &x[4]);
        let (first, second) = (first.parse::<usize>().unwrap(), second.parse::<usize>().unwrap());
        let letter = letter.chars().next();
        let mut iter = password.chars();
        (iter.nth(first - 1) == letter) ^ (iter.nth(second - first - 1) == letter)
    }).count();
    println!("{}", count);
}
