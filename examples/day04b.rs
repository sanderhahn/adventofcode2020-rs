use std::{
    collections::HashMap, fs::File, io::BufRead, io::BufReader, io::Error, ops::RangeInclusive,
};

fn between(value: &str, range: RangeInclusive<usize>) -> bool {
    let value = value.parse::<usize>().ok().unwrap_or_default();
    range.contains(&value)
}

fn check_height(value: &str) -> bool {
    if value.len() < 2 {
        return false;
    }
    let (num, suffix) = value.split_at(value.len() - 2);
    let num = num.parse::<usize>().unwrap_or(0);
    let range = match suffix {
        "cm" => 150..=193,
        "in" => 59..=76,
        _ => return false,
    };
    range.contains(&num)
}

fn check_hair_color(value: &str) -> bool {
    if value.len() != 7 {
        return false;
    }
    let value: &[u8] = value.as_bytes();
    let valid = value[1..]
        .into_iter()
        .fold(true, |acc, b| acc && matches!(b, b'0'..=b'9' | b'a'..=b'f'));
    value[0] == b'#' && valid
}

fn check_eye_color(value: &str) -> bool {
    let valid: Vec<&str> = "amb blu brn gry grn hzl oth".split_whitespace().collect();
    valid.contains(&value)
}

fn check_passport_id(value: &str) -> bool {
    if value.len() != 9 {
        return false;
    }
    let value: &[u8] = value.as_bytes();
    let valid = value[1..]
        .into_iter()
        .fold(true, |acc, b| acc && matches!(*b, b'0'..=b'9'));
    valid
}

fn validate(key: &str, value: &str) -> bool {
    match key {
        "byr" => between(value, 1920..=2002),
        "iyr" => between(value, 2010..=2020),
        "eyr" => between(value, 2020..=2030),
        "hgt" => check_height(value),
        "hcl" => check_hair_color(value),
        "ecl" => check_eye_color(value),
        "pid" => check_passport_id(value),
        "cid" => true,
        _ => false,
    }
}

struct Row {
    map: HashMap<String, String>,
}

impl Row {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn assign(&mut self, assignments: Vec<&str>) {
        for assignment in assignments {
            let parts: Vec<&str> = assignment.splitn(2, ":").collect();
            self.map.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    fn clear(&mut self) {
        self.map.clear()
    }

    fn check(&self) -> bool {
        let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
        for key in keys {
            if !validate(key, self.map.get(key).unwrap_or(&"".to_string())) {
                return false;
            }
        }
        return true;
    }
}

fn main() -> Result<(), Error> {
    let file = File::open("inputs/day4.txt")?;
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let lines: Vec<String> = lines.collect();
    let mut valid = 0;
    let mut row = Row::new();
    for line in lines {
        if line == "" {
            if row.check() {
                valid += 1;
            }
            row.clear();
        } else {
            let assignments: Vec<&str> = line.split_whitespace().collect();
            row.assign(assignments);
        }
    }
    println!("{}", valid);
    Ok(())
}
