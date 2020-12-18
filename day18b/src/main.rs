use std::boxed::Box;
use std::io;
use std::io::Read;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mult,
}

#[derive(Debug)]
enum Token {
    Number(i64),
    Op(Op),
    Lbrace,
    Rbrace,
}

struct Lexer<'a> {
    source: Box<dyn Iterator<Item=char> + 'a>,
    next_char: Option<char>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut next_char) = self.next_char {
            while next_char.is_whitespace() {
                if let Some(new_char) = self.source.next() {
                    next_char = new_char;
                } else {
                    return None;
                }
            }

            match next_char {
                '(' => {
                    self.next_char = self.source.next();
                    Some(Token::Lbrace)
                }

                ')' => {
                    self.next_char = self.source.next();
                    Some(Token::Rbrace)
                }

                '+' => {
                    self.next_char = self.source.next();
                    Some(Token::Op(Op::Add))
                }

                '*' => {
                    self.next_char = self.source.next();
                    Some(Token::Op(Op::Mult))
                }

                _ => {
                    if !next_char.is_numeric() {
                        panic!("Syntax error {}", next_char);
                    }
                    let mut number: String = [next_char].iter().collect();
                    self.next_char = None;
                    while let Some(next_char) = self.source.next() {
                        if next_char.is_numeric() {
                            number.push(next_char);
                        } else {
                            self.next_char = Some(next_char);
                            break;
                        }
                    }
                    Some(Token::Number(number.parse().unwrap()))
                },
            }
        } else {
            None
        }
    }
}

impl<'a> Lexer<'a> {
    fn new(source: &str) -> Lexer {
        let mut iter = Box::new(source.chars());
        let next_char = iter.next();
        Lexer {
            source: iter,
            next_char: next_char,
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", input.lines().map(|line| {
        let mut tokens = Lexer::new(line);

        let mut stack = Vec::new();
        let mut to_mult = 1;
        let mut acc = 0;
        let mut saved_op = None;

        while let Some(token) = tokens.next() {
            match token {
                Token::Number(val) => {
                    if let Some(op) = saved_op {
                        match op {
                            Op::Add => {
                                acc += val;
                            }

                            Op::Mult => {
                                to_mult *= acc;
                                acc = val;
                            }
                        }
                        saved_op = None;
                    } else {
                        acc = val;
                    }
                }

                Token::Op(op) => {
                    saved_op = Some(op);
                }

                Token::Lbrace => {
                    stack.push((to_mult, acc, saved_op));
                    saved_op = None;
                    to_mult = 1;
                }

                Token::Rbrace => {
                    acc *= to_mult;

                    let (old_to_mult, old_acc, old_op) = stack.pop().unwrap();
                    to_mult = old_to_mult;
                    match old_op {
                        Some(Op::Add) => {
                            acc += old_acc;
                        }

                        Some(Op::Mult) => {
                            to_mult *= old_acc;
                        }

                        None => {}
                    }
                    saved_op = None;
                }
            };
        }
        acc *= to_mult;
        acc
    }).sum::<i64>());
}
