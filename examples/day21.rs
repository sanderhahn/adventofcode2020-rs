use std::collections::{HashMap, HashSet};

fn parse_rules(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    let mut rules = vec![];
    for line in input.lines() {
        const SPLIT: &str = " (contains ";
        let contains = line.find(SPLIT).unwrap();
        let values: Vec<&str> = line[0..contains].split(" ").collect();
        let keys: Vec<&str> = line[contains + SPLIT.len()..line.len() - 1]
            .split(", ")
            .collect();
        rules.push((keys.clone(), values.clone()));
    }
    rules
}

fn name_index<'s>(names: &mut Vec<&'s str>, name: &'s str) -> usize {
    if let Some(index) = names.iter().position(|&item| item == name) {
        index
    } else {
        let index = names.len();
        names.push(name);
        index
    }
}

type Mapping = HashMap<usize, usize>;
type Constraint = HashMap<usize, HashSet<usize>>;
type Constraints = Vec<Constraint>;

fn is_consistent(
    variable: usize,
    value: usize,
    mapping: &Mapping,
    constraints: &Constraints,
) -> bool {
    if mapping.contains_key(&variable) {
        return false;
    }
    if let Some(_) = mapping.values().position(|&v| v == value) {
        return false;
    }
    for constraint in constraints.iter() {
        if let Some(values) = constraint.get(&variable) {
            if !values.contains(&value) {
                return false;
            }
        }
    }
    true
}

fn solve(
    variables: usize,
    values: usize,
    mapping: Mapping,
    constraints: &Constraints,
) -> Vec<Mapping> {
    let mut solutions = vec![];
    let variable = mapping.len();
    for value in 0..values {
        if is_consistent(variable, value, &mapping, constraints) {
            let mut new_mapping = mapping.clone();
            new_mapping.insert(variable, value);
            if new_mapping.len() == variables {
                solutions.push(new_mapping);
            } else {
                solutions.extend(solve(variables, values, new_mapping, constraints));
            }
        }
    }
    solutions
}

fn day21(input: &str) -> (usize, String) {
    let name_rules = parse_rules(input);
    let mut variable_names = vec![];
    let mut value_names = vec![];

    let mut constraints = Constraints::default();
    for (keys, values) in name_rules.iter() {
        let mut constraint = Constraint::default();
        for key in keys {
            let key_index = name_index(&mut variable_names, key);

            let mut values_set = HashSet::new();
            for &value in values {
                let value_index = name_index(&mut value_names, value);
                values_set.insert(value_index);
            }
            constraint.insert(key_index, values_set);
        }
        constraints.push(constraint);
    }

    let solutions = solve(
        variable_names.len(),
        value_names.len(),
        Mapping::default(),
        &constraints,
    );

    let mut assigned: HashSet<usize> = HashSet::new();
    for solution in solutions.iter() {
        assigned.extend(solution.values());
    }

    let all: HashSet<usize> = (0..value_names.len()).collect();
    let free = all.difference(&assigned).cloned().collect();

    let mut count = 0;
    for constraint in constraints.iter() {
        let mut values = HashSet::new();
        for values_set in constraint.values() {
            values.extend(values_set);
        }
        let matches: Vec<usize> = values.intersection(&free).cloned().collect();
        count += matches.len();
    }
    assert_eq!(solutions.len(), 1);
    let solution = solutions.first().unwrap();
    let solution: HashMap<&str, &str> = solution
        .iter()
        .map(|(&k, &v)| (variable_names[k], value_names[v]))
        .collect();

    let mut sorted_variables: Vec<&str> = solution.keys().cloned().collect();
    sorted_variables.sort();
    let result: String = sorted_variables
        .iter()
        .map(|&k| solution[k])
        .collect::<Vec<&str>>()
        .join(",");

    (count, result)
}

fn main() {
    let example = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";
    assert_eq!(5, day21(example).0);

    let input = include_str!("../inputs/day21.txt");
    println!("{}", day21(input).0);

    println!("{}", day21(input).1);
}
