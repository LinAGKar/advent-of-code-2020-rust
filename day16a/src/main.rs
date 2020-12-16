use std::io;
use std::io::Read;

use regex::Regex;

fn matches_rule(val: u16, rule: (u16, u16, u16, u16)) -> bool {
    (rule.0 <= val && val <= rule.1) || (rule.2 <= val && val <= rule.3)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let re_rule = Regex::new(r"(?m)^[a-z ]+: (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let re_nearby_tickets = Regex::new(r"(?m)^nearby tickets:\n((?:\d+[,\n])*\d+)$").unwrap();

    let rules: Vec<(u16, u16, u16, u16)> = re_rule.captures_iter(&input).map(|rule| {
        (rule[1].parse().unwrap(), rule[2].parse().unwrap(), rule[3].parse().unwrap(), rule[4].parse().unwrap())
    }).collect();

    println!("{}", re_nearby_tickets.captures(&input).unwrap()[1].lines().filter_map(|x| {
        x.split(',').map(|y| y.parse().unwrap()).find(|&y| {
            !rules.iter().any(|&z| matches_rule(y, z))
        }).map(|x| x as u32)
    }).sum::<u32>());
}
