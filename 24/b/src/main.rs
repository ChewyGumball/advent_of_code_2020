use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

type Tile = (i64, i64);

fn parse_line(line: std::io::Result<String>, file_name: String) -> Tile {
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

fn parse_file(file_name: &Path) -> Vec<Tile> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let lines = io::BufReader::new(file).lines();

    return lines.map(|line| parse_line(line, file_name.display().to_string()))
                .collect();
}

fn insert_tile(tile: Tile, floor: &mut HashMap<Tile, &str>) {
    if !floor.contains_key(&tile) {
        floor.insert(tile, "white");
    }
}

fn flip_tile(tile: Tile, mut floor: &mut HashMap<Tile, &str>) {
    ensure_tile_border_exists(tile, &mut floor);
    if *floor.get(&tile).unwrap() == "black" {
        floor.insert(tile, "white");
    } else {
        floor.insert(tile, "black");
    }
}

const TILE_WITH_BORDER: [Tile; 7] = [(0,0), (1,1), (2,0), (1,-1), (-1,-1), (-2,0), (-1,1)];
fn ensure_tile_border_exists(tile: Tile, mut floor: &mut HashMap<Tile, &str>) {
    for offset in TILE_WITH_BORDER.iter() {
        insert_tile((tile.0 + offset.0, tile.1 + offset.1), &mut floor);
    }
}


fn count_black_border(tile: Tile, floor: &HashMap<Tile, &str>) -> usize {
    let mut count = 0;
    for offset in TILE_WITH_BORDER.iter() {
        if *offset == (0,0) {
            continue;
        }

        let target = (tile.0 + offset.0, tile.1 + offset.1);

        if let Some(&value) = floor.get(&target) {
            if value == "black" {
                count += 1;
            }
        }
    }

    return count;
}

fn tick(mut floor: &mut HashMap<Tile, &str>) {
    let mut tiles_to_flip = Vec::new();

    for (&tile, &value) in floor.iter() {
        let black_count = count_black_border(tile, &floor);
        if value == "black" && (black_count == 0 || black_count > 2) {
            tiles_to_flip.push(tile);
        } else if value == "white" && (black_count == 2) {
            tiles_to_flip.push(tile);
        }
    }

    for tile in tiles_to_flip {
        flip_tile(tile, &mut floor);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let flips = parse_file(&input_file);

    let mut tiles = HashMap::new();

    for flip in flips {
        flip_tile(flip, &mut tiles);
    }

    for i in 0..100 {
        tick(&mut tiles);
        let black_tiles = tiles.values().filter(|x| **x == "black").count();
        println!("Day {}: {}", i + 1, black_tiles);
    }
}