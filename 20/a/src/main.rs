use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};


#[derive(Debug, Clone)]
struct Tile {
    id: i64,
    orientation: String,
    top: Vec<char>,
    bottom: Vec<char>,
    left: Vec<char>,
    right: Vec<char>
}

fn print_tile(t: &Tile) {
    println!("Tile {} ({}):", t.id, t.orientation);
    print!("\t");
    for c in &t.top {
        print!("{}", c);
    }
    println!("");
    assert!(t.top[0] == t.left[0]);
    assert!(t.top[t.top.len()-1] == t.right[0]);
    assert!(t.bottom[0] == t.left[t.left.len()-1]);
    assert!(t.bottom[t.bottom.len()-1] == t.right[t.right.len()-1]);

    for i in 1..(t.top.len()-1) {
        print!("\t{}", t.left[i]);
        for _j in 1..(t.top.len()-1) {
            print!(" ");
        }
        println!("{}", t.right[i]);
    }
    print!("\t");
    for c in &t.bottom {
        print!("{}", c);
    }
    println!("");
    println!("");
}

fn print_tiles(tiles: &Vec<Tile>) {
    for t in tiles {
        print_tile(t);
    }
}

fn make_rotations(tile: &Tile, flip_type: &str) -> Vec<Tile> {
    let mut tiles = Vec::new();

    let mut reversed_top = tile.top.clone();
    reversed_top.reverse();

    let mut reversed_bottom = tile.bottom.clone();
    reversed_bottom.reverse();

    let mut reversed_left = tile.left.clone();
    reversed_left.reverse();

    let mut reversed_right = tile.right.clone();
    reversed_right.reverse();

    // regular
    tiles.push(tile.clone()); 
    
    // 90 rotation
    tiles.push(Tile {
        id: tile.id,
        orientation: format!("{}, {} rotation", flip_type, 90),
        top: reversed_left.clone(),
        bottom: reversed_right.clone(),
        left: tile.bottom.clone(),
        right: tile.top.clone(),
    });

    // 180 rotation
    tiles.push(Tile {
        id: tile.id,
        orientation: format!("{}, {} rotation", flip_type, 180),
        top: reversed_bottom.clone(),
        bottom: reversed_top.clone(),
        left: reversed_right.clone(),
        right: reversed_left.clone(),
    });
    

    // 270 rotation
    tiles.push(Tile {
        id: tile.id,
        orientation: format!("{}, {} rotation", flip_type, 270),
        top: tile.right.clone(),
        bottom: tile.left.clone(),
        left: reversed_top.clone(),
        right: reversed_bottom.clone(),
    });

    return tiles;
}

fn make_rotations_and_flips(tile: &Tile) -> Vec<Tile> {
    let mut tiles = Vec::new();

    let mut reversed_top = tile.top.clone();
    reversed_top.reverse();

    let mut reversed_bottom = tile.bottom.clone();
    reversed_bottom.reverse();

    let mut reversed_left = tile.left.clone();
    reversed_left.reverse();

    let mut reversed_right = tile.right.clone();
    reversed_right.reverse();
    
    // regular
    tiles.append(&mut make_rotations(tile, "regular")); 

    // Vertical flip
    let vflip = Tile {
        id: tile.id,
        orientation: String::from("vertical flip"),
        top: tile.bottom.clone(),
        bottom: tile.top.clone(),
        left: reversed_left.clone(),
        right: reversed_right.clone(),
    };
    tiles.append(&mut make_rotations(&vflip, "vertical flip"));

    // Horizontal flip
    let hflip = Tile {
        id: tile.id,
        orientation: String::from("horizonal flip"),
        top: reversed_top.clone(),
        bottom: reversed_bottom.clone(),
        left: tile.right.clone(),
        right: tile.left.clone(),
    };
    tiles.append(&mut make_rotations(&hflip, "horizonal flip"));

    //print_tiles(tile.id, &tiles);

    return tiles;
}

fn make_tiles(id: i64, data: &Vec<&String>) -> Vec<Tile> {

    let mut tile = Tile {
        id: id,
        orientation: String::from("regular"),
        top: data.first().unwrap().chars().collect(),
        bottom: data.last().unwrap().chars().collect(),
        left: Vec::new(),
        right: Vec::new(),
    };

    for i in 0..data.len() {
        let mut it = data[i].chars();
        tile.left.push(it.next().unwrap());
        tile.right.push(it.last().unwrap());
    }

    return make_rotations_and_flips(&tile);
}

fn parse_file(file_name: &Path) -> Vec<Vec<Tile>> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let lines: Vec<String> = io::BufReader::new(file).lines().map(|l| String::from(l.unwrap().trim())).collect();

    let mut tiles = Vec::new();

    let mut current_tile_id = 0;
    let mut current_tile = Vec::new();
    for i in 0..lines.len() {
        let line = &lines[i];
        if line.is_empty() {
            tiles.push(make_tiles(current_tile_id, &current_tile));
            continue;
        }

        if line.starts_with("Tile") {
            current_tile_id = line[5..(line.len() - 1)].parse::<i64>().unwrap();
            current_tile.clear();
            continue;
        }

        current_tile.push(&line);
    }

    tiles.push(make_tiles(current_tile_id, &current_tile));
    
    return tiles;
}

type TileID = i64;
type Orientation = usize;
type TileOrientation = (i64, usize);

const NO_TILE: TileOrientation = (-1, 1000);

type Cache = HashMap<TileOrientation, Vec<TileOrientation>>;

fn make_bottom_cache(tiles: &Vec<Vec<Tile>>) -> (Cache, HashMap<usize, String>) {
    let mut cache = HashMap::new();
    let mut orientations = HashMap::new();

    for i in 0..tiles[0].len() {
        orientations.insert(i, tiles[0][i].orientation.clone());
    }

    for i in 0..tiles.len() {
        let i_orientations = &tiles[i];

        for i_o in 0..(i_orientations.len()) {
            let i_t = &i_orientations[i_o];
            let t = cache.entry((i_t.id, i_o)).or_insert_with(|| Vec::new());
            for j in 0..tiles.len() {
                if i == j {
                    continue;
                }

                let j_orientations = &tiles[j];

                for j_o in 0..j_orientations.len() {
                    let j_t = &j_orientations[j_o];
                    if i_t.bottom == j_t.top {
                        t.push((j_t.id, j_o));
                    }
                }
            }
        }
    }

    return (cache, orientations);
}

fn make_right_cache(tiles: &Vec<Vec<Tile>>) -> Cache {
    let mut cache = HashMap::new();

    for i in 0..tiles.len() {
        let i_orientations = &tiles[i];

        for i_o in 0..(i_orientations.len()) {
            let i_t = &i_orientations[i_o];
            let t = cache.entry((i_t.id, i_o)).or_insert_with(|| Vec::new());
            for j in 0..tiles.len() {
                if i == j {
                    continue;
                }

                let j_orientations = &tiles[j];

                for j_o in 0..j_orientations.len() {
                    let j_t = &j_orientations[j_o];
                    if i_t.right == j_t.left {
                        t.push((j_t.id, j_o));
                    }
                }
            }
        }
    }

    return cache;
}

struct TileData {
    tiles: HashMap<i64, Vec<TileOrientation>>,
    right_cache: Cache,
    bottom_cache: Cache,
    orientations: HashMap<usize, String>
}

fn path_to_string(path: &Vec<i64>) -> String {
    return path.iter().map(|i| i.to_string()).collect();
}

fn find_tile(x: usize, y: usize, tile_data: &TileData, used_tiles: &mut Vec<i64>, board: &mut Vec<Vec<TileOrientation>>, memo: &mut HashSet<String>) -> Option<i64> {
    let current_path = path_to_string(used_tiles);
    if memo.contains(&current_path) {
        return None;
    }
    
    if x == 0 && y == 0 {
        let available_tiles: Vec<(&i64, &Vec<TileOrientation>)> = tile_data.tiles.iter().filter(|(k,_v)| !used_tiles.contains(k)).collect();
        for (id, tile_set) in available_tiles {
            println!("Searching: {}", tile_set[0].0);
            for tile in tile_set {
                //println!("{} ({}) at ({},{})", tile.0, orientations.get(&tile.1)?, x, y);
                board[x][y] = *tile;
                used_tiles.push(tile.0);
                let result = find_tile(1, 0, tile_data, used_tiles, board, memo);
                if result.is_some() {
                    return result;
                } else {
                    used_tiles.pop();
                }
            }
        }

        assert!(false);
    }

    let up_candidates: HashSet<&TileOrientation> = if y > 0 {
        let up = board[x][y - 1];
        tile_data.bottom_cache.get(&up)?.iter().filter(|t| !used_tiles.contains(&t.0)).collect()
    } else {
        tile_data.bottom_cache.keys().filter(|t| !used_tiles.contains(&t.0)).collect()
    };

    let left_candidates: HashSet<&TileOrientation> = if x > 0 {
        let left = board[x - 1][y];
        tile_data.right_cache.get(&left)?.iter().filter(|t| !used_tiles.contains(&t.0)).collect()
    } else {
        tile_data.right_cache.keys().filter(|t| !used_tiles.contains(&t.0)).collect()
    };

    let candidates: Vec<&&TileOrientation> = up_candidates.intersection(&left_candidates).collect();


    for candidate in up_candidates.intersection(&left_candidates) {
        //println!("{} ({}) at ({},{})", candidate.0, orientations.get(&candidate.1)?, x, y);
        board[x][y] = **candidate;
        used_tiles.push(candidate.0);

        if used_tiles.len() == tile_data.tiles.len() {
            let end = board.len() - 1;
            let top_left = board[0][0].0;
            let top_right = board[end][0].0;
            let bottom_left = board[0][end].0;
            let bottom_right = board[end][end].0;
            
            //println!("Final Board {:?}", board);
    
            return Some(top_left * top_right * bottom_left * bottom_right);
        }

        let next_x = x + 1;
        let result = if next_x == board.len() {
            find_tile(0, y + 1, tile_data, used_tiles, board, memo) 
        } else {
            find_tile(next_x, y, tile_data, used_tiles, board, memo)
        };
        
        if result.is_some() {
            return result;
        } else {
            memo.insert(path_to_string(used_tiles));
            used_tiles.pop();
        }
    }

    board[x][y] = NO_TILE;

    return None;
}

fn arrange_tiles(tiles: &Vec<Vec<Tile>>) -> i64 {
    let dimensions = (tiles.len() as f32).sqrt() as usize;
    println!("Dimensions: {}x{} ({} total tiles)", dimensions, dimensions, tiles.len());

    let mut board = Vec::new();
    for x in 0..dimensions {
        board.push(Vec::new());
        for _y in 0..dimensions {
            board[x].push(NO_TILE);
        }
    }

    let mut available_tiles = HashMap::new();
    for tile in tiles {
        let mut ts = Vec::new();
        for t in 0..(tile.len() - 1) {
            ts.push((tile[0].id, t));
        }
        available_tiles.insert(tile[0].id, ts);
    }
    let mut used_tiles = Vec::new();

    println!("Available Tiles: {:?}", available_tiles.keys());

    let (bottom_cache, orientations) = make_bottom_cache(&tiles);
    let right_cache = make_right_cache(&tiles);

    let tile_data = TileData {
        tiles: available_tiles,
        right_cache: right_cache,
        bottom_cache: bottom_cache,
        orientations: orientations
    };

    let mut memo = HashSet::new();

    return find_tile(0,0, &tile_data, &mut used_tiles, &mut board, &mut memo).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let tiles = parse_file(&input_file);

    let result = arrange_tiles(&tiles);

    println!("Result: {}", result);
}