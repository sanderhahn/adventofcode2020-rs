use std::{fs::File, io::BufRead, io::BufReader, io::Error, collections::HashSet};

fn main() -> Result<(), Error> {
    let file = File::open("inputs/day6.txt")?;
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let mut answers = 0;
    let mut group = HashSet::new();
    for line in lines {
        if line != "" {
            for ch in line.chars() {
                group.insert(ch);
            }
            continue;
        }
        answers += group.len();
        group.clear();
    }
    answers += group.len();
    println!("{}", answers);
    Ok(())
}
