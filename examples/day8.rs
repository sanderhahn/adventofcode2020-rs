use std::collections::HashSet;

fn mutate(prog: Vec<(&str, isize)>) -> Vec<Vec<(&str, isize)>> {
    let mut all = vec![];
    for (i, (op, _)) in prog.iter().enumerate() {
        match *op {
            "nop" => {
                let mut more = prog.clone();
                more[i].0 = "jmp";
                all.push(more);
            },
            "jmp" => {
                let mut more = prog.clone();
                more[i].0 = "nop";
                all.push(more);
            },
            "acc" => {},
            _ => {
                panic!("error");
            }
        }
    }
    all
}

fn parse(input: &str) -> Vec<(&str, isize)> {
    let prog: Vec<(&str, isize)> = input
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.split_whitespace().collect();
            let op = v[0];
            let val: isize = v[1].trim_start_matches("+").parse().unwrap();
            (op, val)
        })
        .collect();
    prog
}

fn run(prog: &Vec<(&str, isize)>) -> Result<isize, isize> {
    let mut executed = HashSet::new();
    let mut acc = 0;
    let mut pc = 0;
    while pc < prog.len() {
        if executed.contains(&pc) {
            return Err(acc)
        }
        let op = prog[pc];
        executed.insert(pc);
        match op.0 {
            "acc" => {
                acc += op.1;
            }
            "jmp" => {
                let npc = (pc as isize) + op.1 - 1;
                if npc >= 0 {
                    pc = npc as usize;
                }
            }
            "nop" => {
            }
            _ => {
                panic!("invalid op");
            }
        }
        pc += 1;
    }
    Ok(acc)
}

fn day8a(input: &str) -> isize {
    run(&parse(input)).unwrap_err()
}

fn day8b(input: &str) -> isize {
    let prog = parse(input);
    let mutations = mutate(prog);
    for mutation in mutations {
        if let Ok(result) = run(&mutation) {
            return result
        }
    }
    panic!("error");
}

fn main() {
    let input = include_str!("../inputs/day8.txt");

    let example = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";
    assert_eq!(5, day8a(example));

    println!("{}", day8a(input));

    assert_eq!(8, day8b(example));

    println!("{}", day8b(input));
}
