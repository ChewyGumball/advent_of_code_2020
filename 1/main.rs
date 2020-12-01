use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_line(line: std::io::Result<String>, file_name: String) -> i32 {
    let real_line = match line {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name, why),
        Ok(line) => line
    };

    return match real_line.parse() {
        Err(why) => panic!("Couldn't parse \"{}\" into a number: {}", real_line, why),
        Ok(number) => number
    };
}

fn parse_file(file_name: &Path) -> Vec<i32> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let lines = io::BufReader::new(file).lines();

    return lines.map(|line| parse_line(line, file_name.display().to_string()))
                .collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let numbers = parse_file(&input_file);
    let number_count = numbers.len() - 1;
    for i in 0..number_count {
        let a = numbers[i];
        for j in i..number_count {
            let b = numbers[j];
            for k in j..number_count {
                let c = numbers[k];
                if a + b + c == 2020 {
                    println!("{} ({}) * {} ({}) * {} ({}) = {}", a, i, b, j, c, k, a * b * c);
                }
            }
        }
    }
}