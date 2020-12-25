fn calc(subject_number: usize, loop_size: usize) -> usize {
    let mut num = 1;
    for _ in 0..loop_size {
        num *= subject_number;
        num %= 20201227;
    }
    num
}

fn find_secret(public_key: usize) -> usize {
    let mut secret = 1;
    let mut secret_key = 1;
    loop {
        secret_key *= 7;
        secret_key %= 20201227;
        if secret_key == public_key {
            return secret;
        }
        secret += 1;
    }
}

fn find_secret_loopsize(public_key1: usize, public_key2: usize) -> usize {
    let private_key1 = find_secret(public_key1);
    let private_key2 = find_secret(public_key2);

    let mut secret = 1;
    let mut secret_handshake1 = 1;
    let mut secret_handshake2 = 1;
    loop {
        secret_handshake1 *= private_key1;
        secret_handshake1 %= 20201227;
        secret_handshake2 *= private_key2;
        secret_handshake2 %= 20201227;

        if secret_handshake1 == secret_handshake2 {
            return secret;
        }
        secret += 1;
    }
}

fn main() {
    let card_public = 5764801;
    let door_public = 17807724;

    assert_eq!(find_secret(card_public), 8);
    assert_eq!(find_secret(door_public), 11);

    assert_eq!(find_secret_loopsize(card_public, door_public), 20201226);
    assert_eq!(calc(card_public, find_secret(door_public)), 14897079);
    assert_eq!(calc(door_public, find_secret(card_public)), 14897079);

    let card_public = 335121;
    let door_public = 363891;
    println!("{}", calc(card_public, find_secret(door_public)));
}
