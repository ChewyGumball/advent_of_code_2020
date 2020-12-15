use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn parse_file(file_name: &Path) -> Vec<i64> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let mut lines = io::BufReader::new(file).lines();

    return lines.next().unwrap().unwrap().split(",").map(|num| num.parse::<i64>().unwrap()).collect();
}

fn play_game(start_numbers: &Vec<i64>, rounds: i64) -> i64 {
    let mut previous_numbers: HashMap<i64, i64> = HashMap::new();

    let mut current_turn = 1;

    for number in start_numbers {
        previous_numbers.entry(*number).or_insert(current_turn);
        current_turn += 1;
    }

    let mut previous_number = *start_numbers.last().unwrap();
    for i in current_turn..(rounds + 1) {
        if previous_numbers.contains_key(&previous_number) {
            let previous_turn_spoken = *previous_numbers.get(&previous_number).unwrap();
            previous_numbers.insert(previous_number, i - 1);
            
            previous_number = (i - 1) - previous_turn_spoken;
        } else {
            previous_numbers.insert(previous_number, i - 1);
            previous_number = 0;
        }

        if i % 1000000 == 0{
            println!("i: {}", i / 1000000);
        }
    }

    return previous_number;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let starting_numbers = parse_file(&input_file);
    let final_number = play_game(&starting_numbers, 30000000);
    println!("{}", final_number);
}