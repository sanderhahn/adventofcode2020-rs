// Every cup has a unique label and you only need to keep track of the cup next to it.
// The circle of cups can be stored as an array of cup label to next cup label.
// The current pointer uses the cup label instead of a position.

fn decr(label: &mut usize, num_cups: usize) {
    if *label > 1 {
        *label -= 1;
    } else {
        *label = num_cups;
    }
}

fn game(input: &str, num_cups: usize, num_moves: usize) -> Vec<usize> {
    let mut next_cup: Vec<usize> = vec![0; num_cups+1];
    for i in 1..=num_cups {
        next_cup[i] = i+1;
    }
    next_cup[num_cups] = 1;

    let init: Vec<usize> = input.chars().map(|c| {
        c.to_digit(10).unwrap() as usize
    }).collect();

    for i in 0..init.len() {
        let cup = init[i];
        let next = if i < init.len()-1 {
            init[i+1]
        } else {
            init.len()+1
        };
        next_cup[cup] = next;
    }
    if init.len() < num_cups {
        next_cup[num_cups] = init[0];
    } else {
        next_cup[*init.last().unwrap()] = init[0];
    }
    // dbg!(&init);

    fn print(mut current: usize, next_cup: &Vec<usize>) {
        print!("cups: ({})", &current);
        for _ in 1..next_cup.len() {
            current = next_cup[current];
            print!(" {}", &current);
        }
        println!();
    }

    let mut current = init[0];
    for _m in 1..=num_moves {
        // println!("move {}", _m);
        // print(current, &next_cup);

        let mut dest = current;
        decr(&mut dest, num_cups);
        let pick1 = next_cup[current];
        let pick2 = next_cup[pick1];
        let pick3 = next_cup[pick2];
        while dest == pick1 || dest == pick2 || dest == pick3 {
            decr(&mut dest, num_cups);
        }
        // println!("pick up: {}, {}, {}", pick1, pick2, pick3);
        // println!("destination: {}", dest);
        next_cup[current] = next_cup[pick3];
        next_cup[pick3] = pick1;
        let navigation = next_cup[current];

        let tmp = next_cup[dest];
        next_cup[dest] = pick1;
        next_cup[pick3] = tmp;

        current = navigation;
    }
    next_cup
}

fn day23a(input: &str) -> usize {
    let next_cup = game(input, 9, 100);
    let mut current = 1;
    let mut num = 0;
    for _ in 1..next_cup.len()-1 {
        current = next_cup[current];
        num *= 10;
        num += current;
    }
    num
}

fn day23b(input: &str) -> usize {
    let next_cup = game(input, 1000000, 10000000);
    let cup1 = next_cup[1];
    let cup2 = next_cup[cup1];
    cup1 * cup2
}

fn main() {
    let example = "389125467";
    assert_eq!(67384529, day23a(example));
    println!("{}", day23a("716892543"));

    assert_eq!(149245887792, day23b(example));
    println!("{}", day23b("716892543"));
}
