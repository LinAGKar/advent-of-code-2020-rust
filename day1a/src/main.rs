use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers: Vec<u32> = input.lines().map(|x| x.parse().unwrap()).collect();
    'outer: for (n, i) in numbers.iter().enumerate() {
        for j in numbers.iter().skip(n + 1) {
            if i + j == 2020 {
                println!("{}", i * j);
                break 'outer;
            }
        }
    }
}
