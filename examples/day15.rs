use std::collections::HashMap;

fn day15(input: &str, limit: usize) -> isize {
    let numbers: Vec<isize> = input
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut encounters = HashMap::new();
    let mut speak = 0;
    let mut last_spoken = 0;
    for turn in 1..=limit {
        let iturn = turn as isize;
        if turn <= numbers.len() {
            speak = numbers[turn - 1];
        } else {
            if encounters.contains_key(&last_spoken) {
                speak = (iturn - 1) - encounters.get(&last_spoken).unwrap();
            } else {
                speak = 0;
            }
        }

        if turn > 1 {
            encounters.insert(last_spoken, iturn - 1);
        }
        last_spoken = speak;
    }
    speak
}

fn main() {
    assert_eq!(436, day15("0,3,6", 2020));

    println!("{}", day15("20,0,1,11,6,3", 2020));

    assert_eq!(175594, day15("0,3,6", 30000000));

    println!("{}", day15("20,0,1,11,6,3", 30000000));
}
