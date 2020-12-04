use std::{
    collections::HashMap, collections::HashSet, fs::File, io::BufRead, io::BufReader, io::Error,
};

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

    fn between(&self, name: &str, low: usize, high: usize) -> bool {
        if let Some(value) = self.map.get(name) {
            if let Ok(value) = value.parse::<usize>() {
                value >= low && value <= high
            } else {
                false
            }
        } else {
            false
        }
    }

    fn check_height(&self) -> bool {
        if let Some(value) = self.map.get("hgt") {
            let cm = value.ends_with("cm");
            let inch = value.ends_with("in");
            if let Ok(value) = value[..value.len() - 2].parse::<usize>() {
                if cm {
                    value >= 150 && value <= 193
                } else if inch {
                    value >= 59 && value <= 76
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn check_hair_color(&self) -> bool {
        if let Some(value) = self.map.get("hcl") {
            let value: &[u8] = value.as_bytes();
            let valid = value[1..]
                .into_iter()
                .fold(true, |acc, b| acc && matches!(b, b'0'..=b'9' | b'a'..=b'f'));
            value[0] == b'#' && value.len() == 7 && valid
        } else {
            false
        }
    }

    fn check_eye_color(&self) -> bool {
        let valid: Vec<String> = "amb blu brn gry grn hzl oth"
            .split_whitespace()
            .map(String::from)
            .collect();
        if let Some(value) = self.map.get("ecl") {
            valid.contains(value)
        } else {
            false
        }
    }

    fn check_passport_id(&self) -> bool {
        if let Some(value) = self.map.get("pid") {
            let value: &[u8] = value.as_bytes();
            let valid = value[1..]
                .into_iter()
                .fold(true, |acc, b| acc && matches!(*b, b'0'..=b'9'));
            value.len() == 9 && valid
        } else {
            false
        }
    }

    fn check(&self) -> bool {
        let valid_keys: Vec<String> = "ecl pid eyr hcl byr iyr cid hgt"
            .split_whitespace()
            .map(String::from)
            .collect();
        let mut valid_keys: HashSet<String> = valid_keys.into_iter().collect();
        valid_keys.remove("cid");
        let mut keys: HashSet<String> = self.map.keys().cloned().collect();
        keys.remove("cid");

        self.between("byr", 1920, 2002)
            && self.between("iyr", 2010, 2020)
            && self.between("eyr", 2020, 2030)
            && self.check_height()
            && self.check_hair_color()
            && self.check_eye_color()
            && self.check_passport_id()
            && keys == valid_keys
    }
}

fn main() -> Result<(), Error> {
    let file = File::open("day4.txt")?;
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
