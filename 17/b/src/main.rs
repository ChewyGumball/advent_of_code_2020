use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_line(line: std::io::Result<String>, file_name: String) -> Vec<char> {
    let value = match line {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name, why),
        Ok(line) => line,
    };

    return value.as_str().chars().collect::<Vec<char>>();
}

fn parse_file(file_name: &Path) -> Vec<Vec<char>> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file).lines();

    return lines
        .map(|line| parse_line(line, file_name.display().to_string()))
        .collect();
}

struct Change {
    row: usize,
    column: usize,
    layer: usize,
    other_layer: usize,
}

fn count_active_around(map: &Vec<Vec<Vec<Vec<char>>>>, row: i32, column: i32, layer: i32, other_layer: i32) -> usize {
    let mut occupied = 0;

    for i in -1..2 {
        for j in -1..2 {
            for k in -1..2 {
                for l in -1..2 {
                    if j == 0 && i == 0 && k == 0 && l == 0 {
                        continue;
                    }

                    let check_row = row + i;
                    let check_col = column + j;
                    let check_layer = layer + k;
                    let check_other_layer = other_layer + l;

                    if check_row < 0 || check_row >= map.len() as i32 {
                        continue;
                    }

                    if check_col < 0 || check_col >= map[0].len() as i32 {
                        continue;
                    }

                    if check_layer < 0 || check_layer >= map[0][0].len() as i32 {
                        continue;
                    }

                    if check_other_layer < 0 || check_other_layer >= map[0][0][0].len() as i32 {
                        continue;
                    }

                    if map[check_row as usize][check_col as usize][check_layer as usize][check_other_layer as usize] == '#' {
                        occupied += 1;
                    }
                }
            }
        }
    }

    return occupied;
}

fn find_changes(map: &Vec<Vec<Vec<Vec<char>>>>) -> Vec<Change> {
    let mut changes = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            for k in 0..map[0][0].len() {
                for l in 0..map[0][0][0].len() {
                    let current_value = map[i as usize][j as usize][k as usize][l as usize];
                    let occupied = count_active_around(map, i as i32, j as i32, k as i32, l as i32);

                    if current_value == '.' && occupied == 3 {
                        changes.push(Change{row: i, column: j, layer: k, other_layer: l});
                    } else if current_value == '#' && !(occupied == 2 || occupied == 3) {
                        changes.push(Change{row: i, column: j, layer: k, other_layer: l});
                    }
                }
            }
        }
    }

    return changes;
}

fn apply_changes(map: &mut Vec<Vec<Vec<Vec<char>>>>, changes: &Vec<Change>) {
    for change in changes {
        if map[change.row][change.column][change.layer][change.other_layer] == '.' {
            map[change.row][change.column][change.layer][change.other_layer] = '#';
        } else {
            map[change.row][change.column][change.layer][change.other_layer] = '.';
        }
    }
}

// fn print_map(map: &Vec<Vec<Vec<Vec<char>>>>, cycles: usize) {
//     for height in 0..(cycles * 2 + 1) {
//         println!("z={}", height as i32 - cycles as i32);
//         for row in map {
//             for col in row {
//                 print!("{}", col[height]);
//             }
//             print!("\n");
//         }
//         print!("\n\n");
//     }
//     print!("\n");
// }

fn expand_map(start_layer: &Vec<Vec<char>>, cycles: usize) -> Vec<Vec<Vec<Vec<char>>>> {
    let max_width = start_layer.len() + cycles * 2;
    let max_length = start_layer[0].len() + cycles * 2;
    let max_height = cycles * 2 + 1;
    let max_other = cycles * 2 + 1;

    let mut expanded_map = Vec::new();
    for i in 0..max_width {
        expanded_map.push(Vec::new());
        for j in 0..max_length {
            expanded_map[i].push(Vec::new());
            for _ in 0..max_height {
                expanded_map[i][j].push(vec!['.'; max_other]);
            }
        }
    }

    let z = cycles;
    let w = cycles;
    for x in cycles..(cycles + start_layer.len()) {
        for y in cycles..(cycles + start_layer[0].len()) {
            expanded_map[x][y][z][w] = start_layer[x - cycles][y - cycles];
        }
    }

    return expanded_map;
}

fn count_total_active(map: &Vec<Vec<Vec<Vec<char>>>>) -> i32{
    let mut count = 0;
    for row in map {
        for col in row {
            for layer in col {
                for other in layer {
                    if *other == '#' {
                        count += 1;
                    }
                }
            }
        }
    }

    return count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let start_layer = parse_file(&input_file);
    let mut map = expand_map(&start_layer, 6);
    //print_map(&map, 6);


    for i in 0..6 {
        let changes = find_changes(&map);
        apply_changes(&mut map, &changes);
        println!("Iteration {}", i + 1);
        //print_map(&map, 6);
    }

    println!("Total Active: {}", count_total_active(&map));
}
