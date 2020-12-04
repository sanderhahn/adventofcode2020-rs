use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() -> Result<(), std::io::Error> {
    let mut valid = 0;
    let file = File::open("inputs/day2.txt")?;
    let lines = io::BufReader::new(file).lines().map(|line| line.unwrap());
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let &[range, letter, password] = parts.as_slice() {
            let range: Vec<&str> = range.split("-").collect();
            if let &[start, end] = range.as_slice() {
                let start: usize = start.parse().unwrap();
                let end: usize = end.parse().unwrap();
                let letter = letter.chars().next().unwrap();
                let count =
                    password
                        .chars()
                        .fold(0, |acc, c| if c == letter { acc + 1 } else { acc });
                if count >= start && count <= end {
                    valid += 1;
                }
            }
        }
    }
    println!("{}", valid);
    Ok(())
}
