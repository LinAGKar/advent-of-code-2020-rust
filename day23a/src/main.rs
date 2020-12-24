use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut cups = u64::from_str_radix(input.trim(), 16).unwrap();
    let cup_count = input.trim().len();
    let pick_up_mask = (0..cup_count - 4).fold(0, |acc, n| acc | (0xF << n * 4));
    let keep_masks: Vec<_> = (0..cup_count).map(|i| {
        (0..i).fold(0, |acc, j| acc | 0xF << j * 4)
    }).collect();

    for _ in 0..100 {
        let current_cup = cups >> (cup_count - 1) * 4;
        let removed_cups = cups >> (cup_count - 4) * 4 & 0xFFF;
        cups = current_cup << (cup_count - 4) * 4 | cups & pick_up_mask;
        let destination = (current_cup as usize - 1..current_cup as usize + cup_count - 1).rev().find_map(|cup| {
            let cup = cup % cup_count + 1;
            if let Some(pos) = (0..cup_count).find(|pos| cups >> pos * 4 & 0xF == cup as u64) {
                Some(pos)
            } else {
                None
            }
        }).unwrap();
        cups = cups & keep_masks[destination] |
               cups >> destination * 4 << (destination + 3) * 4 |
               removed_cups << destination * 4;
        cups = (cups & keep_masks[cup_count - 1]) << 4 | cups >> (cup_count - 1) * 4;
    }

    while cups & 0xF != 1 {
        cups = (cups & 0xF) << (cup_count - 1) * 4 | cups >> 4;
    }

    println!("{:X}", cups >> 4);
}
