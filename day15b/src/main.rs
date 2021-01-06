use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    const ITERATIONS: i32 = 30000000;

    let numbers: Vec<_> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();
    let (&last_number, rest): (&i32, _) = numbers.split_last().unwrap();
    let mut last_number = last_number;
    let mut number_timestamps = vec![-1; ITERATIONS as usize];
    for (&i, n) in rest.iter().zip(1..) {
        number_timestamps[i as usize] = n;
    }

    for curr_time in numbers.len() as i32..ITERATIONS {
        let last_time = number_timestamps[last_number as usize];
        let new_number = if last_time >= 0 {
            curr_time - last_time
        } else {
            0
        };
        number_timestamps[last_number as usize] = curr_time;
        last_number = new_number;
    }

    println!("{}", last_number);
}
