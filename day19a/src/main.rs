use std::collections::HashMap;
use std::io;
use std::io::Read;

use regex::Regex;

#[derive(Debug)]
enum Rule {
    CharRule(char),
    SuperRule(Vec<Vec<usize>>),
}

fn append_rule(regex_string: &mut String, index: usize, rules: &HashMap<usize, Rule>) {
    match rules.get(&index).unwrap() {
        Rule::CharRule(character) => {
            regex_string.push(*character);
        }

        Rule::SuperRule(options) => {
            if options.len() > 1 {
                regex_string.push_str("(?:");
            }
            for (n, subrules) in options.iter().enumerate() {
                if n > 0 {
                    regex_string.push('|');
                }
                for &subrule in subrules {
                    append_rule(regex_string, subrule, rules);
                }
            }
            if options.len() > 1 {
                regex_string.push(')');
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let rule_regex  = Regex::new("(?m)^(\\d+): (?:\"(\\w)\"|([\\d |]+))").unwrap();
    let rules: HashMap<usize, _> = rule_regex.captures_iter(&input).map(|captures| {
        let index = captures[1].parse().unwrap();
        if let Some(character) = captures.get(2) {
            (index, Rule::CharRule(character.as_str().chars().next().unwrap()))
        } else {
            let subrules = &captures[3];
            (index, Rule::SuperRule(subrules.split('|').map(|option| {
                option.split_whitespace().map(|subrule| subrule.parse().unwrap()).collect()
            }).collect()))
        }
    }).collect();

    let mut regex_string = "(?m)^".to_string();
    append_rule(&mut regex_string, 0, &rules);
    regex_string.push('$');

    let message_regex = Regex::new(&regex_string).unwrap();

    println!("{}", message_regex.captures_iter(&input).count());
}
