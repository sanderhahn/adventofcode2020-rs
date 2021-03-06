use std::{fs::File, io::BufRead, io::BufReader, io::Error};

fn parse_num(str: &str) -> u32 {
    let mut num = 0;
    for c in str.chars() {
        num <<= 1;
        num |= match c {
            'F' => 0,
            'B' => 1,
            'R' => 1,
            'L' => 0,
            c => panic!("invalid input {}", c),
        };
    }
    num
}

fn main() -> Result<(), Error> {
    let file = File::open("inputs/day5.txt")?;
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());

    assert_eq!(parse_num("FBFBBFFRLR"), 357);
    assert_eq!(parse_num("BFFFBBFRRR"), 567);
    assert_eq!(parse_num("FFFBBBFRRR"), 119);
    assert_eq!(parse_num("BBFFBBFRLL"), 820);

    let highest = lines.map(|line| parse_num(&line)).fold(0, |a, b| a.max(b));
    println!("{}", highest);
    Ok(())
}
