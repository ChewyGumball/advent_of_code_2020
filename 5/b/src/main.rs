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

fn find_row(seat: &String) -> usize {
    let rows: Vec<usize> = (0..128).collect();
    let mut remaining_rows: &[usize] = &rows;
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

fn find_column(seat: &String) -> usize {
    let columns: Vec<usize> = (0..8).collect();
    let mut remaining_columns: &[usize] = &columns;
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

fn get_seat_plan(boarding_passes: &Vec<BoardingPass>) -> Vec<bool> {
    let mut seat_plan = Vec::new();
    for _row in 0..128 {
        for _column in 0..8 {
            seat_plan.push(false);
        }
    }

    for boarding_pass in boarding_passes {
        let row = find_row(&boarding_pass.seat);
        let column = find_column(&boarding_pass.seat);
        let seat_id = row * 8 + column;

        seat_plan[seat_id] = true;
    }

    return seat_plan;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let boarding_passes = parse_file(&input_file);
    let seat_plan = get_seat_plan(&boarding_passes);

    for seat in 1..seat_plan.len() - 1 {
         let previous_seat = seat - 1;
         let next_seat = seat + 1;

         if seat_plan[previous_seat] && seat_plan[next_seat] && !seat_plan[seat] {
             print!("Missing boarding pass for seat id {}", seat);
         }
    }
}
