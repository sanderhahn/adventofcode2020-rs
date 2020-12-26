use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let file = File::open("inputs/day1.txt")?;
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let mut nums: Vec<u32> = lines.map(|num| num.parse::<u32>().unwrap()).collect();
    nums.sort();

    let mut small = 0;
    let mut big = nums.len() - 1;
    while big > small {
        let sum = nums[small] + nums[big];
        if sum == 2020 {
            println!("{}", nums[small] * nums[big]);
            break;
        }
        if sum > 2020 {
            big -= 1;
        }
        if sum < 2020 {
            small += 1;
            big = nums.len() - 1;
        }
    }
    Ok(())
}
