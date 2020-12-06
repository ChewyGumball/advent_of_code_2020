use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


struct BoardingPass {
    seat: String
}

fn parse_line(line: std::io::Result<String>, file_name: String) -> BoardingPass {
    let value = match line {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name, why),
        Ok(line) => line
    };

    return BoardingPass {
        seat: value
    };
}

fn parse_file(file_name: &Path) -> Vec<BoardingPass> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let lines = io::BufReader::new(file).lines();

    return lines.map(|line| parse_line(line, file_name.display().to_string()))
                .collect();
}

fn find_row(seat: &String) -> i32 {
    let rows: Vec<i32> = (0..128).collect();
    let mut remaining_rows: &[i32] = &rows;
    let row_part = &seat[..7];
    for split in row_part.chars() {
        let (front, back) = remaining_rows.split_at(remaining_rows.len() / 2);
        match split {
            'F' => remaining_rows = front,
            'B' => remaining_rows = back,
            _ => panic!("Unknown row split: {}", split)
        };
    }

    if remaining_rows.len() != 1 {
        panic!("How?");
    }

    return remaining_rows[0];
}

fn find_column(seat: &String) -> i32 {
    let columns: Vec<i32> = (0..8).collect();
    let mut remaining_columns: &[i32] = &columns;
    let column_part = &seat[7..];
    for split in column_part.chars() {
        let (left, right) = remaining_columns.split_at(remaining_columns.len() / 2);
        match split {
            'L' => remaining_columns = left,
            'R' => remaining_columns = right,
            _ => panic!("Unknown column split: {}", split)
        };
    }

    if remaining_columns.len() != 1 {
        panic!("Why?");
    }

    return remaining_columns[0];
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let boarding_passes = parse_file(&input_file);

    let mut highest_seat_id = 0;
    for boarding_pass in boarding_passes {
        let row = find_row(&boarding_pass.seat);
        let column = find_column(&boarding_pass.seat);

        let seat_id = row * 8 + column;
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }

    print!("Highest Seat ID: {}", highest_seat_id);
}
