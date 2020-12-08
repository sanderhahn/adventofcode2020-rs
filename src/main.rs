use std::process::Command;
use std::str::from_utf8;

#[derive(Debug)]
struct Error(String);

impl Error {
    fn new(day: &str, expected: &str, got: String) -> Self {
        let message = format!("{} expected {} but got {}", day, expected, got);
        Self(message)
    }
}

fn run(day: &str) -> String {
    let output = Command::new("cargo")
        .args(&["run", "--example", day])
        .output()
        .unwrap()
        .stdout;
    let str = from_utf8(output.as_ref()).unwrap();
    str.trim_end().to_string()
}

fn expect(day: &str, expected: &str) -> Result<(), Error> {
    let output = run(day);
    if output != expected {
        Err(Error::new(day, expected, output))
    } else {
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    expect("day1a", "838624")?;
    expect("day1b", "52764180")?;
    expect("day2a", "536")?;
    expect("day2b", "558")?;
    expect("day3a", "250")?;
    expect("day3b", "1592662500")?;
    expect("day4a", "190")?;
    expect("day4b", "121")?;
    expect("day5a", "864")?;
    expect("day5b", "739")?;
    expect("day6a", "6703")?;
    expect("day6b", "3430")?;
    expect("day7", "261\n3765")?;
    expect("day8", "1818\n631")?;
    Ok(())
}
