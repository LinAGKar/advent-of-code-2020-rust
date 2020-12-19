use std::boxed::Box;
use std::collections::BTreeMap;
use std::io;
use std::io::Read;
use std::iter::Iterator;

use regex::Regex;

#[derive(Debug)]
enum Rule {
    CharRule(char),
    SuperRule(Vec<Vec<u8>>),
}

enum RuleState<'a, 'b> {
    CharRule(Option<char>),
    SuperRule(
        Box<dyn Iterator<Item=&'a Vec<u8>> + 'a>,
        Option<Vec<(u8, Option<RuleChecker<'a, 'b>>)>>,
    ),
}

struct RuleChecker<'a, 'b> {
    state: RuleState<'a, 'b>,
    rules: &'a Vec<Option<Rule>>,
    string: &'b str,
}

impl<'a, 'b> RuleChecker<'a, 'b> {
    fn new(string: &'b str, index: u8, rules: &'a Vec<Option<Rule>>) -> RuleChecker<'a, 'b> {
        RuleChecker {
            state: match rules[index as usize].as_ref().unwrap() {
                Rule::CharRule(character) => {
                    RuleState::CharRule(Some(*character))
                }

                Rule::SuperRule(options) => {
                    RuleState::SuperRule(Box::new(options.iter()), None)
                }
            },
            rules: rules,
            string: string,
        }
    }

    fn increment_rule_checkers(
        checkers: &mut [(u8, Option<RuleChecker<'a, 'b>>)],
        rules: &'a Vec<Option<Rule>>,
    ) -> Option<&'b str> {
        let pos = checkers.len() - 1;
        loop {
            if let Some(checker) = &mut checkers[pos].1 {
                if let Some(string) = checker.next() {
                    return Some(string);
                }
            }

            if pos == 0 {
                return None;
            }

            if let Some(string) = RuleChecker::increment_rule_checkers(&mut checkers[0..pos], rules) {
                checkers[pos].1 = Some(RuleChecker::new(string, checkers[pos].0, rules));
            } else {
                return None;
            }
        }
    }
}

impl<'a, 'b> Iterator for RuleChecker<'a, 'b> {
    type Item = &'b str;

    fn next(&mut self) -> Option<Self::Item> {
        let string = self.string;
        let rules = self.rules;
        match &mut self.state {
            RuleState::CharRule(char_state) => {
                if let Some(character) = *char_state {
                    *char_state = None;
                    self.string.chars().next().map_or(None, |x| if x == character {
                        Some(&self.string[1..])
                    } else {
                        None
                    })
                } else {
                    None
                }
            }

            RuleState::SuperRule(options, option) => {
                loop {
                    if let Some(subrules) = option {
                        if let Some(string) = RuleChecker::increment_rule_checkers(subrules, self.rules) {
                            return Some(string);
                        } else {
                            *option = None;
                        }
                    } else {
                        if let Some(next_option) = options.next() {
                            *option = Some(next_option.into_iter().enumerate().map(|(n, &index)| {
                                (index, if n == 0 {
                                    Some(RuleChecker::new(string, index, rules))
                                } else {
                                    None
                                })
                            }).collect());
                        } else {
                            return None;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let rule_regex  = Regex::new("(?m)^(\\d+): (?:\"(\\w)\"|([\\d |]+))").unwrap();
    let rules_tree: BTreeMap<u8, _> = rule_regex.captures_iter(&input).map(|captures| {
        let index = captures[1].parse().unwrap();
        if let Some(character) = captures.get(2) {
            (index, Rule::CharRule(character.as_str().chars().next().unwrap()))
        } else {
            let subrules = if index == 8 {
                "42 | 42 8"
            } else if index == 11 {
                "42 31 | 42 11 31"
            } else {
                &captures[3]
            };
            (index, Rule::SuperRule(subrules.split('|').map(|option| {
                option.split_whitespace().map(|subrule| subrule.parse().unwrap()).collect()
            }).collect()))
        }
    }).collect();

    let mut rules = Vec::new();
    for (n, i) in rules_tree {
        while rules.len() < n as usize {
            rules.push(None);
        }
        rules.push(Some(i));
    }

    println!("{}", input.lines().filter(|line| {
        RuleChecker::new(line, 0, &rules).any(|string| string.len() == 0)
    }).count());
}
