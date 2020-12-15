use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    const ITERATIONS: usize = 30000000;

    let numbers: Vec<_> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();
    let (&last_number, rest) = numbers.split_last().unwrap();
    let mut number_timestamps = vec![None; ITERATIONS];
    for (&i, n) in rest.iter().zip(1..) {
        number_timestamps[i] = Some(n);
    }
    
    let (_, last_number) = (numbers.len()..ITERATIONS).fold(
        (number_timestamps, last_number),
        |(mut number_timestamps, last_number), curr_time| {
            let new_number = if let Some(last_time) = number_timestamps[last_number] {
                curr_time - last_time
            } else {
                0
            };
            number_timestamps[last_number] = Some(curr_time);
            (number_timestamps, new_number)
        },
    );

    println!("{}", last_number);
}
