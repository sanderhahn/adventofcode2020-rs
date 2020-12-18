#[derive(Debug)]

enum Oper {
    Num(usize),
    Op(char),
}
use Oper::*;

fn rpn(program: &Vec<Oper>) -> usize {
    let mut stack: Vec<usize> = vec![];
    for op in program {
        match op {
            Num(n) => {
                stack.push(*n);
            }
            Op(op) => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let result = match op {
                    '+' => left + right,
                    '*' => left * right,
                    _ => {
                        panic!("error");
                    }
                };
                stack.push(result);
            }
        }
    }
    stack.pop().unwrap()
}

fn parse(input: &str, part_b: bool) -> usize {
    let mut program = vec![];
    let mut ops = vec![];
    for c in input.chars() {
        match c {
            '0'..='9' => {
                let n = c.to_digit(10).unwrap() as usize;
                program.push(Num(n));
            }
            '+' | '*' => {
                if ops.len() >= 1 {
                    if part_b {
                        if ops.last() == Some(&'+') {
                            program.push(Op(ops.pop().unwrap()));
                        }
                    } else {
                        if ops.last() != Some(&'(') {
                            program.push(Op(ops.pop().unwrap()));
                        }
                    }
                }
                ops.push(c);
            }
            '(' => {
                ops.push(c);
            }
            ')' => {
                while ops.last() != Some(&'(') {
                    program.push(Op(ops.pop().unwrap()));
                }
                ops.pop().unwrap();
            }
            ' ' => {}
            e => {
                panic!("error {:?}", e)
            }
        }
    }
    while ops.len() > 0 {
        program.push(Op(ops.pop().unwrap()));
    }
    rpn(&program)
}

fn day18(input: &str, part_a: bool) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        sum += parse(line, part_a);
    }
    sum
}

fn main() {
    let input = include_str!("../inputs/day18.txt");

    let example = "\
1 + (2 * 3) + (4 * (5 + 6))
";
    assert_eq!(51, day18(example, false));

    println!("{}", day18(input, false));

    let example = "\
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
";
    assert_eq!(23340, day18(example, true));

    println!("{}", day18(input, true));
}
