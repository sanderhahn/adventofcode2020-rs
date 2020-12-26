use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Map {
    map: Vec<String>,
}

impl Map {
    fn new(map: Vec<String>) -> Self {
        Self { map: map }
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<bool> {
        let x = x % self.map[0].len();
        if y >= self.map.len() {
            None
        } else {
            let found = self.map[y].as_bytes()[x];
            Some(found == b'#')
        }
    }

    fn count_slope(&self, (right, down): (usize, usize)) -> usize {
        let mut pos = (0, 0);
        let mut trees = 0;
        loop {
            pos = (pos.0 + right, pos.1 + down);
            if let Some(found) = self.get(pos) {
                if found {
                    trees += 1;
                }
            } else {
                break;
            }
        }
        return trees;
    }
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("inputs/day3.txt")?;
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let map = Map::new(lines.collect());
    println!("{}", map.count_slope((3, 1)));
    Ok(())
}
