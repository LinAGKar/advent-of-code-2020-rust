use std::io::Read;

struct Tile {
    black: bool,
    neighbors: u8,
    timestamp: u8,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    const GRID_SIZE: usize = 150;
    let mut tiles: Vec<Vec<_>> = (0..GRID_SIZE).map(|_| {
        (0..GRID_SIZE).map(|_| Tile {
            black: false,
            neighbors: 0,
            timestamp: u8::MAX,
        }).collect()
    }).collect();

    let mut black = std::collections::HashSet::new();
    for line in input.lines() {
        let (x, y, _) = line.chars().fold((GRID_SIZE / 2, GRID_SIZE / 2, None), |(x, y, previous), character| {
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
        tiles[x][y].black = !tiles[x][y].black;
    }

    let mut black: Vec<_> = black.into_iter().collect();
    let mut touched = Vec::new();

    let neighbors = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, -1),
        (-1, 1),
    ];

    for i in 0..100 {
        for &(x, y) in &black {
            for (dx, dy) in neighbors.iter().chain([(0, 0)].iter()) {
                let (x, y) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
                let tile = &mut tiles[x][y];
                if tile.timestamp != i {
                    tile.timestamp = i;
                    tile.neighbors = 0;
                    touched.push((x, y));
                }
            }

            for (dx, dy) in &neighbors {
                let (x, y) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
                tiles[x][y].neighbors += 1;
            }
        }

        black = touched.drain(0..).filter(|&(x, y)| {
            let tile = &mut tiles[x][y];
            let black = if tile.black {
                tile.neighbors == 1 || tile.neighbors == 2
            } else {
                tile.neighbors == 2
            };
            tile.black = black;
            black
        }).collect();
    }

    println!("{}", black.len());
}
