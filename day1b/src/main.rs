use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers: Vec<u32> = input.lines().map(|x| x.parse().unwrap()).collect();
    'outer: for (n, i) in numbers.iter().enumerate() {
        for (m, j) in numbers.iter().enumerate().skip(n + 1) {
            for k in numbers.iter().skip(m + 1) {
                if i + j + k == 2020 {
                    println!("{}", i * j * k);
                    break 'outer;
                }
            }
        }
    }
}
