use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let curr_time: i32 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let (id, remain) = input.trim().split(",").filter(|&x| x != "x").map(|x| {
        let period: i32 = x.parse().unwrap();
        (period, (-curr_time).rem_euclid(period))
    }).min_by_key(|&(_, remain)| remain).unwrap();

    println!("{}", id * remain);
}
