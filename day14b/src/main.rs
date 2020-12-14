use std::collections::HashMap;
use std::io;
use std::io::Read;

use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(r"(?m)(?:^mask = ([01X]+)$)|(?:^mem\[(\d+)\] = (\d+)$)").unwrap();
    let mut mem = HashMap::new();
    let mut mask_dis = 0xFFFFFFFFF;
    let mut masks_en = Vec::new();

    for i in re.captures_iter(&input) {
        if let Some(mask) = i.get(1) {
            let mask_en = mask.as_str().chars().fold(0, |acc, x| {
                (acc << 1) | match x { '1' => 1, _ => 0 }
            });
            mask_dis = mask.as_str().chars().fold(0, |acc, x| {
                (acc << 1) | match x { 'X' => 0, _ => 1 }
            });
            let mask_float: Vec<_> = mask
                .as_str()
                .chars()
                .rev()
                .enumerate()
                .filter(|&(_, x)| x == 'X')
                .map(|(n, _)| 1 << n)
                .collect();
            masks_en = (0..1 << mask_float.len()).map(|x| {
                ((0..mask_float.len()).filter(|y| x & (1 << y) != 0 ).map(|y| mask_float[y]).sum::<u64>()) | mask_en
            }).collect();
        } else if let (Some(addr), Some(val)) = (i.get(2), i.get(3)) {
            let addr: u64 = addr.as_str().parse().unwrap();
            let val: u64 = val.as_str().parse().unwrap();
            for j in &masks_en {
                mem.insert((addr & mask_dis) | j, val);
            }
        }
    }

    println!("{}", mem.values().sum::<u64>());
}
