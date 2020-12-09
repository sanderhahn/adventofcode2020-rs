fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn find_sum(sum: usize, numbers: &[usize]) -> bool {
    for a in numbers.iter() {
        for b in numbers.iter() {
            if a + b == sum {
                return true;
            }
        }
    }
    false
}

fn day9a(input: &str, preamble: usize) -> usize {
    let numbers = parse(input);
    for index in preamble..numbers.len() {
        let sum = numbers[index];
        if !find_sum(sum, &numbers[index - preamble..index]) {
            return sum;
        }
    }
    panic!("error");
}

fn find_range(invalid: usize, numbers: &[usize]) -> usize {
    for start in 0..numbers.len() {
        for end in start..numbers.len() {
            let sum: usize = numbers[start..=end].iter().sum();
            if sum == invalid {
                let mut range: Vec<usize> = numbers[start..=end].iter().cloned().collect();
                range.sort();
                return range.first().unwrap() + range.last().unwrap();
            }
        }
    }
    panic!("error");
}

fn day9b(input: &str, preamble: usize) -> usize {
    let invalid = day9a(input, preamble);
    let numbers = parse(input);
    find_range(invalid, &numbers)
}

fn main() {
    let input = include_str!("../inputs/day9.txt");

    let example = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";
    assert_eq!(127, day9a(example, 5));

    println!("{}", day9a(input, 25));

    assert_eq!(62, day9b(example, 5));

    println!("{}", day9b(input, 25));
}
