use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

use regex::Regex;
use lazy_static::lazy_static;

#[derive(Copy, Clone)]
struct BinaryMask {
    forced_ones: u64,
    forced_zeros: u64,
}

struct MemoryAssignment {
    address: usize,
    value: u64,
}

enum Instruction {
    Mask(BinaryMask),
    Assignment(MemoryAssignment),
}

fn parse_mask(line: &String) -> Instruction {
    let value = line.as_str().chars().skip(7).collect::<String>();

    let zeros = value.replace("X", "0");
    let ones = value.replace("X", "1");

    return Instruction::Mask(BinaryMask{
        forced_ones: u64::from_str_radix(zeros.as_str(), 2).unwrap(),
        forced_zeros: u64::from_str_radix(ones.as_str(), 2).unwrap()
    });
}


lazy_static! {
    static ref ASSIGNMENT_REGEX: Regex = Regex::new(r"^mem\[(?P<address>\d+)] = (?P<value>\d+)$").unwrap();
}

fn parse_assignment(line: &String) -> Instruction {
    let captures = ASSIGNMENT_REGEX.captures(line).unwrap();
    return Instruction::Assignment(MemoryAssignment {
        address: captures.name("address").unwrap().as_str().parse().unwrap(),
        value: captures.name("value").unwrap().as_str().parse().unwrap()
    });
}

fn parse_line(line: std::io::Result<String>, file_name: String) -> Instruction {
    let value = match line {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name, why),
        Ok(line) => line
    };

    if value.starts_with("mask") {
        return parse_mask(&value);
    } else {
        return parse_assignment(&value);
    }
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

fn execute(instructions: &Vec<Instruction>) -> u64 {
    let mut current_mask = BinaryMask {
        forced_zeros: u64::MAX,
        forced_ones: 0
    };

    let mut memory: HashMap<usize, u64> = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask)=> current_mask = *mask,
            Instruction::Assignment(assignment) => {
                let mut new_value = assignment.value;
                new_value = new_value | current_mask.forced_ones;
                new_value = new_value & current_mask.forced_zeros;

                let entry = memory.entry(assignment.address).or_default();
                *entry = new_value;
            }
        };
    }

    return memory.values().sum();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let instructions = parse_file(&input_file);
    let sum = execute(&instructions);

    println!("Sum: {}", sum);
}