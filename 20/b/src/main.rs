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
    right: Vec<char>,
    data: Vec<Vec<char>>
}

fn print_tile(t: &Tile) {
    println!("Tile {} ({}):", t.id, t.orientation);
    for y in 0..t.data.len() {
        print!("\t");
        for x in 0..t.data.len() {
            print!("{}", t.data[x][y]);
        }
        println!("");
    }
    println!("");
}

fn print_tiles(tiles: &Vec<Tile>) {
    for t in tiles {
        print_tile(t);
    }
}

fn horizontal_flip(x: usize, y: usize, max: usize) -> (usize, usize) {
    return (max - x, y);
}


fn vertical_flip(x: usize, y: usize, max: usize) -> (usize, usize) {
    return (x, max - y);
}

fn rotate(x: usize, y: usize, max: usize, count: usize) -> (usize, usize) {
    let mut new_x = x;
    let mut new_y = y;
    for _ in 0..count {
        let old_x = new_x;
        new_x = max - new_y;
        new_y = old_x;
    }
    return (new_x, new_y);
}

fn horizontal_flip_tile(tile: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_tile = Vec::new();
    for x in 0..tile.len() {
        new_tile.push(vec!['0'; tile.len()]);
    }

    for x in 0..tile.len() {
        for y in 0..tile.len() {
            let (ox, oy) = horizontal_flip(x, y, tile.len() - 1);
            new_tile[x][y] = tile[ox][oy];
        }
    }

    return new_tile;
}

fn vertical_flip_tile(tile: &Vec<Vec<char>>, p: bool) -> Vec<Vec<char>> {
    let mut new_tile = Vec::new();
    for x in 0..tile.len() {
        new_tile.push(vec!['0'; tile.len()]);
    }

    for x in 0..tile.len() {
        for y in 0..tile.len() {
            let (ox, oy) = vertical_flip(x, y, tile.len() - 1);
            new_tile[x][y] = tile[ox][oy];

            if p {
                println!("{:?} -> {:?}", (x, y), (ox, oy));
            }
        }
        if p {
            print_it(&new_tile);
            println!("");
        }
    }

    return new_tile;
}

fn rotate_tile(tile: &Vec<Vec<char>>, count: usize, p: bool) -> Vec<Vec<char>> {
    let mut new_tile = Vec::new();
    for x in 0..tile.len() {
        new_tile.push(vec!['0'; tile.len()]);
    }
    if p {
        println!("original: ");
        print_it(&tile);
    }
    for x in 0..tile.len() {
        for y in 0..tile.len() {
            let (ox, oy) = rotate(x, y, tile.len() - 1, count);
            new_tile[ox][oy] = tile[x][y];

            if p {
                println!("{:?} -> {:?}", (x, y), (ox, oy));
            }
        }
        if p {
            print_it(&new_tile);
            println!("");
        }
    }

    return new_tile;
}

fn make_rotations(tile: &Tile, flip_type: &str, tile_id: i64) -> Vec<Tile> {
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
        data: rotate_tile(&tile.data, 1, tile_id == 2473),
    });

    // 180 rotation
    tiles.push(Tile {
        id: tile.id,
        orientation: format!("{}, {} rotation", flip_type, 180),
        top: reversed_bottom.clone(),
        bottom: reversed_top.clone(),
        left: reversed_right.clone(),
        right: reversed_left.clone(),
        data: rotate_tile(&tile.data, 2, false),
    });
    

    // 270 rotation
    tiles.push(Tile {
        id: tile.id,
        orientation: format!("{}, {} rotation", flip_type, 270),
        top: tile.right.clone(),
        bottom: tile.left.clone(),
        left: reversed_top.clone(),
        right: reversed_bottom.clone(),
        data: rotate_tile(&tile.data, 3, false),
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
    tiles.append(&mut make_rotations(tile, "regular", 0)); 

    if tile.id == 2473 {
        println!("Original");
        print_tile(&tile);
    }

    // Vertical flip
    let vflip = Tile {
        id: tile.id,
        orientation: String::from("vertical flip"),
        top: tile.bottom.clone(),
        bottom: tile.top.clone(),
        left: reversed_left.clone(),
        right: reversed_right.clone(),
        data: vertical_flip_tile(&tile.data, false),
    };

    tiles.append(&mut make_rotations(&vflip, "vertical flip", 0));

    // Horizontal flip
    let hflip = Tile {
        id: tile.id,
        orientation: String::from("horizonal flip"),
        top: reversed_top.clone(),
        bottom: reversed_bottom.clone(),
        left: tile.right.clone(),
        right: tile.left.clone(),
        data: horizontal_flip_tile(&tile.data),
    };
    tiles.append(&mut make_rotations(&hflip, "horizonal flip", 0));

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
        data: Vec::new(),
    };

    for i in 0..data.len() {
        let mut it = data[i].chars();
        tile.left.push(it.next().unwrap());
        tile.right.push(it.last().unwrap());
        tile.data.push(data[i].chars().collect());
    }

    tile.data = rotate_tile(&vertical_flip_tile(&tile.data, false), 1, false);

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
                //    let tile = &tile_set[4];
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

fn print_board(board: &Vec<Vec<TileOrientation>>, tiles: &HashMap<TileOrientation, &Tile>) -> Vec<Vec<char>> {
    let mut realized_board = Vec::new();
    for x in 0..board.len() {
        realized_board.push(Vec::new());
        for y in 0..board.len() {
            realized_board[x].push(tiles.get(&board[x][y]).unwrap());
        }
    }

    println!("Tile Orientations:");
    for y in 0..board[0].len() {
        for x in 0..board.len() {
            print!("{:?} ", board[x][y]);
        }
        println!("");
    }
    println!("");

    println!("Tiles:");
    for y in 0..realized_board[0].len() {
        for x in 0..realized_board.len() {
            print!("Tile {} ({}) ", realized_board[x][y].id, realized_board[x][y].orientation);
        }
        println!("");
    }
    println!("");

    let tile_height = realized_board[0][0].data.len() - 2;

    let mut combined = Vec::new();
    for _ in 0..(tile_height * realized_board.len()) {
        combined.push(vec!['0'; tile_height * realized_board.len()]);
    }

    println!("Board:");
    for y in 0..board.len() {
        for h in 0..(tile_height + 2) {
            for x in 0..board.len() {
                for l in 0..(tile_height + 2) {
                    print!("{}", realized_board[x][y].data[l][h]);
                }
                print!(" ");
            }
            println!("");
        }
        println!("");
    }

    for x in 0..realized_board.len() {
        for y in 0..realized_board.len() {
            for w in 1..(realized_board[x][y].data.len() - 1) {
                for l in 1..(realized_board[x][y].data.len() - 1) {
                    combined[(x * tile_height) + (w - 1)][(y * tile_height) + (l - 1)] = realized_board[x][y].data[w][l];
                }
            }
        }
    }

    println!("Rows: {}, Cols: {}", combined.len(), combined[0].len());
    print_it(&combined);

    return combined;
}

fn arrange_tiles(tiles: &Vec<Vec<Tile>>) -> Vec<Vec<char>> {
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
    let mut tile_mapping = HashMap::new();
    for i in 0..tiles.len() {
        let tile = &tiles[i];
        let mut ts = Vec::new();
        for t in 0..tile.len() {
            ts.push((tile[0].id, t));
            tile_mapping.insert((tile[0].id, t), &tile[t]);
            //println!("Tile {} ({}) -> {}", tile[t].id, tile[t].orientation, t);
            //print_it(&tile[t].data);
        }
        available_tiles.insert(tile[0].id, ts);
    }

    // for (_id, orientations) in available_tiles.iter() {
    //     for orientation in orientations {
    //         print_tile(tile_mapping.get(orientation).unwrap());
    //     }
    //     println!("");
    // }

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

    let _result = find_tile(0,0, &tile_data, &mut used_tiles, &mut board, &mut memo).unwrap();

    return print_board(&board, &tile_mapping);
}

static SEA_MONSTER: [(usize, usize); 15] = [(18, 0), (0, 1), (5, 1), (6, 1), (11, 1), (12, 1), (17, 1), (18, 1), (19, 1), (1, 2), (4, 2), (7, 2), (10, 2), (13, 2), (16, 2)];
const SEA_MONSTER_HEIGHT: usize = 3;
const SEA_MONSTER_WIDTH: usize = 20;


fn is_sea_monster(board: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    // if x == 1 && y == 16 {
    //     println!("Testing {:?} ({}, {})", (x, y), board.len() - SEA_MONSTER_WIDTH, board[x].len() - SEA_MONSTER_HEIGHT);
    //     for (dx, dy) in &SEA_MONSTER {
    //         println!("Checking {:?}: {}", (x + dx, y + dy), board[x + dx][y + dy]);
    //         if board[x + dx][y + dy] == '#' {
    //             println!("Matched {:?}", (x + dx, y + dy));
    //         }
    //     }
    // }
    if x <= board.len() - SEA_MONSTER_WIDTH && y <= board[x].len() - SEA_MONSTER_HEIGHT {
        return SEA_MONSTER.iter().all(|(dx, dy)| board[x + dx][y + dy] == '#');
    } else {
        return false;
    }
}

fn find_sea_monsters(board: &Vec<Vec<char>>) -> (usize, Vec<Vec<char>>) {
    let mut b = board.clone();
    let mut sea_monsters = Vec::new();

    for x in 0..board.len() {
        for y in 0..board[x].len() {
            if is_sea_monster(board, x, y) {
                sea_monsters.push((x, y));
            }
        }
    }

    if sea_monsters.is_empty() {
        println!("No sea monsters");
    }

    for (x, y) in &sea_monsters {
        println!("Sea monster at: {:?}", (x, y));
        for (dx, dy) in &SEA_MONSTER {
            b[x + dx][y + dy] = 'O';
        }
    }

    return (sea_monsters.len(), b);
}

fn print_it(board: &Vec<Vec<char>>) {
    for y in 0..board[0].len() {
        for x in 0..board.len() {
            print!("{}", board[x][y]);
        }
        println!("");
    }
}

fn count_rough_waters(board: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for x in 0..board.len() {
        for y in 0..board.len() {
            if board[x][y] == '#' {
                count += 1;
            }
        }
    }

    return count;
}

fn get_boards(board: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut boards = Vec::new();

    boards.push(board.clone());

    let hflip = horizontal_flip_tile(&board);
    let vflip = vertical_flip_tile(&board, false);

    for i in 1..=3 {
        boards.push(rotate_tile(&board, i, false));
        boards.push(rotate_tile(&hflip, i, false));
        boards.push(rotate_tile(&vflip, i, false));
    }

    boards.push(hflip);
    boards.push(vflip);

    println!("Boards Count: {}", boards.len());
    return boards;
}

fn test_sea_monster() {
    let mut board = Vec::new();
    for x in 0..SEA_MONSTER_WIDTH {
        board.push(Vec::new());
        for y in 0..SEA_MONSTER_HEIGHT {
            board[x].push('#');
        }
    }

    let (count, new_board) = find_sea_monsters(&board);

    print_it(&board);
    println!("TEST:");
    print_it(&new_board);

    assert!(count == 1);    
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let tiles = parse_file(&input_file);

    let result = arrange_tiles(&tiles);

    let all = get_boards(&result);

    for b in &all {
        let (sea_monster_count, filled_sea_monsters) = find_sea_monsters(b);
        if sea_monster_count != 0 {
            print_it(&filled_sea_monsters);
            let count = count_rough_waters(&filled_sea_monsters);   
            println!("Result: {}", count);
        }
        println!("\n");
    }
    //test_sea_monster();
}