use std::{
    collections::{HashSet, VecDeque},
    io,
};

type GameState = (VecDeque<usize>, VecDeque<usize>);

fn parse_stacks(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut stacks = vec![];
    for lines in input.split("\n\n") {
        let mut stack = VecDeque::new();
        for (index, line) in lines.lines().enumerate() {
            if index > 0 {
                stack.push_back(line.parse::<usize>().unwrap());
            }
        }
        stacks.push(stack);
    }
    (stacks[0].clone(), stacks[1].clone())
}

fn score(stack: VecDeque<usize>) -> usize {
    let mut level = 1;
    let mut score = 0;
    for &card in stack.iter().rev() {
        score += card * level;
        level += 1;
    }
    score
}

fn day22a(input: &str) -> usize {
    let (mut lefts, mut rights) = parse_stacks(input);
    while !lefts.is_empty() && !rights.is_empty() {
        let left = lefts.pop_front().unwrap();
        let right = rights.pop_front().unwrap();
        let winner = left > right;
        if winner {
            lefts.push_back(left);
            lefts.push_back(right);
        } else {
            rights.push_back(right);
            rights.push_back(left);
        }
        // enter();
    }
    if lefts.len() > rights.len() {
        score(lefts)
    } else {
        score(rights)
    }
}

pub fn enter() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn recursive_game(
    lefts: &mut VecDeque<usize>,
    rights: &mut VecDeque<usize>,
    game_states: &mut HashSet<GameState>,
) -> bool {
    while !lefts.is_empty() && !rights.is_empty() {
        let left = lefts.pop_front().unwrap();
        let right = rights.pop_front().unwrap();
        let game_state = (lefts.clone(), rights.clone());
        if game_states.contains(&game_state) {
            return true;
        }
        let mut winner = left > right;
        if lefts.len() >= left && rights.len() >= right {
            let mut lefts = lefts.clone();
            while lefts.len() > left {
                lefts.pop_back();
            }
            let mut rights = rights.clone();
            while rights.len() > right {
                rights.pop_back();
            }
            winner = recursive_game(&mut lefts, &mut rights, &mut HashSet::new());
        }
        if winner {
            lefts.push_back(left);
            lefts.push_back(right);
        } else {
            rights.push_back(right);
            rights.push_back(left);
        }
        // enter();
        game_states.insert(game_state);
    }
    lefts.len() > rights.len()
}

pub fn day22b(input: &str) -> usize {
    let (mut lefts, mut rights) = parse_stacks(input);
    if recursive_game(&mut lefts, &mut rights, &mut HashSet::new()) {
        score(lefts)
    } else {
        score(rights)
    }
}

fn main() {
    let example = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";
    assert_eq!(306, day22a(example));

    let input = include_str!("../inputs/day22.txt");
    println!("{}", day22a(input));

    assert_eq!(291, day22b(example));
    println!("{}", day22b(input));
}
