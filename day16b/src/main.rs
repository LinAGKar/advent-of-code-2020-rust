use std::io;
use std::io::Read;

use regex::Regex;

fn matches_rule(val: u16, rule: (u16, u16, u16, u16)) -> bool {
    (rule.0 <= val && val <= rule.1) || (rule.2 <= val && val <= rule.3)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let re_rule = Regex::new(r"(?m)^([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let re_your_ticket = Regex::new(r"(?m)^your ticket:\n((?:\d+,)*\d+)$").unwrap();
    let re_nearby_tickets = Regex::new(r"(?m)^nearby tickets:\n((?:\d+[,\n])*\d+)$").unwrap();

    let rules: Vec<_> = re_rule.captures_iter(&input).map(|rule| {(
        rule[1].to_string(),
        (rule[2].parse().unwrap(), rule[3].parse().unwrap(), rule[4].parse().unwrap(), rule[5].parse().unwrap()),
    )}).collect();

    let mut rules_matched = vec![false; rules.len()];

    let your_ticket: Vec<u16> = re_your_ticket
        .captures(&input)
        .unwrap()[1]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let tickets: Vec<_> = re_nearby_tickets.captures(&input).unwrap()[1].lines().filter_map(|x| {
        let values: Vec<_> = x.split(',').map(|y| y.parse().unwrap()).collect();
        if values.iter().all(|&y| {
            rules.iter().any(|&(_, z)| matches_rule(y, z))
        }) {
            Some(values)
        } else {
            None
        }
    }).chain([&your_ticket].iter().cloned().cloned()).collect();

    let mut ticket_fields: Vec<(_, Vec<_>)> = (0..your_ticket.len()).map(|x| {
        (None, tickets.iter().map(|y| y[x]).collect())
    }).collect();

    let mut product = 1;

    while !rules_matched.iter().enumerate().all(|(n, &x)| x || !rules[n].0.starts_with("departure")) {
        let new_matched: Vec<_> = ticket_fields.iter_mut().enumerate().filter_map(|(n, (matched_rules, values))| {
            let matched_rules: &mut Vec<_> = if let Some(matched) = matched_rules {
                matched
            } else {
                *matched_rules = Some(rules.iter().enumerate().filter_map(|(n, &(_, limits))| { 
                    if values.iter().all(|&y| matches_rule(y, limits)) {
                        Some(n)
                    } else {
                        None
                    }
                }).collect());
                matched_rules.as_mut().unwrap()
            };

            *matched_rules = matched_rules.iter().cloned().filter(|&x| !rules_matched[x]).collect();

            if matched_rules.len() == 1 {
                let rule_index = matched_rules[0];
                if rules[rule_index].0.starts_with("departure") {
                    product *= *values.last().unwrap() as u64;
                }
                rules_matched[rule_index] = true;
                Some(n)
            } else {
                None
            }
        }).collect();

        if new_matched.len() == 0 {
            panic!("Found no matches");
        }

        for &i in new_matched.iter().rev() {
            ticket_fields.remove(i);
        }
    }

    println!("{}", product);
}
