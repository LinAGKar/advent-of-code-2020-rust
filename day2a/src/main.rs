use std::io;
use std::io::Read;

use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"(?m)^(\d+)-(\d+) (\w): (\w+)$", ).unwrap();
    let count = re.captures_iter(&input).filter(|x| {
        let (min, max, letter, password) = (&x[1], &x[2], &x[3], &x[4]);
        let (min, max) = (min.parse::<usize>().unwrap(), max.parse::<usize>().unwrap());
        let count = password.matches(letter).count();
        min <= count && count <= max
    }).count();
    println!("{}", count);
}
