use std::{fs::File, io::BufRead, io::BufReader, io::Error, collections::HashSet};

fn main() -> Result<(), Error> {
    let file = File::open("inputs/day6.txt")?;
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let mut answers = 0;
    let mut new_group = true;
    let mut group_answers = HashSet::new();
    for line in lines {
        if line != "" {
            let mut answers = HashSet::new();
            for ch in line.chars() {
                answers.insert(ch);
            }
            if new_group {
                group_answers = answers.clone();
                new_group = false;
            } else {
                group_answers.retain(|answer| answers.contains(answer));
            }
            continue;
        } else {
            new_group = true;
            answers += group_answers.len();
            group_answers.clear();
        }
    }
    answers += group_answers.len();
    println!("{}", answers);
    Ok(())
}
