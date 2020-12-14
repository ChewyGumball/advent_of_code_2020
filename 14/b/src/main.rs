use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

use regex::Regex;
use lazy_static::lazy_static;

struct BinaryMask {
    mask: String
}

struct MemoryAssignment {
    address: u64,
    value: u64,
}

enum Instruction {
    Mask(BinaryMask),
    Assignment(MemoryAssignment),
}

fn parse_mask(line: &String) -> Instruction {
    return Instruction::Mask(BinaryMask {
        mask: line.as_str().chars().skip(7).collect::<String>()
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

fn apply_mask2(address: u64, mask: &[u8], current_index: usize, addresses: &mut Vec<u64>) {
    if current_index >= mask.len() {
        addresses.push(address);
        return;
    }

    let bit_mask = 1 << (35 - current_index);


    match mask[current_index] as char{
        '1' => apply_mask2(address | bit_mask, mask, current_index + 1, addresses),
        'X' => {
            apply_mask2(address | bit_mask, mask, current_index + 1, addresses);
            apply_mask2(address & (!bit_mask), mask, current_index + 1, addresses);
        },
        _ => apply_mask2(address, mask, current_index + 1, addresses)
    };
}

fn apply_mask(address: u64, mask: &BinaryMask) -> Vec<u64> {
    let mut addresses: Vec<u64> = Vec::new();
    apply_mask2(address, mask.mask.as_bytes(), 0, &mut addresses);
    return addresses;
}

fn execute(instructions: &Vec<Instruction>) -> u64 {
    let initial_mask = BinaryMask {
        mask: String::from("000000000000000000000000000000000000")
    };
    let mut current_mask = &initial_mask;

    let mut memory: HashMap<u64, u64> = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask)=> current_mask = mask,
            Instruction::Assignment(assignment) => {
                for address in apply_mask(assignment.address, current_mask) {
                    let entry = memory.entry(address).or_default();
                    *entry = assignment.value;
                }
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