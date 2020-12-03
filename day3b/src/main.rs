use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|x| {
        x.chars().collect()
    }).collect();
    let mut product = 1u64;
    for (diff_x, diff_y) in &[
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ] {
        let mut x = 0;
        let mut count = 0;
        for i in (0..map.len()).step_by(*diff_y) {
            if map[i][x] == '#' {
                count += 1;
            }
            x = (x + diff_x) % map[0].len();
        }
        product *= count;
    }
    println!("{}", product);
}
