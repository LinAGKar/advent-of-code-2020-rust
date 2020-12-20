use std::io;
use std::io::Read;

fn find_positions(
    tiles: &Vec<(u32, Vec<Vec<u16>>)>,
    side_len: usize,
    used_orientations: &mut Vec<(usize, usize)>,
) -> Option<u64> {
    let row = used_orientations.len() / side_len;
    let col = used_orientations.len() % side_len;
    for (n, (_, tile)) in tiles.iter().enumerate() {
        if used_orientations.iter().any(|&(m, _)| n == m) {
            continue;
        }
        for orientation in 0..8 {
            let flipped = orientation / 4;
            if row > 0 {
                let (adjacent_index, adjacent_orientation) = used_orientations[(row - 1) * side_len + col];
                let adjacent_flipped = adjacent_orientation / 4;
                let (_, adjacent_tile) = &tiles[adjacent_index];
                if adjacent_tile[(adjacent_orientation + 2) % 4][1 - adjacent_flipped] != tile[orientation % 4][flipped] {
                    continue;
                }
            }
            if col > 0 {
                let (adjacent_index, adjacent_orientation) = used_orientations[row * side_len + col - 1];
                let adjacent_flipped = adjacent_orientation / 4;
                let (_, adjacent_tile) = &tiles[adjacent_index];
                if adjacent_tile[(adjacent_orientation + 1 + adjacent_flipped * 2) % 4][1 - adjacent_flipped] != tile[(orientation + 3 + flipped * 2) % 4][flipped] {
                    continue;
                }
            }
            used_orientations.push((n, orientation));
            if used_orientations.len() == tiles.len() {
                return Some([
                    0,
                    side_len - 1,
                    (side_len - 1) * side_len,
                    side_len * side_len - 1,
                ].iter().map(|&i| {
                    let (index, _) = used_orientations[i];
                    let (tile_id, _) = tiles[index];
                    tile_id as u64
                }).product());
            } else {
                let result = find_positions(tiles, side_len, used_orientations);
                if result.is_some() {
                    return result;
                }
            }
            used_orientations.pop();
        }
    }
    None
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let tiles: Vec<(u32, _)> = input.trim().split("\n\n").map(|text| {
        let mut lines = text.lines();
        let first_line = lines.next().unwrap();
        let side_len = text.lines().skip(1).count();
        let image: Vec<Vec<u16>> = lines.map(|line| {
            line.chars().map(|y| if y == '#' { 1 } else { 0 } ).collect()
        }).collect();

        let sides: Vec<Vec<u16>> = (0..4).map(|side| {
            [false, true].iter().map(|&reverse| {
                (0..side_len).map(|mut i| {
                    let mut col = i;
                    let mut row = 0;
                    for _ in 0..side {
                        let new_row = col;
                        col = side_len - row - 1;
                        row = new_row;
                    }
                    if !reverse {
                        i = side_len - i - 1;
                    }
                    image[row][col] << i
                }).sum()
            }).collect()
        }).collect();

        (
            first_line["Tile ".len()..first_line.len() - 1].parse().unwrap(),
            sides,
        )
    }).collect();

    let mut used_orientations = Vec::new();
    println!("{}", find_positions(&tiles, (tiles.len() as f64).sqrt() as usize, &mut used_orientations).unwrap());
}
