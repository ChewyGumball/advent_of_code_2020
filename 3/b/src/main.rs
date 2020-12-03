use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_line(line: std::io::Result<String>, file_name: String) -> Vec<bool> {
    let value = match line {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name, why),
        Ok(line) => line
    };

    let mut row = Vec::new();
    for element in value.as_str().chars() {
        match element {
            '.' => row.push(false),
            '#' => row.push(true),
            _ => panic!("Unknown space type: {}", element)
        }
    }
    
    return row;
}

fn parse_file(file_name: &Path) -> Vec<Vec<bool>> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let lines = io::BufReader::new(file).lines();

    return lines.map(|line| parse_line(line, file_name.display().to_string()))
                .collect();
}

fn count_trees(map: &Vec<Vec<bool>>, dx: usize, dy: usize) -> usize {
    let mut tree_count: usize = 0;

    let mut x: usize = 0;
    let mut y: usize = 0;

    let max_height = map.len();
    let max_width = map[0].len();

    while y < max_height {
        if map[y][x % max_width] {
            tree_count += 1;
        }

        y += dy;
        x += dx;
    }

    return tree_count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let map = parse_file(&input_file);
    let trees = count_trees(&map, 1, 1) * count_trees(&map, 3, 1) * count_trees(&map, 5, 1) * count_trees(&map, 7, 1) * count_trees(&map, 1, 2);
    print!("Trees: {}", trees);
}