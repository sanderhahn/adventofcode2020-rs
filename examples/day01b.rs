use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let file = File::open("inputs/day1.txt")?;
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let mut nums: Vec<u32> = lines.map(|num| num.parse::<u32>().unwrap()).collect();
    nums.sort();

    for small in 0..nums.len() - 3 {
        for big in (small + 1..nums.len() - 1).rev() {
            for between in small + 1..big - 1 {
                let sum = nums[small] + nums[between] + nums[big];
                if sum == 2020 {
                    println!("{}", nums[small] * nums[between] * nums[big]);
                    break;
                }
            }
        }
    }
    Ok(())
}
