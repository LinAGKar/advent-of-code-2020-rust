use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", input.split("\n\n").map(|x| {
        x.lines()
         .map(|y| y.chars().collect::<HashSet<char>>())
         .fold(('a'..='z').collect::<HashSet<char>>(), |acc, y| acc.intersection(&y).cloned().collect())
        //  .fold_first(|acc, y| acc.intersection(&y).cloned().collect())
        //  .unwrap()
         .len()
    }).sum::<usize>());
}
