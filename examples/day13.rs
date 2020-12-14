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

fn day13b(input: &str) -> usize {
    let mut lines = input.lines();
    let _ = lines.next();
    let mut busses: Vec<(usize, usize)> = lines
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .map(|(pos, bus)| {
            if bus == "x" {
                (pos, 0)
            } else {
                (pos, bus.parse().unwrap())
            }
        })
        .filter(|(_, bus)| *bus != 0)
        .collect();

    let mut step = 1;
    let mut t = 0;
    'l: loop {
        for i in 0..busses.len() {
            let (pos, bus) = busses[i];
            if (t + pos) % bus == 0 {
                step *= bus;
                busses.remove(i);
                if busses.len() == 0 {
                    return t;
                }
                continue 'l;
            }
        }
        t += step;
    }
}

fn main() {
    let input = include_str!("../inputs/day13.txt");

    let example = "\
939
7,13,x,x,59,x,31,19
";
    assert_eq!(295, day13a(example));

    println!("{}", day13a(input));

    let example = "\
1
17,x,13,19
";
    assert_eq!(3417, day13b(example));

    println!("{}", day13b(input));
}
