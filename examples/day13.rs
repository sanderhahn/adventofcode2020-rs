fn parse(input: &str) -> (usize, Vec<usize>) {
    let mut lines = input.lines();
    let time: usize = lines.next().unwrap().parse().unwrap();
    let nums: Vec<usize> = lines
        .next()
        .unwrap()
        .split(",")
        .filter(|&num| num != "x")
        .map(|num| num.parse().unwrap())
        .collect();
    (time, nums)
}

fn day13a(input: &str) -> usize {
    let (time, nums) = parse(input);

    let rests: Vec<(usize, usize)> = nums.iter().map(|&bus| (bus, bus - (time % bus))).collect();
    let mut bus: usize = std::usize::MIN;
    let mut diff: usize = std::usize::MAX;
    for (time_bus, time) in rests {
        if time <= diff {
            diff = time;
            bus = time_bus;
        }
    }
    if bus != 0 {
        return diff * bus;
    }
    panic!("error")
}

fn main() {
    let input = include_str!("../inputs/day13.txt");

    let example = "\
939
7,13,x,x,59,x,31,19
";
    assert_eq!(295, day13a(example));

    println!("{}", day13a(input));
}
