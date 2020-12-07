use std::collections::{HashMap, HashSet};

type DepGraph = HashMap<String, Vec<(usize, String)>>;

fn parse_input(input: &str) -> DepGraph {
    let lines = input.lines();
    let mut deps: DepGraph = HashMap::new();
    for line in lines {
        let v: Vec<&str> = line.splitn(2, " bags contain ").collect();
        if let &[container, contains] = &v[..] {
            let container = container.trim_end_matches(" bag").trim_end_matches(" bags");
            let contains = contains.trim_end_matches(".");
            let contents = contains.split(", ");
            let mut vec = vec![];
            for content in contents {
                let v: Vec<&str> = content.splitn(2, " ").collect();
                if let &[num, bag] = &v[..] {
                    let bag = bag.trim_end_matches(" bag").trim_end_matches(" bags");
                    if num != "no" {
                        let num = num.parse().unwrap();
                        if num > 0 {
                            vec.push((num, bag.into()));
                        }
                    }
                }
            }
            deps.insert(container.into(), vec);
        }
    }
    deps
}

type InverseDeps = HashMap<String, Vec<String>>;

fn inverse_deps(deps: &DepGraph) -> InverseDeps {
    let mut inverse_deps: InverseDeps = HashMap::new();
    for (container, contents) in deps {
        for (_, bag) in contents {
            if !inverse_deps.contains_key(bag) {
                inverse_deps.insert(bag.into(), vec![]);
            }
            if let Some(deps) = inverse_deps.get_mut(bag) {
                deps.push(container.into());
            }
        }
    }
    inverse_deps
}

fn lookup(bag: &str, set: &mut HashSet<String>, inverse_deps: &InverseDeps) -> HashSet<String> {
    if let Some(deps) = inverse_deps.get(bag) {
        for dep in deps {
            if !set.contains(dep) {
                set.insert(dep.clone());
                lookup(&dep, set, inverse_deps);
            }
        }
    }
    set.clone()
}

fn day7a(input: &str) -> usize {
    let deps = parse_input(input);
    let inverse_deps = inverse_deps(&deps);

    let mut set = HashSet::new();
    set = lookup("shiny gold", &mut set, &inverse_deps);
    set.len()
}

fn count(bag: &str, deps: &DepGraph) -> usize {
    let mut total = 1;
    if let Some(contains) = deps.get(bag) {
        for (num, bag) in contains {
            total += num * count(bag, deps);
        }
    }
    total
}

fn day7b(input: &str) -> usize {
    let deps = parse_input(input);
    count("shiny gold", &deps) - 1
}

fn main() {
    let input = include_str!("../inputs/day7.txt");

    let example = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";
    assert_eq!(4, day7a(example));

    println!("{}", day7a(input));

    let example = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";
    assert_eq!(126, day7b(example));

    println!("{}", day7b(input));
}
