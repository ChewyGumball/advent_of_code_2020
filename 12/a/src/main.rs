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

fn rotate_direction(current_direction: char, rotation_direction: char, angle: i32) -> char {
    let mut new_direction = current_direction;
    let mut current_angle = angle;
    while current_angle > 0 {
        current_angle -= 90;
        new_direction = match rotation_direction {
            'R' => match new_direction {
                        'N' => 'E',
                        'E' => 'S',
                        'S' => 'W',
                        'W' => 'N',
                        _ => panic!("Unknown direction: {}", new_direction),
                    },
            'L' => match new_direction {
                'N' => 'W',
                'W' => 'S',
                'S' => 'E',
                'E' => 'N',
                _ => panic!("Unknown direction: {}", new_direction),
            },
            _ => panic!("Unknown rotation direction: {}", rotation_direction),
        }
    }

    return new_direction;
}

fn follow_instructions(instructions: &Vec<Instruction>) -> i32 {
    let mut current_facing = 'E';
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for instruction in instructions {
        if instruction.direction == 'R' || instruction.direction == 'L' {
            current_facing = rotate_direction(current_facing, instruction.direction, instruction.amount);
            continue;
        }

        let mut direction_to_move = instruction.direction;
        if instruction.direction == 'F' {
            direction_to_move = current_facing;
        }

        match direction_to_move {
            'N' => y += instruction.amount,
            'E' => x += instruction.amount,
            'S' => y -= instruction.amount,
            'W' => x -= instruction.amount,
            _ => panic!("Unknown direction {}", direction_to_move),
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