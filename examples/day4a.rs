use std::{
    collections::HashMap, collections::HashSet, fs::File, io::BufRead, io::BufReader, io::Error,
};

fn check(map: &HashMap<String, String>) -> bool {
    let valid_keys: Vec<String> = "ecl pid eyr hcl byr iyr cid hgt"
        .split_whitespace()
        .map(String::from)
        .collect();
    let mut valid_keys: HashSet<String> = valid_keys.into_iter().collect();
    valid_keys.remove("cid");

    let mut keys: HashSet<String> = map.keys().cloned().collect();
    keys.remove("cid");

    keys == valid_keys
}

fn main() -> Result<(), Error> {
    let file = File::open("inputs/day4.txt")?;
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let lines: Vec<String> = lines.collect();
    let mut valid = 0;
    let mut row = HashMap::new();
    for line in lines {
        if line == "" {
            if check(&row) {
                valid += 1;
            }
            row.clear();
        } else {
            let assignments: Vec<&str> = line.split_whitespace().collect();
            for assignment in assignments {
                let parts: Vec<&str> = assignment.splitn(2, ":").collect();
                row.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
    }
    println!("{}", valid);
    Ok(())
}
