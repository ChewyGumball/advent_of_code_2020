use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Instruction {
    Nop { visited: bool },
    Acc { visited: bool, argument: i32 },
    Jmp { visited: bool, offset: i32 },
}

fn parse_instruction(line: std::io::Result<String>, file_name: String) -> Instruction {
    let value = match line {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name, why),
        Ok(line) => line
    };
    let instruction_type = &value.as_str()[..3];
    let argument_string = &value.as_str()[4..];
    let argument = match argument_string.parse::<i32>() {
        Ok(number) => number,
        Err(why) => panic!("Couldn't parse argument from line {}: {}", value, why)
    };

    return match instruction_type {
        "nop" => Instruction::Nop { visited: false },
        "acc" => Instruction::Acc { visited: false, argument: argument },
        "jmp" => Instruction::Jmp { visited: false, offset: argument},
        _ => panic!("Unknown instruction type: {}", instruction_type),
    };
}

fn parse_file(file_name: &Path) -> Vec<Instruction> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let lines = io::BufReader::new(file).lines();

    return lines.map(|line| parse_instruction(line, file_name.display().to_string()))
                .collect();
}

fn has_been_visited(instruction: &Instruction) -> bool {
    return match instruction {
        Instruction::Nop {visited} => *visited,
        Instruction::Acc {visited, argument: _} => *visited,
        Instruction::Jmp {visited, offset: _} => *visited,
    };
}

fn set_visited(instruction: &mut Instruction) {
    return match instruction {
        Instruction::Nop{visited} => *visited = true,
        Instruction::Acc{visited, argument: _} => *visited = true,
        Instruction::Jmp{visited, offset: _} => *visited = true,
    };
}

fn execute(instructions: &mut Vec<Instruction>) -> i32 {
    let mut accumulator: i32 = 0;
    let mut current_instruction: usize = 0;

    while !has_been_visited(&instructions[current_instruction]) {
        set_visited(&mut instructions[current_instruction]);
        match instructions[current_instruction] {
            Instruction::Nop{visited: _} => current_instruction += 1,
            Instruction::Acc{visited: _, argument} => {
                current_instruction += 1;
                accumulator += argument;
            },
            Instruction::Jmp{visited: _, offset} => {
                current_instruction = (current_instruction as i32 + offset) as usize;
            }
        }
    }

    return accumulator;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let mut instructions = parse_file(&input_file);
    let accumulater_before_infinite_loop = execute(&mut instructions);
    
    print!("Accumulator: {}", accumulater_before_infinite_loop);
}
