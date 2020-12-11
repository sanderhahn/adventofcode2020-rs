use std::ops::RangeInclusive;

fn parse(input: &str) -> Vec<Vec<u8>>{
    let map: Vec<Vec<u8>> = input.lines().map(|line| line.into()).collect();
    map
}

pub fn dbg_map(map: &Vec<Vec<u8>>) {
    for row in map {
        for &chair in row {
            print!("{}", chair as char);
        }
        println!();
    }
}

fn around(v: usize, limit: usize) -> RangeInclusive<usize> {
    0.max(v.saturating_sub(1))..=(limit-1).min(v+1)
}

fn iterate(map: &mut Vec<Vec<u8>>) -> Option<usize> {
    let width = map[0].len();
    let height = map.len();
    let mut new_map = map.clone();
    let mut changed = false;
    let mut occupied = 0;
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == b'.' {
                continue;
            }
            if map[y][x] == b'#' {
                occupied += 1;
            }

            let mut adjecent = 0;
            for ty in around(y, height) {
                for tx in around(x, width) {
                    if ty == y && tx == x {
                        continue;
                    }
                    if map[ty][tx] == b'#' {
                        adjecent += 1;
                    }
                }
            }

            if map[y][x] == b'L' && adjecent == 0 {
                new_map[y][x] = b'#';
                changed = true;
            }

            if map[y][x] == b'#' && adjecent >= 4 {
                new_map[y][x] = b'L';
                changed = true;
            }
        }
    }
    if changed {
        *map = new_map;
        None
    } else {
        Some(occupied)
    }
}

fn day11a(input: &str) -> usize {
    let mut map = parse(input);
    loop {
        if let Some(occupied) = iterate(&mut map) {
            return occupied
        }
    }
}

fn iterate_b(map: &mut Vec<Vec<u8>>) -> Option<usize> {
    let width = map[0].len();
    let height = map.len();
    let mut new_map = map.clone();
    let mut changed = false;
    let mut occupied = 0;
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == b'.' {
                continue;
            }
            if map[y][x] == b'#' {
                occupied += 1;
            }

            let mut adjecent = 0;
            for dy in 0..=2 {
                for dx in 0..=2 {
                    if dy == 1 && dx == 1 {
                        continue;
                    }

                    let mut ty = y;
                    let mut tx = x;
                    loop {
                        if dy == 0 {
                            if ty == 0 {
                                break;
                            }
                            ty -= 1;
                        } else if dy == 2 {
                            ty += 1;
                            if ty >= height {
                                break;
                            }
                        }
                        if dx == 0 {
                            if tx == 0 {
                                break;
                            }
                            tx -= 1;
                        } else if dx == 2 {
                            tx += 1;
                            if tx >= width {
                                break;
                            }
                        }

                        if map[ty][tx] == b'#' {
                            adjecent += 1;
                            break;
                        }
                        if map[ty][tx] == b'L' {
                            break;
                        }
                    }
                }
            }

            if map[y][x] == b'L' && adjecent == 0 {
                new_map[y][x] = b'#';
                changed = true;
            }

            if map[y][x] == b'#' && adjecent >= 5 {
                new_map[y][x] = b'L';
                changed = true;
            }
        }
    }
    if changed {
        *map = new_map;
        None
    } else {
        Some(occupied)
    }
}

fn day11b(input: &str) -> usize {
    let mut map = parse(input);
    loop {
        if let Some(occupied) = iterate_b(&mut map) {
            return occupied
        }
    }
}

fn main() {
    let input = include_str!("../inputs/day11.txt");

    let example = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
    assert_eq!(37, day11a(example));

    println!("{}", day11a(input));

    assert_eq!(26, day11b(example));

    println!("{}", day11b(input));
}
