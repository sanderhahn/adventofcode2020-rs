fn parse(input: &str) -> Vec<usize> {
    let mut numbers: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    numbers.sort();
    let adapter = numbers.last().unwrap() + 3;
    numbers.push(adapter);
    numbers
}

fn day10a(input: &str) -> usize {
    let numbers = parse(input);
    let mut charge = 0;
    let mut diffs = vec![0, 0, 0];
    for number in numbers {
        let diff = number - charge;
        if diff > 3 {
            panic!("too large")
        }
        diffs[diff - 1] += 1;
        charge = number;
    }
    diffs[0] * diffs[2]
}

fn count(
    charge: usize,
    pos: usize,
    numbers: &Vec<usize>,
    counts: &mut Vec<usize>,
) -> usize {
    if pos >= numbers.len() {
        1
    } else if counts[pos] > 0 {
        counts[pos]
    } else {
        let mut total = 0;
        for pos in pos..pos + 3 {
            if let Some(&adaper) = numbers.get(pos) {
                if adaper - charge <= 3 {
                    total += count(adaper, pos + 1, numbers, counts)
                }
            }
        }
        counts[pos] = total;
        total
    }
}

fn day10b(input: &str) -> usize {
    let numbers = parse(input);
    let mut counts = vec![0; numbers.len()];
    count(0, 0, &numbers, &mut counts)
}

fn main() {
    let input = include_str!("../inputs/day10.txt");

    let example = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";
    assert_eq!(22 * 10, day10a(example));

    println!("{}", day10a(input));

    assert_eq!(19208, day10b(example));

    println!("{}", day10b(input));
}
