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
        .args(&["run", "--example", day, "--release"])
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
    expect("day01a", "838624")?;
    expect("day01b", "52764180")?;
    expect("day02a", "536")?;
    expect("day02b", "558")?;
    expect("day03a", "250")?;
    expect("day03b", "1592662500")?;
    expect("day04a", "190")?;
    expect("day04b", "121")?;
    expect("day05a", "864")?;
    expect("day05b", "739")?;
    expect("day06a", "6703")?;
    expect("day06b", "3430")?;
    expect("day07", "261\n3765")?;
    expect("day08", "1818\n631")?;
    expect("day09", "731031916\n93396727")?;
    expect("day10", "2368\n1727094849536")?;
    expect("day11", "2275\n2121")?;
    expect("day12", "590\n42013")?;
    expect("day13", "410\n600691418730595")?;
    expect("day14", "10035335144067\n3817372618036")?;
    expect("day15", "421\n436")?;
    expect("day16", "29019\n517827547723")?;
    expect("day17", "313\n2640")?;
    expect("day18", "8298263963837\n145575710203332")?;
    expect("day19", "285\n412")?;
    expect("day21", "2573\nbjpkhx,nsnqf,snhph,zmfqpn,qrbnjtj,dbhfd,thn,sthnsg")?;
    expect("day22", "32495\n32665")?;
    expect("day23", "49725386\n538935646702")?;
    expect("day24", "326\n3979")?;
    expect("day25", "9420461")?;
    Ok(())
}
