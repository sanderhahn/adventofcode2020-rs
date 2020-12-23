use std::{collections::HashMap, io};

pub fn enter() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

#[derive(Debug)]
struct Item {
    num: usize,
    next: usize,
}

#[derive(Debug)]
struct Ring {
    current: usize,
    items: Vec<Item>,
    empty: Vec<usize>,
    map: HashMap<usize, usize>,
}

impl Ring {
    fn new(num: usize) -> Self {
        let mut map = HashMap::new();
        map.insert(num, 0);
        Self {
            current: 0,
            items: vec![Item { num, next: 0 }],
            empty: vec![],
            map,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        print!("cups:");
        let mut here = self.current;
        loop {
            if here == self.current {
                print!(" ({})", self.items[here].num);
            } else {
                print!(" {}", self.items[here].num);
            }
            here = self.items[here].next;
            if here == self.current {
                break;
            }
        }
        println!()
    }

    fn current_num(&self) -> usize {
        self.items[self.current].num
    }

    fn insert_right(&mut self, num: usize) {
        let other = self.items[self.current].next;
        if let Some(unused) = self.empty.pop() {
            self.items[self.current].next = unused;
            self.map.insert(num, unused);
            self.items[unused].num = num;
            self.items[unused].next = other;
        } else {
            let pos = self.items.len();
            self.map.insert(num, pos);
            self.items[self.current].next = pos;
            self.items.push(Item { num, next: other });
        }
    }

    fn next(&mut self) {
        self.current = self.items[self.current].next
    }

    fn remove_right(&mut self) -> usize {
        let removal = self.items[self.current].next;
        self.items[self.current].next = self.items[removal].next;
        self.empty.push(removal);
        let num = self.items[removal].num;
        self.map.remove(&num);
        self.items[removal].next = std::usize::MAX;
        num
    }

    fn lookup(&mut self, num: usize) {
        if let Some(position) = self.map.get(&num) {
            self.current = *position;
        } else {
            panic!("error lookup {}", num);
        }
    }
}

fn decr(num: usize, max_cups: usize) -> usize {
    if num > 1 {
        num - 1
    } else {
        max_cups
    }
}

fn char_to_num(char: char) -> usize {
    char.to_digit(10).unwrap() as usize
}

fn day23(input: &str, max_cups: usize, max_moves: usize) -> String {
    let mut iter = input.chars().rev();

    let num = char_to_num(iter.next().unwrap());
    let mut ring = Ring::new(num);

    let mut start = num;

    for char in iter {
        let num = char_to_num(char);
        start = start.max(num);
        ring.insert_right(num);
    }
    if max_cups > start {
        for num in (start + 1..=max_cups).rev() {
            ring.insert_right(num);
        }
        ring.lookup(max_cups);
    }
    ring.next();

    for _m in 1..=max_moves {
        // if _m % (max_moves / 10) == 0 {
        //     println!("-- move {} --", _m);
        // }
        // ring.print();
        let pick1 = ring.remove_right();
        let pick2 = ring.remove_right();
        let pick3 = ring.remove_right();
        // println!("pick up: {}, {}, {}", pick1, pick2, pick3);
        // ring.print();
        let current = ring.current_num();
        // println!("current: {}", current);
        let mut destination = decr(current, max_cups);
        while destination == pick1 || destination == pick2 || destination == pick3 {
            destination = decr(destination, max_cups)
        }
        // println!("destination: {}", destination);

        let safepoint = ring.current;
        // dbg!(pick3,pick2,pick1);
        ring.lookup(destination);
        ring.insert_right(pick3);
        ring.insert_right(pick2);
        ring.insert_right(pick1);
        ring.current = safepoint;
        ring.next();

        // enter();
    }

    ring.lookup(1);
    ring.next();

    if max_cups <= 9 {
        // part 1
        let mut result = vec![];
        for _ in 1..9 {
            result.push(ring.current_num());
            ring.next();
        }
        result.iter().map(|&num| format!("{}", num)).collect()
    } else {
        // part 2
        let cup1 = ring.current_num();
        ring.next();
        let cup2 = ring.current_num();
        format!("{}", cup1 * cup2)
    }
}

fn main() {
    let example = "389125467";
    assert_eq!("67384529", day23(example, 9, 100));

    println!("{}", day23("716892543", 9, 100));

    const MAX_CUPS: usize = 1000000;
    const MAX_MOVES: usize = 10000000;

    assert_eq!("149245887792", day23(example, MAX_CUPS, MAX_MOVES));
    println!("{}", day23("716892543", MAX_CUPS, MAX_MOVES));
}
