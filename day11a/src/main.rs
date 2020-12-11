use std::collections::HashSet;
use std::io;
use std::io::Read;
use std::ops::Add;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
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

    let mut filled_seats = HashSet::new();

    let adjacent: Vec<_> = (-1..=1)
        .flat_map(|x| (-1..=1)
            .filter(move |&y| (x, y) != (0, 0))
            .map(move |y| Pos {x: x, y: y})
        )
        .collect();

    let mut changed = true;
    let mut to_remove = Vec::new();
    let mut to_add = Vec::new();

    while changed {
        for &pos in seats.iter() {
            let adjacent_filled = adjacent.iter().filter(|&&delta| filled_seats.contains(&(pos + delta))).count();
            let filled = filled_seats.contains(&pos);
            if adjacent_filled >= 4 && filled {
                to_remove.push(pos);
            } else if adjacent_filled == 0 && !filled {
                to_add.push(pos);
            }
        }
        changed = to_remove.len() > 0 || to_add.len() > 0;
        for &i in &to_remove {
            filled_seats.remove(&i);
        }
        for &i in &to_add {
            filled_seats.insert(i);
        }
        to_remove.clear();
        to_add.clear();
    }
    println!("{}", filled_seats.len());
}
