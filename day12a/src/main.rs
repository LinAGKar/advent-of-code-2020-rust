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

    let (pos, _) = input.lines().fold((Pos { e: 0, n: 0 }, Pos { e: 1, n: 0 }), |(mut pos, mut dir), line| {
        let mut line = line.chars();
        let action = line.next().unwrap();
        let number: i32 = line.collect::<String>().parse().unwrap();
        match action {
            'N' => { pos.n += number; },
            'S' => { pos.n -= number; },
            'E' => { pos.e += number; },
            'W' => { pos.e -= number; },
            'L' => { for _ in 0..number / 90 {
                let n = dir.e;
                dir.e = -dir.n;
                dir.n = n;
            } },
            'R' => { for _ in 0..number / 90 {
                let e = dir.n;
                dir.n = -dir.e;
                dir.e = e;
            } },
            'F' => { pos = pos + dir * number; },
            _ => {},
        }
        (pos, dir)
    });

    println!("{}", pos.e.abs() + pos.n.abs());
}
