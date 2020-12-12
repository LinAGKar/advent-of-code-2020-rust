use std::io;
use std::io::Read;
use std::ops::{Add, Mul};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Pos {
    e: i32,
    n: i32,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Pos) -> Pos {
        Pos {e: self.e + other.e, n: self.n + other.n}
    }
}

impl Mul<i32> for Pos {
    type Output = Self;

    fn mul(self, other: i32) -> Pos {
        Pos {e: self.e * other, n: self.n * other}
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (pos, _) = input.lines().fold((Pos { e: 0, n: 0 }, Pos { e: 10, n: 1 }), |(mut pos, mut wp), line| {
        let mut line = line.chars();
        let action = line.next().unwrap();
        let number: i32 = line.collect::<String>().parse().unwrap();
        match action {
            'N' => { wp.n += number; },
            'S' => { wp.n -= number; },
            'E' => { wp.e += number; },
            'W' => { wp.e -= number; },
            'L' => { for _ in 0..number / 90 {
                let n = wp.e;
                wp.e = -wp.n;
                wp.n = n;
            } },
            'R' => { for _ in 0..number / 90 {
                let e = wp.n;
                wp.n = -wp.e;
                wp.e = e;
            } },
            'F' => { pos = pos + wp * number; },
            _ => {},
        }
        (pos, wp)
    });

    println!("{}", pos.e.abs() + pos.n.abs());
}
