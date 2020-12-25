use std::io::Read;

fn main() {
    const MOD: u32 = 20201227;
    const PUB_SUB: u32 = 7;

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let pub_keys: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();

    let loop_size = (1..).try_fold(1, |acc, i| {
        let val = acc * PUB_SUB % MOD;
        if val == pub_keys[0] {
            Err(i)
        } else {
            Ok(val)
        }
    }).err().unwrap();

    println!("{}", (0..loop_size).fold(1, |acc, _| acc * pub_keys[1] as u64 % MOD as u64));
}
