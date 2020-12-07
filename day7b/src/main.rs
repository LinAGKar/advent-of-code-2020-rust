use std::collections::HashMap;
use std::io;
use std::io::Read;

use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line_regex = Regex::new(r"(?m)^([a-z ]+) bags contain (.+)\.$", ).unwrap();
    let inner_regex = Regex::new(r"(\d+) ([a-z ]+) bags?", ).unwrap();
    let bag_tree: HashMap<String, Vec<(u32, String)>> = line_regex.captures_iter(&input).map(|x| {
        (x[1].to_string(), inner_regex.captures_iter(&x[2]).map(|y| {
            (y[1].parse().unwrap(), y[2].to_string())
        }).collect())
    }).collect();

    let mut visit_next: HashMap<&str, u32> = [("shiny gold", 1)].iter().cloned().collect();
    let mut total_count: u32 = 0;

    while visit_next.len() > 0 {
        visit_next = visit_next.into_iter().fold(HashMap::new(), |mut acc, (name, count)| {
            if let Some(children) = bag_tree.get(name) {
                acc = children.iter().fold(acc, |mut inner_acc, (child_count, child_name)| {
                    *inner_acc.entry(child_name).or_insert(0) += child_count * count;
                    inner_acc
                });
            }
            acc
        });
        total_count += visit_next.values().sum::<u32>();
    }
    println!("{}", total_count);
}
