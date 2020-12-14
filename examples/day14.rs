use std::collections::HashMap;

fn parse_mask(input: &str) -> (usize, usize) {
    let mut mask = 0;
    let mut bits = 0;
    for &c in input.as_bytes() {
        mask <<= 1;
        bits <<= 1;
        match c {
            b'X' => {},
            b'0' => {
                mask |= 1;
            },
            b'1' => {
                mask |= 1;
                bits |= 1;
            },
            _ => panic!("error")
        }
    }
    (mask, bits)
}

fn parse_mem(address: &str, number: &str) -> (usize, usize) {
    (address.parse().unwrap(), number.parse().unwrap())
}

fn apply_mask(number: usize, mask: (usize, usize)) -> usize {
    let number_keep = number & (!mask.0);
    let mask_keep = mask.1 & mask.0;
    number_keep | mask_keep
}

const MASK: &str = "mask = ";
const MEM_START: &str = "mem[";
const MEM_END: &str = "] = ";

fn day14a(input: &str) -> usize {
    let mut memory = HashMap::new();
    let mut mask: (usize, usize) = (0, 0);
    for line in input.lines() {
        if line.starts_with(MASK) {
            mask = parse_mask(&line[MASK.len()..]);
        } else if line.starts_with(MEM_START) {
            let split = line.find(MEM_END).unwrap();
            let address = &line[MEM_START.len()..split];
            let number = &line[split+MEM_END.len()..];
            let (address, number) = parse_mem(address, number);
            let number = apply_mask(number, mask);
            memory.insert(address, number);
        } else {
            panic!("error");
        }
    }
    let mut sum = 0;
    for &number in memory.values() {
        sum += number;
    }
    sum
}

fn parse_mask_b(input: &str) -> (usize, Vec<usize>) {
    let mut mask = 0;
    let mut bits: Vec<usize> = vec![];
    for &c in input.as_bytes() {
        mask <<= 1;
        match c {
            b'X' => {},
            b'0' => {
            },
            b'1' => {
                mask |= 1;
            },
            _ => panic!("error")
        }
    }
    let mut bit = 0;
    for &c in input.as_bytes().iter().rev() {
        match c {
            b'X' => {
                bits.push(bit);
            },
            _ => {}
        }
        bit += 1;
    }
    (mask, bits)
}

fn addresses(mut address: usize, bits: &Vec<usize>) -> Vec<usize> {
    let mut addresses = vec![];
    let mut mask = 0;
    for i in bits {
        mask = mask | (1 << i);
    }
    address = address & (!mask);
    addresses.push(address);
    for i in 0..bits.len() {
        let mask = 1 << bits[i];
        for i in 0..addresses.len() {
            addresses.push(addresses[i] | mask);
        }
    }
    addresses
}

fn apply_addressing(memory: &mut HashMap<usize, usize>, address: usize, number: usize, mask: &(usize, Vec<usize>)) {
    let address = address | mask.0;
    for address in addresses(address, &mask.1) {
        memory.insert(address, number);
    }
}

fn day14b(input: &str) -> usize {
    let mut memory = HashMap::new();
    let mut mask: (usize, Vec<usize>) = (0, vec![]);
    for line in input.lines() {
        if line.starts_with(MASK) {
            mask = parse_mask_b(&line[MASK.len()..]);
        } else if line.starts_with(MEM_START) {
            let split = line.find(MEM_END).unwrap();
            let address = &line[MEM_START.len()..split];
            let number = &line[split+MEM_END.len()..];
            let (address, number) = parse_mem(address, number);
            apply_addressing(&mut memory, address, number, &mask);
        } else {
            panic!("error");
        }
    }
    let mut sum = 0;
    for &number in memory.values() {
        sum += number;
    }
    sum
}

fn main() {
    let example = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    assert_eq!(165, day14a(example));

    let input = include_str!("../inputs/day14.txt");
    println!("{}", day14a(input));

    let example = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    assert_eq!(208, day14b(example));

    println!("{}", day14b(input));
}
