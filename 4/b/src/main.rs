use std::env;
use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::path::Path;

use regex::Regex;
use lazy_static::lazy_static;

struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_colour: Option<String>,
    eye_colour: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

fn validate_year(value: &str, min: i32, max: i32) -> Option<String> {
    let number = match value.parse::<i32>() {
        Err(_) => {
            println!("Couldn't parse year {}", value);
            return None
        },
        Ok(number) => number
    };

    if number >= min && number <= max {
        return Some(String::from(value));
    } else {
        println!("Invalid year {} (min {}, max {})", value, min, max);
        return None;
    }
}

fn validate_height(value: &str) -> Option<String> {
    if value.ends_with("in") {
        let number = match value.trim_end_matches("in").parse::<i32>() {
            Err(_) => {
                println!("Couldn't parse height {}", value);
                return None
            },
            Ok(number) => number
        };

        if number >= 59 && number <= 76 {
            return Some(String::from(value));
        } else {
            println!("Height in 'in' out of range {}", value);
            return None;
        }
    } else if value.ends_with("cm") {
        let number = match value.trim_end_matches("cm").parse::<i32>() {
            Err(_) => {
                println!("Couldn't parse height {}", value);
                return None
            },
            Ok(number) => number
        };

        if number >= 150 && number <= 193 {
            return Some(String::from(value));
        } else {
            println!("Height in 'cm' out of range {}", value);
            return None;
        }
    } else {
        println!("Invalid height {}", value);
        return None;
    }
}


lazy_static! {
    static ref HAIR_COLOUR_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
}
fn validate_hair_colour(value: &str) -> Option<String> {
    if HAIR_COLOUR_REGEX.is_match(value) {
        return Some(String::from(value));
    } else {
        println!("Invalid heir colour {}", value);
        return None;
    }
}

fn validate_eye_colour(value: &str) -> Option<String> {
    if value == "amb" || value == "blu" || value == "brn" || value == "gry" || value == "grn" || value == "hzl" || value == "oth"  {
        return Some(String::from(value));
    } else {
        println!("Invalid eye colour {}", value);
        return None;
    }
}

lazy_static! {
    static ref PASSPORT_ID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}
fn validate_passport_id(value: &str) -> Option<String> {
    if PASSPORT_ID_REGEX.is_match(value) {
        return Some(String::from(value));
    } else {
        println!("Invalid passport id {}", value);
        return None;
    }
}

fn parse_passport(lines: &mut std::iter::Peekable<Lines<io::BufReader<File>>>) -> Passport {
    let mut passport =  Passport {
        birth_year: None,
        issue_year: None,
        expiration_year: None,
        height: None,
        hair_colour: None,
        eye_colour: None,
        passport_id: None,
        country_id: None,
    };
    
    let passport_lines: Vec<String> = lines.by_ref().take_while(|line| !line.as_ref().unwrap().is_empty()).collect::<Result<_,_>>().unwrap();
    let passport_line = passport_lines.join(" ");

    for field in passport_line.split(' ') {
        let (field_type, value) = field.split_at(3);
        let real_value = value.trim_start_matches(":");
        match field_type {
            "byr" => passport.birth_year = validate_year(real_value, 1920, 2002),
            "iyr" => passport.issue_year = validate_year(real_value, 2010, 2020),
            "eyr" => passport.expiration_year = validate_year(real_value, 2020, 2030),
            "hgt" => passport.height = validate_height(real_value),
            "hcl" => passport.hair_colour = validate_hair_colour(real_value),
            "ecl" => passport.eye_colour = validate_eye_colour(real_value),
            "pid" => passport.passport_id = validate_passport_id(real_value),
            "cid" => passport.country_id = Some(String::from(real_value)),
            _ => panic!("Unknown field type {}", field_type)
        }
    }

    return passport;
}

fn parse_file(file_name: &Path) -> Vec<Passport> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file,
    };

    let mut passports: Vec<Passport> = Vec::new();

    let mut lines = io::BufReader::new(file).lines().peekable();
    while lines.peek().is_some() {
        passports.push(parse_passport(&mut lines));
    }

    return passports;
}

fn is_valid_passport(passport: &Passport) -> bool {
    return passport.birth_year.is_some()
        && passport.issue_year.is_some()
        && passport.expiration_year.is_some()
        && passport.height.is_some()
        && passport.hair_colour.is_some() & passport.eye_colour.is_some()
        && passport.passport_id.is_some();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let passports = parse_file(&input_file);
    let valid_count = passports.iter().filter(|passport| is_valid_passport(*passport)).count();
    print!("Valid Passports: {}", valid_count);
}
