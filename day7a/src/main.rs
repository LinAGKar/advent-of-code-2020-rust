use std::collections::HashSet;
use std::collections::HashMap;
use std::io;
use std::io::Read;

use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line_regex = Regex::new(r"(?m)^([a-z ]+) bags contain (.+)\.$", ).unwrap();
    let inner_regex = Regex::new(r"\d+ ([a-z ]+) bags?", ).unwrap();
    let bag_tree = line_regex.captures_iter(&input).fold(HashMap::new(), |acc, x| {
        inner_regex.captures_iter(&x[2]).fold(acc, |mut acc_inner, y| {
            acc_inner.entry(y[1].to_string()).or_insert(Vec::new()).push(x[1].to_string());
            acc_inner
        })
    });

    let mut visit_next: HashSet<&str> = ["shiny gold"].iter().cloned().collect();
    let mut visited: HashSet<&str> = visit_next.iter().cloned().collect();
    while visit_next.len() > 0 {
        visit_next = visit_next.into_iter().fold(HashSet::new(), |mut acc, x| {
            if let Some(parents) = bag_tree.get(x) {
                acc.extend(parents.iter().map(|y| y as &str).filter(|y| !visited.contains(y)));
            }
            acc
        });
        visited.extend(&visit_next);
    }
    println!("{}", visited.len() - 1);
}
