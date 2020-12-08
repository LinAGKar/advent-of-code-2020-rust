use std::io;
use std::io::Read;

use regex::Regex;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    operation: Operation,
    argument: i32,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"(?m)^(\w+) ([+-]\d+)$", ).unwrap();
    let instructions: Vec<Instruction> = re.captures_iter(&input).filter_map(|x| {
        if let Some(operation) = match &x[1] {
            "acc" => Some(Operation::Acc),
            "jmp" => Some(Operation::Jmp),
            "nop" => Some(Operation::Nop),
            _ => None,
        } {
            Some(Instruction{
                operation: operation,
                argument: x[2].parse().unwrap(),
            })
        } else {
            None
        }
    }).collect();

    let mut executed = vec![false; instructions.len()];
    let mut acc = 0;
    let mut pc = 0;

    while !executed[pc] {
        executed[pc] = true;
        let instruction = instructions[pc];
        match instruction.operation {
            Operation::Acc => {
                acc += instruction.argument;
            }

            Operation::Jmp => {
                pc = (pc as isize + instruction.argument as isize - 1) as usize;
            }

            Operation::Nop => {}
        }
        pc += 1;
    }
    println!("{}", acc);
}
