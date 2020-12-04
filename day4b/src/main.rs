use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Read;

use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let eye_colors: HashSet<&'static str> = [
        "amb", "blu", "brn", "gry", "grn", "hzl", "oth"
    ].iter().cloned().collect();
    let validate_year = |year: &str, start, end| 
        year.len() == 4 && year.parse().map_or(false, |x| start <= x && x <= end);
    let hcl_re = Regex::new(r"^#[\da-f]{6}$").unwrap();
    let hgt_re = Regex::new(r"^(\d+)(cm|in)$").unwrap();

    let expected: Vec<(&str, Box<dyn Fn(&&str) -> bool>)> = vec![
        ("byr", Box::new(|x| validate_year(x, 1920, 2002))),
        ("iyr", Box::new(|x| validate_year(x, 2010, 2020))),
        ("eyr", Box::new(|x| validate_year(x, 2020, 2030))),
        ("hgt", Box::new(|x| hgt_re.captures(x).map_or(false, |cap| {
            let height = cap[1].parse().unwrap();
            if &cap[2] == "cm" {
                150 <= height && height <= 193
            } else {
                59 <= height && height <= 76
            }
        }))),
        ("hcl", Box::new(|x| hcl_re.is_match(x))),
        ("ecl", Box::new(|x| eye_colors.contains(x))),
        ("pid", Box::new(|x| x.len() == 9 && x.chars().all(|y| y.is_numeric()))),
    ];

    println!("{}", input.split("\n\n").filter(|x| {
        let fields: HashMap<&str, &str> = x.split_whitespace().map(|y| {
            let mut iter = y.split(':');
            (iter.next().unwrap(), iter.next().unwrap())
        }).collect();
        expected.iter().all(|(y, f)| fields.get(y).map_or(false, f))
    }).count());
}
