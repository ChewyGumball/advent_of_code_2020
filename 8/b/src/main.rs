use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Nop { argument: i32 },
    Acc { argument: i32 },
    Jmp { offset: i32 },
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
        "nop" => Instruction::Nop { argument: argument },
        "acc" => Instruction::Acc { argument: argument },
        "jmp" => Instruction::Jmp { offset: argument},
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

fn execute(instructions: &Vec<Instruction>) -> (i32, bool) {
    let mut accumulator: i32 = 0;
    let mut current_instruction: usize = 0;
    let mut visited = vec![false; instructions.len()];

    while current_instruction < instructions.len() && !visited[current_instruction] {
        //print!("Current Index: {} ", current_instruction);
        visited[current_instruction] = true;
        match instructions[current_instruction] {
            Instruction::Nop{argument: _} => {
                //println!("Executing nop");
                current_instruction += 1;
            },
            Instruction::Acc{argument} => {
                current_instruction += 1;
                accumulator += argument;
                //println!("Executing acc: {} (accumulator: {})", argument, accumulator);
            },
            Instruction::Jmp{offset} => {
                current_instruction = (current_instruction as i32 + offset) as usize;
                //println!("Executing jmp: {} (index: {})", offset, current_instruction);
            }
        }
    }

    return (accumulator, current_instruction != instructions.len());
}

fn is_jmp_or_nop(instruction: &Instruction) -> bool {
    return match instruction {
        Instruction::Nop {argument: _} => true,
        Instruction::Acc {argument: _} => false,
        Instruction::Jmp {offset: _} => true,
    }
}

fn swap_instruction(instruction: &Instruction) -> Instruction {
    return match instruction {
        Instruction::Nop {argument} => Instruction::Jmp {offset: *argument},
        Instruction::Acc {argument: _} => panic!("Can't swap an acc instruction!"),
        Instruction::Jmp {offset} => Instruction::Nop {argument: *offset},
    }
}

fn fix_program(instructions: &mut Vec<Instruction>) -> i32 {
    //println!("Original: {:?}", instructions);

    for i in 0..instructions.len() {
        if is_jmp_or_nop(&instructions[i]) {
            let original_instruction = instructions[i];
            instructions[i] = swap_instruction(&original_instruction);
            let (accumulator, infinite_loop) = execute(&instructions);

            if !infinite_loop {
                //println!("Solution: {:?}", instructions);
                return accumulator;
            } else {
                instructions[i] = original_instruction;
            }
        }
    }

    panic!("Couldn't find the right swap!");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let mut instructions = parse_file(&input_file);
    let accumulator = fix_program(&mut instructions);
    
    print!("Accumulator: {}", accumulator);
}
