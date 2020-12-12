use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Instruction {
    direction: char,
    amount: i32,
}

fn parse_line(line: std::io::Result<String>, file_name: String) -> Instruction {
    let value = match line {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name, why),
        Ok(line) => line
    };

    let (direction, amount) = value.split_at(1);
    return Instruction {
        direction: direction.chars().next().unwrap(),
        amount: amount.parse::<i32>().unwrap()
    };
}

fn parse_file(file_name: &Path) -> Vec<Instruction> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let lines = io::BufReader::new(file).lines();

    return lines.map(|line| parse_line(line, file_name.display().to_string()))
                .collect();
}

fn rotate_direction(start_x: i32, start_y: i32, rotation_direction: char, angle: i32) -> (i32, i32) {
    let mut new_x = start_x;
    let mut new_y = start_y;
    let mut current_angle = angle;
    while current_angle > 0 {
        current_angle -= 90;
        match rotation_direction {
            'R' => {
                let prev_y = new_y;
                new_y = -new_x;
                new_x = prev_y;
            },
            'L' => {
                let prev_y = new_y;
                new_y = new_x;
                new_x = -prev_y;
            },
            _ => panic!("Unknown rotation direction: {}", rotation_direction),
        }
    }

    return (new_x, new_y);
}

fn follow_instructions(instructions: &Vec<Instruction>) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut wx: i32 = 10;
    let mut wy: i32 = 1;

    for instruction in instructions {
        if instruction.direction == 'F' {
            x += wx * instruction.amount;
            y += wy * instruction.amount;
            continue;
        }

        if instruction.direction == 'R' || instruction.direction == 'L' {
            let (new_wx, new_wy) = rotate_direction(wx, wy, instruction.direction, instruction.amount);
            wx = new_wx;
            wy = new_wy;
            continue;
        }

        match instruction.direction {
            'N' => wy += instruction.amount,
            'E' => wx += instruction.amount,
            'S' => wy -= instruction.amount,
            'W' => wx -= instruction.amount,
            _ => panic!("Unknown direction {}", instruction.direction),
        };
    }

    return x.abs() + y.abs();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let instructions = parse_file(&input_file);
    let manhattan_distance = follow_instructions(&instructions);

    println!("Manhattan Distance: {}", manhattan_distance);
}