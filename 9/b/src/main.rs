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

fn find_contiguous_region_summing_to(numbers: &Vec<i64>, target: i64) -> Option<&[i64]> {
    for i in 0..numbers.len() - 1 {
        for j in 1..(numbers.len() - i) {
            let range = &numbers[i..(i+j)];
            let sum: i64 = range.iter().sum();
            if sum == target {
                return Some(range);
            } else if sum > target {
                break;
            }
        }
    }

    return None;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let numbers = parse_file(&input_file);
    let first_invalid_number = find_first_invalid_number(&numbers, 25).unwrap();
    let summing_region = find_contiguous_region_summing_to(&numbers, first_invalid_number).unwrap();

    let min = summing_region.iter().min().unwrap();
    let max = summing_region.iter().max().unwrap();

    print!("Number: {}", min + max);
}
