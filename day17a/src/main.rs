use std::io;
use std::io::Read;

struct Tile {
    enabled: bool,
    enabled_neighbors: u8,
    enabled_check_cycle: usize,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    const CYCLES: usize = 6;
    let lines = input.lines().count();
    let columns = input.lines().next().unwrap().chars().count();

    // 3D Grid for looking up whether a tile is enabled. The grid can can grow one position per cycle, so keep headroom
    // in each direction equal to the number of cycles.
    let mut tile_lookup: Vec<Vec<Vec<_>>> = (0..CYCLES * 2 + columns).map(|_| {
        (0..CYCLES * 2 + lines).map(|_| {
            // Inwards and outwards are mirror images, so no point in keeping both directions around
            (0..1 + CYCLES).map(|_| Tile {
                enabled: false,
                enabled_neighbors: 0,
                enabled_check_cycle: usize::MAX,
            }).collect()
        }).collect()
    }).collect();

    let mut enabled_tiles = Vec::new();

    // Enable tiles marked with #
    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character == '#' {
                let pos = (x + CYCLES, y + CYCLES, 0);
                tile_lookup[pos.0][pos.1][pos.2].enabled = true;
                enabled_tiles.push(pos);
            }
        }
    }

    for cycle in 0..CYCLES {
        // List of enabled tiles and their neighbors
        let mut touched_tiles = Vec::new();

        for &tile_pos in &enabled_tiles {
            // Iterate 0-2 and later subtract one.
            for x in 0..3 {
                for y in 0..3 {
                    // Don't try to go downward into the mirror if we're at the bottom layer
                    for z in if tile_pos.2 == 0 { 1 } else { 0 }..3 {
                        let neigh_pos = (tile_pos.0 + x - 1, tile_pos.1 + y - 1, tile_pos.2 + z - 1);
                        let tile = &mut tile_lookup[neigh_pos.0][neigh_pos.1][neigh_pos.2];
                        if tile.enabled_check_cycle != cycle {
                            tile.enabled_check_cycle = cycle;
                            // Count is from a previous cycle, and need to be reset
                            tile.enabled_neighbors = 0;
                            // Pushing position when resetting makes sure we don't add duplicates
                            touched_tiles.push(neigh_pos);
                        }
                        // Tile is not neighbor of itself
                        if neigh_pos != tile_pos {
                            tile.enabled_neighbors += if neigh_pos.2 == 0 && tile_pos.2 == 1 {
                                // If the current tile is above z-axis origin, and we are incrementing a tile at origin,
                                // we should increment it twice, since it should also be incremented by our mirror below
                                // origin, which is not included in the simulation.
                                2
                            } else {
                                1
                            };
                        }
                    }
                }
            }
        }

        enabled_tiles = touched_tiles.into_iter().filter(|&tile_pos| {
            let tile = &mut tile_lookup[tile_pos.0][tile_pos.1][tile_pos.2];
            let enabled = if tile.enabled {
                [2, 3].contains(&tile.enabled_neighbors)
            } else {
                tile.enabled_neighbors == 3
            };
            tile.enabled = enabled;
            enabled
        }).collect();
    }

    println!("{}", enabled_tiles.iter().map(|&(_, _, z)| {
        if z != 0 {
            // If this is above z-axis origin, we also need to count its mirror
            2
        } else {
            1
        }
    }).sum::<usize>());
}
