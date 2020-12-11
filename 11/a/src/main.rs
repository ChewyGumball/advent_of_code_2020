use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_line(line: std::io::Result<String>, file_name: String) -> Vec<char> {
    let value = match line {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name, why),
        Ok(line) => line
    };

    return value.as_str().chars().collect::<Vec<char>>();
}

fn parse_file(file_name: &Path) -> Vec<Vec<char>> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let lines = io::BufReader::new(file).lines();

    return lines.map(|line| parse_line(line, file_name.display().to_string()))
                .collect();
}

struct Change {
    row: usize,
    column: usize,
}

fn count_occupied_around(map: &Vec<Vec<char>>, row: i32, column: i32) -> usize {
    let mut occupied = 0;

    for i in -1..2 {
        for j in -1..2 {
            if j == 0 && i == 0 {
                continue;
            }

            let check_row = row + i;
            let check_col = column + j;

            if check_row < 0 || check_row >= map.len() as i32 {
                continue;
            }

            if check_col < 0 || check_col >= map[0].len() as i32 { 
                continue;
            }

            if map[check_row as usize][check_col as usize] == '#' {
                occupied += 1;
            }
        }
    }

    return occupied;
}

fn find_changes(map: &Vec<Vec<char>>) -> Vec<Change> {
    let mut changes = Vec::new();
    
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let current_value = map[i as usize][j as usize];

            if current_value != '.' {
                let occupied = count_occupied_around(map, i as i32, j as i32);
                if (occupied == 0 && current_value == 'L') || (occupied >= 4 && current_value == '#') {
                    changes.push(
                        Change {
                            row: i,
                            column: j,
                        }
                    );
                }
            }
        }
    }

    return changes;
}

fn apply_changes(map: &mut Vec<Vec<char>>, changes: &Vec<Change>) {
    for change in changes {
        if map[change.row][change.column] == 'L' {
            map[change.row][change.column] = '#';
        } else {
            map[change.row][change.column] = 'L';
        }
    }
}

fn count_occupied(map: &Vec<Vec<char>>) -> usize {
    let mut occupied = 0;

    for row in map {
        for seat in row {
            if *seat == '#' {
                occupied += 1;
            }
        }
    }

    return occupied;
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for seat in row {
            print!("{}", seat);
        }
        print!("\n");
    }
    print!("\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let mut map = parse_file(&input_file);
    
    let mut should_procced = true;
    while should_procced {
        //print_map(&map);
        let changes = find_changes(&map);
        if changes.is_empty() {
            should_procced = false;
            let occupied_seats = count_occupied(&map);
            print!("Occupied seats: {}", occupied_seats);
        } else {
            apply_changes(&mut map, &changes);
        }
    }
}