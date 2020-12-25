const PUBLIC_SUBJECT_NUMBER: i64 = 7;
const DOOR_PUBLIC_KEY: i64 = 17115212;
const CARD_PUBLIC_KEY: i64 = 3667832;
const DIVISOR: i64 = 20201227;

// 7^k mod 20201227 = 17807724
// 7^j mod 20201227 = 5764801

fn main() {

    let mut loop_counter = 0;
    let mut current_value = 1;

    while current_value != CARD_PUBLIC_KEY {
        current_value *= PUBLIC_SUBJECT_NUMBER;
        current_value %= DIVISOR;
        loop_counter += 1;
    }

    println!("Card counter: {}", loop_counter);

    let mut current_encryption_value = 1;
    for i in 0..loop_counter {
        current_encryption_value *= DOOR_PUBLIC_KEY;
        current_encryption_value %= DIVISOR;
    }

    println!("Encryption Key: {}", current_encryption_value);
}
