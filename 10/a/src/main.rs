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

    let mut adapters: Vec<i64> = lines.map(|line| line.unwrap().parse::<i64>().unwrap())
                .collect();
    
    let max_joltage = *adapters.iter().max().unwrap();

    let mut joltages: Vec<i64> = Vec::new();

    joltages.push(0);
    joltages.append(&mut adapters);
    joltages.push(max_joltage + 3);

    joltages.sort();

    return joltages;
}

fn find_differences(joltages: &Vec<i64>) -> (i64, i64) {
    let mut one_jolt_differences: i64 = 0;
    let mut three_jolt_differences: i64 = 0;

    for i in 1..joltages.len() {
        let difference = joltages[i] - joltages[i - 1];
        if difference == 1 {
            one_jolt_differences+= 1;
        } else if difference == 3 {
            three_jolt_differences += 1;
        }
    }

    return (one_jolt_differences, three_jolt_differences);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let joltages = parse_file(&input_file);
    let (ojd, tjd) = find_differences(&joltages);
    
    print!("Numbers: {:?}", ojd * tjd);
}
