use std::io;
use std::io::Read;

use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq)]
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
    let orig_instructions: Vec<Instruction> = re.captures_iter(&input).filter_map(|x| {
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

    'outer_loop: for i in 0..orig_instructions.len() {
        if orig_instructions[i].operation == Operation::Acc {
            continue;
        }

        let mut instructions: Vec<Instruction> = orig_instructions.iter().cloned().collect();
        instructions[i].operation = match instructions[i].operation {
            Operation::Acc => Operation::Acc,
            Operation::Jmp => Operation::Nop,
            Operation::Nop => Operation::Jmp,
        };

        let mut executed = vec![false; instructions.len()];
        let mut acc = 0;
        let mut pc = 0;

        loop {
            if pc >= instructions.len() {
                println!("{}", acc);
                break 'outer_loop;
            } else if executed[pc] {
                continue 'outer_loop;
            }

            let instruction = instructions[pc];
            executed[pc] = true;
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
    }
}
