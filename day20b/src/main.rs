use std::io;
use std::io::Read;

fn find_positions(
    tiles: &Vec<(Vec<Vec<char>>, Vec<Vec<u16>>)>,
    side_len: usize,
    used_orientations: &mut Vec<(usize, usize)>,
) -> bool {
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
                return true;
            } else {
                if find_positions(tiles, side_len, used_orientations) {
                    return true;
                }
            }
            used_orientations.pop();
        }
    }
    false
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let tiles: Vec<(Vec<Vec<char>>, Vec<Vec<u16>>)> = input.trim().split("\n\n").map(|text| {
        let side_len = text.lines().skip(1).count();
        let image: Vec<Vec<u16>> = text.lines().skip(1).map(|line| {
            line.chars().map(|y| if y == '#' { 1 } else { 0 } ).collect()
        }).collect();

        (
            image.iter().skip(1).take(side_len - 2).map(|row| {
                row.iter().skip(1).take(side_len - 2).map(|&pix| if pix > 0 { '#' } else { '.' }).collect()
            }).collect(),
            (0..4).map(|side| {
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
            }).collect(),
        )
    }).collect();

    let mut used_orientations = Vec::new();
    let outer_side_len = (tiles.len() as f64).sqrt() as usize;
    let inner_side_len = tiles[0].0.len();
    let full_side_len = outer_side_len * inner_side_len;
    if !find_positions(&tiles, outer_side_len, &mut used_orientations) {
        panic!("");
    }

    let mut full_image: Vec<Vec<char>> = (0..full_side_len).map(|row| {
        let outer_row = row / inner_side_len;
        let inner_row = row % inner_side_len;
        (0..full_side_len).map(|col| {
            let outer_col = col / inner_side_len;
            let mut inner_col = col % inner_side_len;
            let mut inner_row = inner_row;
            let (index, orientation) = used_orientations[outer_row * outer_side_len + outer_col];
            let flipped = orientation / 4 > 0;
            let tile = &tiles[index].0;
            if flipped {
                inner_col = inner_side_len - inner_col - 1;
            }
            for _ in 0..orientation % 4 {
                let new_inner_row = inner_col;
                inner_col = inner_side_len - inner_row - 1;
                inner_row = new_inner_row;
            }
            tile[inner_row][inner_col]
        }).collect()
    }).collect();


    let sea_monster = [
        "                  #",
        "#    ##    ##    ###",
        " #  #  #  #  #  #",
    ];

    let sea_monster_height = sea_monster.len();
    let sea_monster_width = sea_monster.iter().map(|line| line.len()).max().unwrap();

    let sea_monster: Vec<_> = sea_monster.iter().enumerate().flat_map(|(row, &line)| {
        line.chars().enumerate().filter_map(move |(col, pix)| if pix == '#' {
            Some((row, col))
        } else {
            None
        })
    }).collect();

    let mut monster_count = 0;

    for orientation in 0..8 {
        let size_wo_height = full_side_len - sea_monster_height + 1;
        let size_wo_width = full_side_len - sea_monster_width + 1;

        for row in 0..size_wo_height {
            for col in 0..size_wo_width {
                if sea_monster.iter().all(|(diff_row, diff_col)| {
                    full_image[row + diff_row][col + diff_col] != '.'
                }) {
                    for &(diff_row, diff_col) in sea_monster.iter() {
                        full_image[row + diff_row][col + diff_col] = 'O'
                    }
                    monster_count += 1;
                }
            }
        }

        if monster_count > 0 {
            break;
        }

        full_image = (0..full_side_len).map(|row| {
            (0..full_side_len).map(|col| {
                full_image[col][if orientation % 4 == 3 { row } else { full_side_len - row - 1 }]
            }).collect()
        }).collect();
    }

    // for row in &full_image {
    //     for pix in row {
    //         print!("{}", pix);
    //     }
    //     println!("");
    // }

    println!("{}", full_image.iter().flat_map(|x| x.iter()).filter(|&&x| x == '#').count());
}
