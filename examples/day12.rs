fn parse(input: &str) -> Vec<(u8, isize)> {
    let instructions: Vec<(u8, isize)> = input
        .lines()
        .map(|line| (line.as_bytes()[0], line[1..].parse().unwrap()))
        .collect();
    instructions
}

fn day12a(input: &str) -> isize {
    let instructions = parse(input);
    let mut ship = (0isize, 0isize, 1usize);
    for mut instruction in instructions {
        if instruction.0 == b'F' {
            instruction.0 = b"NESW"[ship.2 % 4];
        }
        match instruction {
            (b'N', value) => ship.1 += value,
            (b'E', value) => ship.0 += value,
            (b'S', value) => ship.1 -= value,
            (b'W', value) => ship.0 -= value,
            (b'L', value) => ship.2 += ((360 - value) / 90) as usize,
            (b'R', value) => ship.2 += (value / 90) as usize,
            (b'F', value) => ship.0 -= value,
            _ => panic!("error"),
        }
    }
    ship.0.abs() + ship.1.abs()
}

fn day12b(input: &str) -> isize {
    let instructions = parse(input);
    let mut waypoint = (10isize, 1isize);
    let mut ship = (0isize, 0isize);
    for instruction in instructions {
        match instruction {
            (b'N', value) => waypoint.1 += value,
            (b'E', value) => waypoint.0 += value,
            (b'S', value) => waypoint.1 -= value,
            (b'W', value) => waypoint.0 -= value,
            (b'L', value) => {
                for _ in 0..value / 90 {
                    waypoint = (-waypoint.1, waypoint.0);
                }
            }
            (b'R', value) => {
                for _ in 0..value / 90 {
                    waypoint = (waypoint.1, -waypoint.0);
                }
            }
            (b'F', value) => {
                for _ in 0..value {
                    ship.0 += waypoint.0;
                    ship.1 += waypoint.1;
                }
            }
            _ => panic!("error"),
        }
    }
    ship.0.abs() + ship.1.abs()
}

fn main() {
    let input = include_str!("../inputs/day12.txt");

    let example = "\
F10
N3
F7
R90
F11
";
    assert_eq!(25, day12a(example));

    println!("{}", day12a(input));

    assert_eq!(286, day12b(example));

    println!("{}", day12b(input));
}
