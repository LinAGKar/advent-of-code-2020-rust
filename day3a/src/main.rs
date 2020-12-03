use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|x| {
        x.chars().collect()
    }).collect();
    let mut x = 0;
    let mut count = 0;
    for i in 0..map.len() {
        if map[i][x] == '#' {
            count += 1;
        }
        x = (x + 3) % map[0].len();
    }
    println!("{}", count);
}
