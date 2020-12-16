use std::ops::RangeInclusive;

type Ranges = Vec<(String, Vec<RangeInclusive<usize>>)>;

fn parse_ranges(input: &str) -> Ranges {
    let mut named_ranges = vec![];
    for range in input.lines() {
        let colon = range.find(": ").unwrap();
        let name = &range[0..colon];
        let range: Vec<RangeInclusive<usize>> = range[colon + 2..]
            .split(" or ")
            .map(|range| {
                let range: Vec<usize> = range.split("-").map(|num| num.parse().unwrap()).collect();
                range[0]..=range[1]
            })
            .collect();
        named_ranges.push((name.to_string(), range));
    }
    named_ranges
}

fn parse_tickets(input: &str) -> Vec<Vec<usize>> {
    let mut nearby_tickets = input.lines();
    nearby_tickets.next();
    nearby_tickets
        .map(|line| {
            let numbers: Vec<usize> = line.split(",").map(|num| num.parse().unwrap()).collect();
            numbers
        })
        .collect()
}

fn parse(
    input: &str,
) -> (
    Ranges,
    Vec<usize>,
    Vec<Vec<usize>>,
) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let named_ranges = parse_ranges(parts[0]);
    let my_tickets = parse_tickets(parts[1]);
    let nearby_tickets = parse_tickets(parts[2]);
    (named_ranges, my_tickets[0].clone(), nearby_tickets)
}

fn ticket_options(
    ticket: &Vec<usize>,
    named_ranges: &Ranges,
) -> Vec<Vec<String>> {
    let mut options = vec![];
    for (index, &number) in ticket.iter().enumerate() {
        options.push(vec![]);
        for (name, ranges) in named_ranges {
            let valid = ranges.iter().fold(false, |acc, range| {
                acc || range.contains(&number)
            });
            if valid {
                options.get_mut(index).unwrap().push(name.into());
            }
        }
    }
    options
}

fn day16a(input: &str) -> usize {
    let (named_ranges, _, nearby_tickets) = parse(input);

    let mut errors = 0;
    for ticket in nearby_tickets {
        let options = ticket_options(&ticket, &named_ranges);
        for (index, options) in options.iter().enumerate() {
            if options.len() == 0 {
                errors += ticket[index];
            }
        }
    }
    errors
}

fn solution_contains(option: &String, solution: &Vec<(usize, String)>) -> bool {
    for (_, solution) in solution {
        if solution == option {
            return true;
        }
    }
    false
}

fn minimize_options(all_options: Vec<Vec<String>>) -> Vec<String> {
    let mut all_options: Vec<(usize, Vec<String>)> = all_options.into_iter().enumerate().collect();
    all_options.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    let mut solutions: Vec<Vec<(usize, String)>> = vec![vec![]];
    for (index, options) in all_options {
        let mut new_solutions = vec![];
        for option in options {
            for solution in &solutions {
                let mut new_solution = solution.clone();
                if solution_contains(&option, solution) {
                    continue;
                }
                new_solution.push((index, option.clone()));
                new_solutions.push(new_solution);
            }
        }
        solutions = new_solutions;
    }

    let solutions: Vec<Vec<String>> = solutions
        .iter()
        .map(|solution| {
            let mut solution = solution.clone();
            solution.sort_by(|a, b| a.0.cmp(&b.0));
            solution.iter().map(|(_, name)| name.clone()).collect()
        })
        .collect();

    solutions.first().unwrap().clone()
}

fn day16b(input: &str) -> Vec<(String, usize)> {
    let (named_ranges, my_ticket, nearby_tickets) = parse(input);

    let mut all_options: Vec<Vec<String>> = vec![vec![]; named_ranges.len()];
    'ticket: for ticket in nearby_tickets {
        let options = ticket_options(&ticket, &named_ranges);
        for field_options in &options {
            if field_options.len() == 0 {
                continue 'ticket;
            }
        }
        for (index, field_options) in options.iter().enumerate() {
            let all_option = all_options.get_mut(index).unwrap();
            if all_option.len() == 0 {
                *all_option = options[index].clone();
            } else {
                all_option.retain(|option| field_options.contains(option));
            }
        }
    }

    let solution = minimize_options(all_options);
    solution.into_iter().zip(my_ticket.clone()).collect()
}

fn day16b_final(input: &str) -> usize {
    day16b(input)
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, value)| value)
        .product()
}

fn main() {
    let input = include_str!("../inputs/day16.txt");

    let example = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";
    assert_eq!(71, day16a(example));

    println!("{}", day16a(input));

    let example = "\
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
";
    assert_eq!(
        vec![
            ("row".to_string(), 11),
            ("class".to_string(), 12),
            ("seat".to_string(), 13)
        ],
        day16b(example)
    );

    println!("{}", day16b_final(input));
}
