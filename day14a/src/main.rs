use std::collections::HashMap;
use std::io;
use std::io::Read;

use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(r"(?m)(?:^mask = ([01X]+)$)|(?:^mem\[(\d+)\] = (\d+)$)").unwrap();
    let mut mem = HashMap::new();
    let mut mask_en = 0;
    let mut mask_dis = 0xFFFFFFFFF;

    for i in re.captures_iter(&input) {
        if let Some(mask) = i.get(1) {
            mask_en = mask.as_str().chars().fold(0, |acc, x| {
                (acc << 1) | match x { '1' => 1, _ => 0 }
            });
            mask_dis = mask.as_str().chars().fold(0, |acc, x| {
                (acc << 1) | match x { '0' => 0, _ => 1 }
            });
        } else if let (Some(addr), Some(val)) = (i.get(2), i.get(3)) {
            let addr: u64 = addr.as_str().parse().unwrap();
            let val: u64 = val.as_str().parse().unwrap();
            mem.insert(addr, (val & mask_dis) | mask_en);
        }
    }

    println!("{}", mem.values().sum::<u64>());
}
