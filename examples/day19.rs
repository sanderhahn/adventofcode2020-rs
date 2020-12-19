use std::collections::HashMap;

use Rule::*;

#[derive(Clone, Debug)]
enum Rule {
    Seq(Seq),
    Alt(Box<Rule>, Box<Rule>),
    Char(u8),
}

type Rules = HashMap<usize, Rule>;
type Seq = Vec<usize>;

fn parse_rule(input: &str) -> Rule {
    if input.starts_with("\"") {
        return Rule::Char(input.as_bytes()[1]);
    }
    if let Some(pos) = input.find(" | ") {
        if let Seq(alt1) = parse_rule(&input[0..pos]) {
            if let Seq(alt2) = parse_rule(&input[pos+3..]) {
                return Rule::Alt(Box::new(Seq(alt1)), Box::new(Seq(alt2)));
            }
        }
        panic!("error");
    }
    let refs: Vec<usize> = input.split_ascii_whitespace().map(|num| num.parse().unwrap()).collect();
    Seq(refs)
}

fn parse_rules(input: &str) -> Rules {
    let mut rules = Rules::default();
    for line in input.lines() {
        let pos = line.find(": ").unwrap();
        let index: usize = line[0..pos].parse().unwrap();
        rules.insert(index, parse_rule(&line[pos+2..]));
    }
    rules
}

fn rule_match(position: usize, input: &str, rule: &Rule, rules: &Rules) -> Vec<usize> {
    match rule {
        Rule::Seq(seq) => {
            let mut options = vec![position];
            for rule in seq {
                let all: Vec<Vec<usize>> = options.iter().map(|&option| {
                    rule_match(option, input, &rules.get(rule).unwrap(), rules)
                }).collect();
                options = all.concat();
                if options.len() == 0 {
                    break;
                }
            }
            options
        }
        Alt(alt1, alt2) => {
            let mut alt1 = rule_match(position, input, alt1, rules);
            let mut alt2 = rule_match(position, input, alt2, rules);
            alt1.append(&mut alt2);
            alt1
        }
        Char(c) => {
            if position < input.len() && input.as_bytes()[position] == *c {
                vec![position+1]
            } else {
                vec![]
            }
        }
    }
}

fn day19(input: &str, extra_rules: Option<&str>) -> usize {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut rules = parse_rules(parts[0]);
    if let Some(extra_rules) = extra_rules {
        rules.extend(parse_rules(extra_rules));
    }
    let lines: Vec<&str> = parts[1].lines().collect();
    let mut matches = 0;
    for line in lines {
        let m = rule_match(0, line, rules.get(&0).unwrap(), &rules);
        if m.contains(&line.len()) {
            matches += 1;
        }
    }
    matches
}

fn main() {
    let example = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#;
    assert_eq!(2, day19(example, None));

    let input = include_str!("../inputs/day19.txt");
    println!("{}", day19(input, None));

    let example = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

    let extra_rules = "8: 42 | 42 8
11: 42 31 | 42 11 31";

    assert_eq!(12, day19(example, Some(extra_rules)));

    println!("{}", day19(input, Some(extra_rules)));
}
