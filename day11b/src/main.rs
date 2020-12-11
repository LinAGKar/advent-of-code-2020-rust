use std::collections::HashSet;
use std::io;
use std::io::Read;
use std::ops::{Add, Mul};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Pos {
    x: i8,
    y: i8,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Pos) -> Pos {
        Pos {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Mul<i8> for Pos {
    type Output = Self;

    fn mul(self, other: i8) -> Pos {
        Pos {x: self.x * other, y: self.y * other}
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let seats: HashSet<_> = input.lines().zip(0..).flat_map(|(line, y)| line.chars().zip(0..).filter_map(move |(tile, x)| {
        if tile == 'L' {
            Some(Pos {x: x, y: y})
        } else {
            None
        }
    })).collect();

    let right_edge = seats.iter().map(|pos| pos.x).max().unwrap();
    let bottom_edge = seats.iter().map(|pos| pos.y).max().unwrap();

    let mut filled_seats = HashSet::new();

    let adjacent: Vec<_> = (-1..=1)
        .flat_map(|x| (-1..=1)
            .filter(move |&y| (x, y) != (0, 0))
            .map(move |y| Pos {x: x, y: y})
        )
        .collect();

    let mut changed = true;

    while changed {
        let mut to_remove = Vec::new();
        let mut to_add = Vec::new();
        for &pos in seats.iter() {
            let adjacent_filled = adjacent.iter().filter(|&&delta| {
                (1..).find_map(|n| {
                    let tested_pos = pos + delta * n;
                    if filled_seats.contains(&tested_pos) {
                        Some(true)
                    } else if tested_pos.x < 0 || tested_pos.x > right_edge ||
                              tested_pos.y < 0 || tested_pos.y > bottom_edge ||
                              seats.contains(&tested_pos) {
                        Some(false)
                    } else {
                        None
                    }
                }).unwrap()
            }).count();
            let filled = filled_seats.contains(&pos);
            if adjacent_filled >= 5 && filled {
                to_remove.push(pos);
            } else if adjacent_filled == 0 && !filled {
                to_add.push(pos);
            }
        }
        changed = to_remove.len() > 0 || to_add.len() > 0;
        for i in to_remove {
            filled_seats.remove(&i);
        }
        for i in to_add {
            filled_seats.insert(i);
        }
    }
    println!("{}", filled_seats.len());
}
