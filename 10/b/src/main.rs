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

fn find_distinct_paths(joltages: &Vec<i64>) -> i64 {
    let mut paths: Vec<i64> = vec![0; joltages.len()];
    paths[0] = 1;
    for i in 0..joltages.len() - 1 {
        for j in i + 1..joltages.len() {
            if joltages[j] > (joltages[i] + 3) {
                break;
            }

            paths[j] += paths[i];
        }
    }

    return *paths.last().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let joltages = parse_file(&input_file);
    let last = find_distinct_paths(&joltages);
    
    print!("Numbers: {:?}", last);
}
