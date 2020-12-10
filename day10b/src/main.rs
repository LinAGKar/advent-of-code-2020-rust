use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut joltages: HashSet<u8> = input.lines().map(|x| x.parse().unwrap()).collect();
    let device_joltage = joltages.iter().max().unwrap() + 3;
    joltages.insert(0);
    joltages.insert(device_joltage);

    let (_, (enabled_states, _)) = (0..=device_joltage).fold(
        (1u64, (0, 0)),
        |(last_dis, (last_en, sec_last_en)), x| {
            (last_en + sec_last_en, (if joltages.contains(&x) {
                last_dis + last_en
            } else {
                0
            }, last_en))
        }
    );
    println!("{}", enabled_states);
}
