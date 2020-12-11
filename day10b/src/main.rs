use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut joltages: HashSet<u8> = input.lines().map(|x| x.parse().unwrap()).collect();
    let device_joltage = joltages.iter().max().unwrap() + 3;
    joltages.insert(0);
    joltages.insert(device_joltage);

    let possibilities: VecDeque<u64> = (0..=device_joltage).fold(
        [0, 0, 1].iter().cloned().collect(),
        |mut acc, x| {
            acc.push_front(if joltages.contains(&x) {
                acc.iter().sum()
            } else {
                0
            });
            acc.truncate(3);
            acc
        }
    );

    println!("{}", possibilities.front().unwrap());
}
