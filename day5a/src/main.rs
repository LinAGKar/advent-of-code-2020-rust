use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}",
        input
            .lines()
            .map(|l| l.bytes().fold(0, |a, b| a << 1
                | if b == b'B' || b == b'R' { 1 } else { 0 }))
            .max()
            .unwrap()

    );
}
