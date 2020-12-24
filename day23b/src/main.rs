use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    // Changing this to usize works, but more than doubles the runtime
    type ItemType = u32;
    const FULL_CUP_COUNT: ItemType = 1000000;

    let initial_cups: Vec<_> = input.trim().chars().map(|digit| digit.to_digit(10).unwrap() as ItemType).collect();
    // Lookup table for the next cup after each cup
    let mut cups: Vec<_> = (0..FULL_CUP_COUNT + 1).map(|x| x + 1).collect();
    let mut current_cup = initial_cups[0];
    *cups.last_mut().unwrap() = current_cup;
    for (i, &cup) in initial_cups.iter().enumerate() {
        cups[cup as usize] = if i == initial_cups.len() - 1 {
            if FULL_CUP_COUNT as usize > initial_cups.len() {
                (initial_cups.len() + 1) as ItemType
            } else {
                current_cup
            }
        } else {
            initial_cups[i + 1]
        }
    }

    for _ in 0..10000000 {
        let first_removed = cups[current_cup as usize];
        let last_removed = (0..2).fold(first_removed, |acc, _| cups[acc as usize]);
        cups[current_cup as usize] = cups[last_removed as usize];

        let destination = (current_cup - 1..current_cup + FULL_CUP_COUNT - 1).rev().find_map(|i| {
            let cup = i % FULL_CUP_COUNT + 1;
            if (0..3).try_fold(first_removed, |acc, _| {
                if cup == acc {
                    Err(())
                } else {
                    Ok(cups[acc as usize])
                }
            }).is_ok() {
                Some(cup)
            } else {
                None
            }
        }).unwrap();
        cups[last_removed as usize] = cups[destination as usize];
        cups[destination as usize] = first_removed;
        current_cup = cups[current_cup as usize];
    }

    // For part 1
    // (0..FULL_CUP_COUNT - 1).fold(1, |acc, _| {
    //     let next = cups[acc as usize];
    //     print!("{}", next);
    //     next
    // });
    // println!("");

    println!("{}", (0..2).fold((1, 1), |(cup, product), _| {
        let next = cups[cup as usize];
        (next, product * next as u64)
    }).1);
}
