use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let expected = vec![
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
    ];
    println!("{}", input.split("\n\n").filter(|x| {
        let fields: HashSet<&str> = x.split_whitespace().map(|y| y.split(':').next().unwrap()).collect();
        expected.iter().all(|y| fields.contains(y))
    }).count());
}
