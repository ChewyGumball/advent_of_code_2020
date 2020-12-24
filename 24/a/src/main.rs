use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn parse_line(line: std::io::Result<String>, file_name: String) -> (i64, i64) {
    let s = match line {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name, why),
        Ok(line) => line
    };

    let mut position = (0, 0);
    let chars = s.chars().collect::<Vec<char>>();

    let mut current_index = 0;
    while current_index < chars.len() {
        match chars[current_index] {
            'e' => position = (position.0 - 2, position.1),
            'w' => position = (position.0 + 2, position.1),
            's' => {
                current_index += 1;
                match chars[current_index] {
                    'e' => position = (position.0 - 1, position.1 - 1),
                    'w' => position = (position.0 + 1, position.1 - 1),
                    _ => panic!("Unknown south direction: {}", chars[current_index])
                }
            },
            'n' => {
                current_index += 1;
                match chars[current_index] {
                    'e' => position = (position.0 - 1, position.1 + 1),
                    'w' => position = (position.0 + 1, position.1 + 1),
                    _ => panic!("Unknown north direction: {}", chars[current_index])
                }
            },
            _ => panic!("Unknown direction: {}", chars[current_index])
        }
        current_index += 1;
    }

    return position;
}

fn parse_file(file_name: &Path) -> Vec<(i64, i64)> {
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

    let flips = parse_file(&input_file);

    let mut tiles = HashMap::new();

    for flip in flips {
        if tiles.contains_key(&flip) {
            if *tiles.get(&flip).unwrap() == "black" {
                tiles.insert(flip, "white");
            } else {
                tiles.insert(flip, "black");
            }
        } else {
            tiles.insert(flip, "black");
        }
    }

    let black_tiles = tiles.values().filter(|x| **x == "black").count();

    println!("{:?}", black_tiles);
}