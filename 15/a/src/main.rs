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

fn increment_turns(numbers: &mut HashMap<i64, i64>) {
    for value in numbers.values_mut() {
        *value += 1;
    }
}

fn play_game(start_numbers: &Vec<i64>, rounds: usize) -> i64 {
    let mut previous_numbers: HashMap<i64, i64> = HashMap::new();

    for number in start_numbers {
        increment_turns(&mut previous_numbers);
        previous_numbers.entry(*number).or_insert(0);
    }

    let mut previous_number = *start_numbers.last().unwrap();
    for _ in start_numbers.len()..rounds {
        if previous_numbers.contains_key(&previous_number) {
            let new_number = *previous_numbers.get(&previous_number).unwrap();
            previous_numbers.insert(previous_number, 0);

            //println!("Previous Number: {}, New Number: {}", previous_number, new_number);
            previous_number = new_number;
        } else {
            previous_numbers.insert(previous_number, 0);
            previous_number = 0;
        }
        increment_turns(&mut previous_numbers);
    }

    return previous_number;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let starting_numbers = parse_file(&input_file);
    let final_number = play_game(&starting_numbers, 2020);
    println!("{}", final_number);
}