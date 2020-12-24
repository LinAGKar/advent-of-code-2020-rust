use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut black = std::collections::HashSet::new();
    for line in input.lines() {
        let (x, y, _) = line.chars().fold((0, 0, None), |(x, y, previous), character| {
            match character {
                'n' => (x, y, Some('n')),

                's' => (x, y, Some('s')),

                'e' => match previous {
                    Some('n') => (x, y + 1, None),
                    Some('s') => (x - 1, y, None),
                    Some(_) => (x, y, None),
                    None => (x - 1, y + 1, None),
                },

                'w' => match previous {
                    Some('n') => (x + 1, y, None),
                    Some('s') => (x, y - 1, None),
                    Some(_) => (x, y, None),
                    None => (x + 1, y - 1, None),
                },

                _ => (x, y, previous)
            }
        });

        if black.contains(&(x, y)) {
            black.remove(&(x, y));
        } else {
            black.insert((x, y));
        }
    }

    println!("{}", black.len());
}
