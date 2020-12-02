use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

struct Policy {
    letter: char,
    first_position: usize,
    second_position: usize,
}

struct Password {
    policy: Policy,
    value: String,
}

fn parse_number(value: &Option<regex::Match>) -> usize {

    let value_string = value.unwrap().as_str();
    return match value_string.parse() {
        Err(why) => panic!("Couldn't parse '{}' into a number: {}", value_string, why),
        Ok(number) => number
    };
}

fn parse_password(line: &str) -> Password {
    let regex: Regex = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>.): (?P<value>.+)$").unwrap();
    return regex.captures(line).map(|captures| Password {
        policy: Policy {
            letter: captures.name("letter").unwrap().as_str().chars().next().unwrap(),
            first_position: parse_number(&captures.name("min")),
            second_position: parse_number(&captures.name("max"))
        },
        value: String::from(captures.name("value").unwrap().as_str()),
    }).unwrap();
}

fn parse_line(line: std::io::Result<String>, file_name: String) -> Password {
    return match line {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name, why),
        Ok(line) => parse_password(line.as_str())
    };
}

fn parse_file(file_name: &Path) -> Vec<Password> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let lines = io::BufReader::new(file).lines();

    return lines.map(|line| parse_line(line, file_name.display().to_string()))
                .collect();
}

fn is_valid_password(password: &Password) -> bool {
    let first_character = password.value.chars().nth(password.policy.first_position - 1).unwrap();
    let second_character = password.value.chars().nth(password.policy.second_position - 1).unwrap();

    return (first_character == password.policy.letter) ^ (second_character == password.policy.letter);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let passwords = parse_file(&input_file);
    let valid_password_count = passwords.iter().filter(|password| is_valid_password(password)).count();
    print!("Valid passwords: {}", valid_password_count);
}