use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;
use lazy_static::lazy_static;

struct Policy {
    letter: char,
    first_position: usize,
    second_position: usize,
}

struct Password {
    policy: Policy,
    value: String,
}

lazy_static! {
    static ref PASSWORD_REGEX: Regex = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>.): (?P<value>.+)$").unwrap();
}

fn parse_number(value: &Option<regex::Match>) -> std::result::Result<usize, Box<dyn std::error::Error>> {
    let value_string = value.ok_or("missing number capture")?.as_str();
    let number: usize = value_string.parse()?;

    return Ok(number);
}

fn parse_character(value: &Option<regex::Match>) -> std::result::Result<char, Box<dyn std::error::Error>> {
    let value_string = value.ok_or("missing character capture")?.as_str();
    let character: char = value_string.chars().next().ok_or("missing character")?;

    return Ok(character);
}

fn parse_string(value: &Option<regex::Match>) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let value_string = value.ok_or("missing string capture")?.as_str();
    return Ok(String::from(value_string));
}

fn parse_password(line: &str) -> std::result::Result<Password, Box<dyn std::error::Error>> {
    let captures = PASSWORD_REGEX.captures(line).ok_or("line is not in correct format")?;
    return Ok(Password  {
        policy: Policy {
            letter: parse_character(&captures.name("letter"))?,
            first_position: parse_number(&captures.name("min"))?,
            second_position: parse_number(&captures.name("max"))?
        },
        value: parse_string(&captures.name("value"))?
    });
}

fn parse_line(line: std::io::Result<String>, file_name: String) -> Password {
    let value = match line {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name, why),
        Ok(line) => parse_password(line.as_str())
    };

    return match value {
        Err(why) => panic!("Couldn't parse password: {}", why),
        Ok(password) => password
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