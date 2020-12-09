use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_file(file_name: &Path) -> Vec<i64> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let lines = io::BufReader::new(file).lines();

    return lines.map(|line| line.unwrap().parse::<i64>().unwrap())
                .collect();
}

fn check_window(window: &[i64], target_number: i64) -> bool {
    for i in 0..window.len() {
        for j in i..window.len() {
            if window[i] + window[j] == target_number {
                return true;
            }
        }
    }

    return false;
}

fn find_first_invalid_number(numbers: &Vec<i64>, window_size: usize) -> Option<i64> { 
    for i in window_size..numbers.len() {
        let window = &numbers[(i - window_size)..i];
        let target_number = numbers[i];

        if !check_window(window, target_number) {
            return Some(target_number);
        }
    }

    return None;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let numbers = parse_file(&input_file);
    let first_invalid_number = find_first_invalid_number(&numbers, 25);
    
    print!("Numbers: {:?}", first_invalid_number.unwrap());
}
